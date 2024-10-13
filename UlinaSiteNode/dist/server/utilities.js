"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.matchesImageExtension = exports.acceptedExtensions = exports.capitalise = exports.readFile = exports.writeFile = exports.formatDateForFile = void 0;
const path = require("path");
const app_1 = require("./app");
const result_1 = require("../shared/result");
const fs = require('fs');
const util = require('util');
const writePromise = util.promisify(fs.writeFile);
const readPromise = util.promisify(fs.readFile);
function formatDateForFile(fileName, extension) {
    const now = new Date();
    const date = `${now.toDateString()}-${now.getUTCHours()}-${now.getUTCMinutes()}-${now.getUTCSeconds()}`
        .replace(/\s/g, '_');
    return `${fileName.replace(/\s/g, '_')}-${date}${extension}`;
}
exports.formatDateForFile = formatDateForFile;
async function writeFile(filePath, file) {
    const err = await writePromise(filePath, file, { flag: "w+" }).catch(err => { return err; });
    if (err) {
        return (0, result_1.Err)(err);
    }
    else {
        return (0, result_1.Ok)(true);
    }
}
exports.writeFile = writeFile;
async function readFile(pathFromRoot) {
    return readPromise(path.join(app_1.appDir, pathFromRoot))
        .then(data => {
        return (0, result_1.Ok)(data);
    }).catch(err => {
        console.log(err);
        return (0, result_1.Err)(`Could not read saved map: ${pathFromRoot}`);
    });
}
exports.readFile = readFile;
function capitalise(word) {
    return word.split(" ").map(word => {
        let formatted = word.charAt(0).toUpperCase();
        if (word.length > 1) {
            formatted += word.slice(1).toLowerCase();
        }
        return formatted;
    }).join(" ");
}
exports.capitalise = capitalise;
exports.acceptedExtensions = "jpg|JPG|jpeg|JPEG|png|PNG";
const extensionsArray = exports.acceptedExtensions.split("|").map(extension => `.${extension}`);
function matchesImageExtension(fileName) {
    let matchedExtension;
    const valid = extensionsArray.some(extension => {
        const expected = fileName.slice(fileName.length - extension.length);
        if (expected == extension) {
            matchedExtension = extension;
            return true;
        }
        else
            return false;
    });
    if (valid === false)
        return (0, result_1.Err)("No image extension");
    return (0, result_1.Ok)(matchedExtension);
}
exports.matchesImageExtension = matchesImageExtension;
//# sourceMappingURL=utilities.js.map