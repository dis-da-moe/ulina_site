import {Prisma, PrismaClient} from '@prisma/client';
import * as Buffer from "buffer";
import {appDir} from "./app";
import {parse} from 'csv-parse/sync';
import {capitalise, formatDateForFile, readFile, writeFile} from "./utilities";
import {Err, Ok, Result} from "../shared/result";

const path = require("path");
const fs = require('fs');
export const prisma: PrismaClient = new PrismaClient();

export const nationIdsNames : Prisma.NationSelect = {
    nationId: true,
    name: true
}

export const nationIdsNamesContinents : Prisma.NationSelect = {
    nationId: true,
    name: true,
    continent: true
}

export const nationAndSocials : Prisma.NationSelect = {
    nationId: true,
    continent: true,
    name: true,
    removed: true,
    description: true,
    currentFlagId: true,
    socials: true,
    ownerDiscord: true
}

export const currentNations: Prisma.NationWhereInput = {
    removed: false,
}

export const oldNations: Prisma.NationWhereInput = {
    removed: true,
}

export async function validateId(id: undefined|any): Promise<Result<number>>{
    if(id === undefined) return Err("No ID provided");

    const parsed = parseInt(id.toString(), 10);

    if(isNaN(parsed)) return Err("Id not valid number");

    const nation = await nationExists(nationById(parsed));

    if(nation) return Ok(parsed);
    else return Err(`Nation does not exist by id: ${id}`);
}

function continentByName(continentName: string): Prisma.ContinentWhereUniqueInput{
    return {name: continentName};
}

export async function continentExists(continentName: any|undefined){
    if(continentName === undefined || typeof continentName !== "string") return false;
    const continent = await prisma.continent.count({where: {name: continentName}});
    return continent >= 1;
}

export function nationById(id: number) : Prisma.NationWhereUniqueInput{
    return {nationId: id};
}

export interface UpdateNation{
    nationId: number;
    name?: string;
    continent?: string;
    removed?: boolean;
    ownerDiscord?: string;
    flag?: CreateFlag;
    description?: string;
    socials?: CreateSocial[];
}

type ChangeType = "Creation"|"Removed"|"Continent"|"Flag"|"OwnerDiscord"|"Description"|"Name"

async function nationCreationChange(nationId: number){
    const input: Prisma.NationChangeCreateInput = {
        nation: {connect: nationById(nationId)},
        type: "Creation",
        oldValue: "",
        newValue: "",
        timeStamp: new Date(),
        admin: true,
    }
    await prisma.nationChange.create({data: input});
}

function parseValue(value: string|null){
    return value === null ? "NULL" : value.toString();
}

async function nationChange(nationId: number, changeType: ChangeType, oldValue:string|null, newValue:string|null, admin:boolean){
    const info: Prisma.NationChangeCreateInput = {
        nation: {connect: nationById(nationId)},
        type: changeType,
        oldValue: parseValue(oldValue),
        newValue: parseValue(newValue),
        admin: admin,
        timeStamp: new Date()
    }
    await prisma.nationChange.create({data: info});
}


export async function nationExists(where: Prisma.NationWhereInput){
    const count = await prisma.nation.count({where});
    return count >= 1;
}

export function selectNations(where: Prisma.NationWhereInput, select: Prisma.NationSelect){
    return prisma.nation.findMany({
        select: select,
        where: where
    });
}

export async function currentFlag(where: Prisma.NationWhereUniqueInput): Promise<string|undefined>{
    const nation = await prisma.nation.findUnique({where});
    if(nation.currentFlagId == null) return undefined;
    else{
        const flag = await prisma.flag.findUnique({where: {flagId: nation.currentFlagId}});
        return flag.flagPath;
    }
}

