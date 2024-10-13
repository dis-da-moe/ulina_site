import * as path from "path";
import * as Buffer from "buffer";
import {appDir} from "./app";
import {Err, Ok, Result} from "../shared/result";

const fs = require('fs');
const util = require('util');
const writePromise = util.promisify(fs.writeFile);
const readPromise = util.promisify(fs.readFile);

export function formatDateForFile(fileName: string, extension: string){
    const now = new Date();
    const date = `${now.toDateString()}-${now.getUTCHours()}-${now.getUTCMinutes()}-${now.getUTCSeconds()}`
        .replace(/\s/g, '_');
    return `${fileName.replace(/\s/g, '_')}-${date}${extension}`;
}

export async function writeFile(filePath: string, file: Buffer): Promise<Result<boolean>>{
    const err = await writePromise(filePath, file,{flag: "w+"}).catch(err => {return err});
    if(err){
        return Err(err);
    }
    else{
        return Ok(true);
    }
}

export async function readFile(pathFromRoot: string): Promise<Result<Buffer>>{
    return readPromise(path.join(appDir, pathFromRoot))
        .then(data => {
            return Ok(data);
        }).catch(err => {
            console.log(err);
            return Err(`Could not read saved map: ${pathFromRoot}`)
        });
}

export function capitalise(word: string): string{
    return word.split(" ").map(word => {
        let formatted = word.charAt(0).toUpperCase();
        if(word.length > 1){
            formatted += word.slice(1).toLowerCase();
        }
        return formatted;
    }).join(" ");
}

export const acceptedExtensions = "jpg|JPG|jpeg|JPEG|png|PNG";
const extensionsArray = acceptedExtensions.split("|").map(extension => `.${extension}`);

export function matchesImageExtension(fileName: string): Result<string> {
    let matchedExtension;
    const valid = extensionsArray.some(extension => {
        const expected = fileName.slice(fileName.length - extension.length);
        if (expected == extension) {
            matchedExtension = extension;
            return true;
        } else return false;
    });
    if (valid === false) return Err("No image extension");
    return Ok(matchedExtension);
}