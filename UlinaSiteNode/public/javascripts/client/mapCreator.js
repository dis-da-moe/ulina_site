var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
const xmlns = "http://www.w3.org/2000/svg";
function matchData(filterFunction, returnFunction) {
    const matches = nations.filter(filterFunction);
    if (matches.length == 0) {
        return undefined;
    }
    else {
        return returnFunction(matches[0]);
    }
}
function nameFromId(id) {
    const intId = parseInt(id, 10);
    if (isNaN(intId))
        return undefined;
    return matchData((nation => nation.nationId == id), (nation => nation.name));
}
function idFromName(name) {
    return matchData((nation => nation.name == name), (nation => nation.nationId));
}
window.onload = () => __awaiter(this, void 0, void 0, function* () {
    const svg = document.getElementsByTagName("svg").item(0);
    const nationsParent = document.getElementById("NATIONS");
    Array.from(nationsParent.children)
        .forEach(child => {
        if (child.tagName === "path") {
            const group = document.createElementNS(xmlns, "g");
            group.appendChild(child);
            nationsParent.appendChild(group);
        }
    });
    const nationGroups = Array
        .from(nationsParent.getElementsByTagName("g"))
        .filter(nation => nation.id !== "claims");
    const submitButton = document.getElementById("submit");
    const enterNameButton = document.getElementById("enter-name");
    const nameField = document.getElementById("name");
    const mapField = document.getElementById("map-field");
    const form = document.getElementById("submit-form");
    const selectedNationText = document.getElementById("nation-name");
    const nameStatus = document.getElementById("name-status");
    const submitStatus = document.getElementById("submit-status");
    let selectedGroup = undefined;
    nationGroups.forEach(nation => {
        nation.addEventListener("click", () => {
            selectedGroup = nation;
            const name = nameFromId(selectedGroup.id);
            if (name !== undefined) {
                selectedNationText.textContent = name;
            }
            else {
                selectedNationText.textContent = "unassigned";
            }
        });
    });
    enterNameButton.onclick = () => {
        if (selectedGroup == undefined) {
            nameStatus.textContent = "No Nation Selected";
            return;
        }
        if (nameField.value == "") {
            nameStatus.textContent = "No name entered";
            return;
        }
        const id = idFromName(nameField.value);
        if (id == undefined) {
            nameStatus.textContent = "Can not find name";
            return;
        }
        selectedGroup.id = id.toString();
        Array.from(selectedGroup.children)
            .map(region => region)
            .forEach((region) => {
            region.style.removeProperty("fill");
        });
        nameStatus.textContent = `Saved name ${nameField.value}`;
    };
    submitButton.onclick = () => __awaiter(this, void 0, void 0, function* () {
        const unassigned = nationGroups.filter(nation => {
            return nameFromId(nation.id) === undefined;
        });
        if (unassigned.length > 0) {
            unassigned.forEach(nation => Array.from(nation.children)
                .map(region => region)
                .forEach((region) => {
                region.style.fill = "blue";
            }));
            submitStatus.textContent = `Blue regions are not assigned a nation`;
            return;
        }
        const unassignedNames = nations.reduce((previous, current) => {
            const search = document.getElementById(current.nationId.toString());
            if (search === null) {
                previous.push(current.name);
            }
            return previous;
        }, []);
        if (unassignedNames.length > 0) {
            submitStatus.textContent = `Following nations are unassigned: ${unassignedNames.join(", ")}`;
            return;
        }
        const serializer = new XMLSerializer();
        mapField.value = serializer.serializeToString(svg);
        form.submit();
    });
});
//# sourceMappingURL=mapCreator.js.map