export async function updateNation(updateInfo: UpdateNation, admin=true){
    const nation:any = await prisma.nation.findUnique({where: nationById(updateInfo.nationId), select:nationAndSocials});
    let updateInput : Prisma.NationUpdateInput = {};
    const change = async (type: ChangeType, oldValue, newValue) => {
        await nationChange(nation.nationId, type, oldValue, newValue, admin);
    }
    if(updateInfo.flag !== undefined){
        const flag = await createFlag(updateInfo.flag, nation.nationId, nation.name);
        await change("Flag", nation.currentFlagId, flag.flagId);
        updateInput.currentFlagId = flag.flagId;
    }

    if (updateInfo.socials !== undefined){
        const currentSocials: CreateSocial[] = nation.socials;
        const socialsToAdd = updateInfo.socials.filter(social => social.socialsId === undefined);
        const socialsToUpdate = updateInfo.socials.filter(social => {
            return currentSocials.some(currentSocial => {
                return currentSocial.socialsId === social.socialsId &&
                    (currentSocial.link != social.link || currentSocial.platform != social.platform)
            });
        });
        const socialsToRemove = currentSocials.filter(social => {
            return updateInfo.socials.every(currentSocial => currentSocial.socialsId != social.socialsId);
        });
        for(const social of socialsToAdd){
            await prisma.social.create({data:{link:social.link, platform: social.platform, nationId: nation.nationId}});
        }
        for(const social of socialsToRemove){
            await prisma.social.delete({where:{socialsId: social.socialsId}});
        }
        for(const social of socialsToUpdate){
            await prisma.social.update({where:{socialsId: social.socialsId}, data:{link: social.link, platform: social.platform}});
        }
    }

    if(updateInfo.continent !== undefined){
        updateInput.continent = {connect: continentByName(updateInfo.continent)};
        await change("Continent", nation.continent.name, updateInfo.continent);
    }

    type update = [any, any, ChangeType,(val: any) => void];

    async function setDifferences (update: update){
        if(update[0] !== undefined){
            await change(update[2], update[1], update[0]);
            update[3](update[0]);
        }
    }

    const normalUpdates: update[] = [
        [updateInfo.name, nation.name, "Name",(val) => updateInput.name = val],
        [updateInfo.description, nation.description, "Description",(val) => updateInput.description = val],
        [updateInfo.ownerDiscord, nation.ownerDiscord, "OwnerDiscord",(val) => updateInput.ownerDiscord = val],
        [updateInfo.removed, nation.removed, "Removed",(val) => updateInput.removed = val],
    ]
    for(const update of normalUpdates){
        await setDifferences(update);
    }

    return await prisma.nation.update({where: nationById(updateInfo.nationId), data: updateInput});
}

export async function selectChanges(){
    return await prisma.nationChange.findMany();
}

async function createFlag(flagInfo: CreateFlag, nationId: number, nationName: string){

    const dateString = formatDateForFile(nationName, flagInfo.fileExtension);
    const location = `/flags/${dateString}`;
    const filePath = path.join(appDir, `/public/`, location);
    await fs.writeFile(filePath, flagInfo.file,{flag: "w+"},
        (err) => {if(err != null) console.log(err); });

    const flagInput: Prisma.FlagCreateInput = {
        flagPath: location,
        nation: {connect: nationById(nationId)}
    };
    return await prisma.flag.create({data: flagInput});
}

export async function addMap(file: Buffer): Promise<Result<Boolean>>{
    const fileName = formatDateForFile("map", ".svg");
    const filePath = path.join(appDir, `/data/${fileName}`);
    const result = await writeFile(filePath, file);
    if(result.ok){
        await prisma.map.create({data: {
            fileName: fileName,
            date: new Date()
            }});
    }
    return result;
}

export async function getMostRecentMap(): Promise<Result<Buffer>>{
    const maps = await prisma.map.findMany();
    if(maps.length === 0){
        return Err("No maps found");
    }
    else{
        maps.sort((a, b) => b.date.valueOf() - a.date.valueOf());
        const filePath = `/data/${maps[0].fileName}`;
        return await readFile(filePath);
    }
}

export interface CreateNation{
    name: string;
    continent: string;
    removed: boolean;
    ownerDiscord: string;
    flag?: CreateFlag;
    socials?: CreateSocial[];
    description?: string;
}

export interface CreateSocial {
    socialsId?: number;
    link: string;
    platform: string;
}

export interface CreateFlag {
    file: Buffer,
    fileExtension: string
}

interface Continent{
    name:string;
    description:string;
}

