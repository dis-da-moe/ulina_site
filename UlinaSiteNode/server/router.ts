import * as database from "./database";
import {Request, Response} from 'express-serve-static-core';

import {DOMParser} from '@xmldom/xmldom';
import {appDir} from "./app";
import {acceptedExtensions, matchesImageExtension} from "./utilities";
// @ts-ignore
const Promise = require("bluebird");

const fs = require("fs");
import express = require('express');
import path = require("path");
import {readFileSync} from "fs";
import {lookup} from "mime-types";

const router = express.Router();
const xmlParser = new DOMParser();

const multer = require('multer');

let xmlDocument: Document = undefined;
setXml();

function setXml() {
    const mapBuffer = fs.readFileSync("data/map.svg");
    xmlDocument = xmlParser.parseFromString(mapBuffer.toString(), "image/svg+xml");
}

async function renderCreator(res: Response, message = undefined) {
    const results = await database.selectNations(database.currentNations, database.nationIdsNames);
    res.render('creator', {xml: xmlDocument, data: results, message: message});
}


function renderError(res: Response, message: string){
    res.render("error", {message: message});
}

function renderAdmin(res: Response) {
    res.render('admin');
}

function renderSignin(res: Response, message) {
    res.render("signin", {message: message});
}

function renderAddNation(res: Response, message=undefined){
    res.render("add-nation", {message});
}

type NationsActions = "manage-admin" | "view-nation";
async function renderNations(req: Request, res: Response, action: NationsActions, message=undefined) {
    const newNations = await database.selectNations(database.currentNations, database.nationIdsNamesContinents);
    const oldNations = await database.selectNations(database.oldNations, database.nationIdsNamesContinents);
    res.render('view-nations', {newNations, oldNations, action, message, admin: req.session.admin});
}

async function renderNation(req: Request, res: Response, message=undefined) {
    if(!req.query || !req.query.id){
        await renderNations(req, res, "view-nation","ID not provided");
        return;
    }

    const idResult = await database.validateId(req.query.id);

    if(idResult.err){
        renderNations( req, res, "view-nation",idResult.message);
        return;
    }

    const id = idResult.value;

    const nation: any = await database.selectNation(database.nationById(id), database.nationAndSocials);
    const currentFlag = await database.currentFlag(database.nationById(id));
    res.render('view-nation', {
        nation:nation,
        admin: req.session.admin === true,
        manage: req.session.signedInId === id || req.session.admin === true,
        message:message,
        acceptedExtensions: acceptedExtensions,
        currentFlag: currentFlag});
}

async function renderViewMap(res: Response){
    const nationsDatabase = (await database.selectNations(database.currentNations, database.nationAndSocials));
    const nations = await Promise.map(nationsDatabase, async (nation:any) => {
        return {
            nationId: nation.nationId,
            continent: nation.continent.name,
            description: nation.description === null ? undefined : nation.description,
            name: nation.name,
            socials: nation.socials,
            currentFlagPath: await database.currentFlag(database.nationById(nation.nationId))
        };
    });

    const map = await database.getMostRecentMap();
    if(map.ok){
        try{
            const parsed = xmlParser.parseFromString(map.value.toString(), "image/svg+xml");
            res.render("map-view", {xml:parsed, nations:nations})
        }
        catch (e){
            console.log(e);
            renderError(res,"Map file could not be parsed.")
        }
    }
    else{
        renderError(res,map.message);
    }
}

async function renderChanges(res: Response){
    let result = await database.selectChanges();
    result = result.sort((a, b) => {
        return b.timeStamp.valueOf() - a.timeStamp.valueOf();
    });
    const changes = []
    for(const change of result){
        changes.push(
         {
            type: change.type,
            oldValue: change.oldValue,
            newValue: change.newValue,
            timeStamp: change.timeStamp,
            admin: change.admin,
            nation: await database.selectNation(database.nationById(change.nationId), database.nationIdsNames),
        }
        );
    }
    res.render("nation-changes", {changes: changes});
}

const privateRoutes: [string, ((req: Request, res: Response) => void)][] =
    [
        ["/admin", (_, res) => renderAdmin(res)],
        ["/creator", (_, res) => renderCreator(res)],
        ["/manage-nations", (req, res) => renderNations(req, res, "manage-admin")],
        ["/manage-admin", renderNation],
        ["/add-nation", (_, res) => renderAddNation(res)],
        ["/nation-changes", (_, res) => renderChanges(res)]
    ]

privateRoutes.forEach(route => {
    const path = route[0];
    const action = route[1];
    router.get(path, (req, res) => {
        if(req.session.admin){
            action(req, res);
        }
        else{
            renderSignin(res, "Enter Password");
        }
    });
});

declare module 'express-session' {
    interface SessionData {
        admin: boolean;
        signedInId: number | undefined;
    }
}

router.post("/signin", (req, res) => {
    if(req.body.password !== process.env.ADMIN){
        renderSignin(res, "Incorrect password");
    }
    else{
        req.session.admin = true;

        renderAdmin(res);
    }
});

router.post("/submitmap", async (req, res) => {
    if(!req.session.admin){
        renderSignin(res, "Invalid session");
        return;
    }
    else if(!req.body.map){
        renderCreator(res, "No map provided");
        return;
    }

    try{
        const map = xmlParser.parseFromString(req.body.map, "image/svg+xml");

        const result = await database.addMap(req.body.map);
        if(result.ok){
            console.log(`Wrote map to file`);
            renderCreator(res, "Map saved");
        }
        else{
            console.log(`An error occurred: ${result.message}`);
            renderCreator(res, "Map not saved");
        }
        return;
    }
    catch (e){
        console.log(e);
        renderCreator(res, "Map is invalid");
        return;
    }


});

function parseRemoved(value: undefined|string, original:boolean): undefined | boolean{
    let parsed: boolean|undefined;
    switch (value){
        case undefined:
            parsed = false;
            break;
        case "true":
            parsed = true;
            break;
        default:
            parsed = undefined;
    }
    if(parsed === original || parsed === undefined) return undefined;
    else return parsed;
}

router.post("/add-nation", async (req, res) => {
    if(!req.session.admin){
        renderSignin(res, "Invalid credentials");
        return;
    }
    if(!(await database.continentExists(req.body.continent))){
        renderAddNation(res, `Continent "${req.body.continent}" doesn't exist`);
        return;
    }
    if(!req.body.ownerDiscord || !req.body.name){
        renderAddNation(res, `Missing required fields`);
        return;
    }
    const info: database.CreateNation = {
        name: req.body.name,
        continent: req.body.continent,
        removed: req.body.removed === "on",
        ownerDiscord: req.body.ownerDiscord
    }
    await database.createNation(info);
    renderNations(req, res,`manage-admin`, `nation ${info.name} created successfully`);
})

const imageHandler =
    multer({storage:multer.memoryStorage()}).single("currentFlag");

router.post("/manage-nation",imageHandler,async (req, res) => {
    const idResult = await database.validateId(req.body.nationId);
    if(idResult.err){
        renderNations(req, res, "manage-admin", idResult.message);
        return;
    }
    const id = idResult.value;
    const admin: boolean = req.session.admin === undefined ? false : req.session.admin;

    if(!admin){
        renderSignin(res, "Invalid Credentials");
        return;
    }
    let flag: database.CreateFlag = undefined;
    if(req.file){
        const extensionResult = matchesImageExtension(req.file.originalname);
        if(extensionResult.err){
            renderNations(req, res, "manage-admin",extensionResult.message);
            return;
        }
        flag = {file: req.file.buffer, fileExtension: extensionResult.value};
    }

    const nation: any = await database.selectNation(database.nationById(id), database.nationAndSocials);


    function parseName(name: undefined|string, original: string){
        if(name === undefined) return undefined;
        else if(name === "") return undefined;
        else if(name === original) return undefined;
        else return name;
    }

    function parseContinent(continent: undefined|string, original:string){
        if(continent === undefined) return undefined;
        else if(continent === original) return undefined;
        else if(!database.continentExists(continent)) return undefined;
        else return continent;
    }

    function parseSocials(socials: string|undefined): database.CreateSocial[]|undefined{
        if(socials === undefined) return undefined;
        try{
            const parsed = JSON.parse(socials) as database.CreateSocial[];
            const valid = parsed.every(social =>
                social.link != undefined && typeof social.link == "string"
                && social.platform != undefined && typeof social.platform == "string"
                && social.socialsId == undefined || typeof social.socialsId == "number");
            if(!valid) return undefined;
            else return parsed;
        }
        catch (_){
            return undefined;
        }
    }

    function parseDescription(description: string|undefined, original: string|null){
        if(description === undefined) return undefined;
        else if(description === ""){
            if(original === null) return undefined;
            else return null;
        }
        else if(description === original) return undefined;
        else return description.toString();
    }

    function undefinedIfSame(value: undefined|string, original: string){
        if(value === undefined || value === original){
            return undefined;
        }
        else{
            return value;
        }
    }

    const updateInfo : database.UpdateNation= {
        nationId: id,
        removed: admin ? parseRemoved(req.body.removed, nation.removed) : undefined,
        ownerDiscord: admin ? undefinedIfSame(req.body.ownerDiscord, nation.ownerDiscord) : undefined,
        continent: admin ? parseContinent(req.body.continent, nation.continent.name) : undefined,
        name: parseName(req.body.name, nation.name),
        flag,
        description: parseDescription(req.body.description, nation.description),
        socials: parseSocials(req.body.socials)
    }
    const result = await database.updateNation(updateInfo);
    renderNations(req, res, "manage-admin",`Saved info of ${result.name}`);
});

const postToGetRedirects =  ["/signin", "/submitmap", "/manage-nation"];

postToGetRedirects.forEach(path => {
    router.get(path, (req, res) => {
        res.redirect("/admin");
    });
});

const publicRoutes: [string, ((req: Request, res: Response) => void)][] = [
    ["/tools", (_, res) => res.render("tools")],
    ["/view-nations", (req, res) => renderNations(req, res, "view-nation")],
    ["/view-map", (_, res) => renderViewMap(res)],
    ["/view-nation", (req, res) => renderNation(req, res)],
    ["/time", (_, res) => res.render("time")]
]

publicRoutes.forEach(route => {
    router.get(route[0], route[1]);
});

const staticRoutes: [string, string][] = [
    ["/", "index.html"],
    ["/index", "index.html"],
    ["/join", "join.html"],
    ["/about", "about.html"]
]

staticRoutes.forEach(route => {
    router.get(route[0], (req, res) => {
        res.sendFile(`${appDir}/server/static/${route[1]}`);
    });
});

module.exports = router