export interface Social {
    socialsId: number,
    nationId: number,
    link: string,
    platform: string
}
export type emptyString = ""

export interface NationAndSocials{
    nationId: number,
    continent: Continent,
    name: string,
    removed: boolean,
    description: null | string,
    currentFlagId: null | number,
    socials: Social[],
    ownerDiscord: emptyString | string
}

export async function createNation(nationInfo: CreateNation){
    const nationInput: Prisma.NationCreateInput = {
        continent: {connect: {name: nationInfo.continent}},
        name: nationInfo.name,
        removed: nationInfo.removed,
        description: nationInfo.description,
        ownerDiscord: nationInfo.ownerDiscord,
    }
    const nation = await prisma.nation.create({data: nationInput});
    await nationCreationChange(nation.nationId);
    const connectNation= {connect: nationById(nation.nationId)};

    if (nationInfo.flag){
        const flag = await createFlag(nationInfo.flag, nation.nationId, nation.name);
        await prisma.nation.update({where: nationById(nation.nationId), data:{currentFlagId:flag.flagId}});
    }

    if(nationInfo.socials){
        for (const socialInfo of nationInfo.socials){
            const socialInput: Prisma.SocialCreateInput = {
                nation: connectNation,
                platform: socialInfo.platform,
                link: socialInfo.link
            }
            await prisma.social.create({data: socialInput});
        }
    }
}

export async function selectNation(where: Prisma.NationWhereUniqueInput, select: Prisma.NationSelect){
    return await prisma.nation.findUnique({
        select: select,
        where: where
    });
}

function readCsv<RecordType>(path: string): RecordType[]{
    const fileContent = fs.readFileSync(path);
    return parse(fileContent, {columns:true});
}

export const allContinents = [
    "Ripiero",
    "Kanita",
    "Zapita",
    "Ailou",
    "Sivalat"
]

export interface hasDiscordId {
    ownerDiscord: string
}

export async function nationByName<NationType extends hasDiscordId>(name: string, select: Prisma.NationSelect, includeDead: boolean = false):
    Promise<Result<NationType>>{

    name = name.trim();
    const result = await selectNations({
        name: {
            in: [name, capitalise(name), name.toLowerCase()]
        },
        removed: includeDead
    }, select);

    if(result.length < 1){
        return Err(`nation by the name of "${name}" not found - for the list of nations use \`/all-nations\``);
    }
    const nation = result[0] as unknown as NationType;
    return Ok(nation);
}

interface OldNation{
    Country:string;
    Continent:string;
    Insta:string;
    Wiki:string;
    Description:string;
    ExtraSite:string;
    LinkToExtra:string;
}

async function createContinents(){
    const continentPath = "data/original/continents.csv";
    const continents = readCsv<Continent>(path.join(appDir,continentPath));
    await prisma.continent.deleteMany({where:{name:{endsWith:""}}});
    continents.forEach((async (continent) =>{
        await prisma.continent.create({data: {description: continent.description, name: continent.name}});
    }));
}

async function resetDatabase(){
    const nationsCsv = "/data/original/nations.csv";
    const oldNations = readCsv<OldNation>(path.join(appDir, nationsCsv));
    await prisma.$executeRaw`DELETE FROM Social;`;
    await prisma.$executeRaw`DELETE FROM Flag;`;
    await prisma.$executeRaw`DELETE FROM NationChange;`;
    await prisma.$executeRaw`DELETE FROM Nation;`;
    const nations: CreateNation[] = oldNations.map((oldRecord,) => {
        const socials: CreateSocial[] = [];
        [[oldRecord.Wiki, "Wiki"], [oldRecord.Insta, "Insta"], [oldRecord.LinkToExtra, oldRecord.ExtraSite]]
            .forEach((social) => {
                if  (social[0] !== ''){
                    socials.push({platform: social[1], link: social[0], socialsId:undefined});
                }
            });
        return {
            continent: oldRecord.Continent,
            name: oldRecord.Country,
            description: oldRecord.Description == "" ? undefined : oldRecord.Description,
            removed: false,
            ownerDiscord: "",
            socials: socials
        };
    });

    for(const nation of nations){
        await createNation(nation);
    }
}
