const xmlns = "http://www.w3.org/2000/svg";

interface NationIdName {
    nationId:number,
    name: string
}

// @ts-ignore
declare const nations: NationIdName[];

function matchData(filterFunction, returnFunction){
    const matches = nations.filter(filterFunction);
    if(matches.length == 0){
        return undefined;
    }
    else{
        return returnFunction(matches[0]);
    }
}

function nameFromId(id: string) : (string|undefined){
    const intId = parseInt(id, 10);
    if(isNaN(intId)) return  undefined;
    return matchData((nation => nation.nationId == id), (nation => nation.name));
}

function idFromName(name: string): (number|undefined){
    return matchData((nation => nation.name == name), (nation => nation.nationId));
}

window.onload = async () => {
    const svg = document.getElementsByTagName("svg").item(0);
    const nationsParent = document.getElementById("NATIONS") as HTMLElement;
    Array.from(nationsParent.children)
        .forEach(child => {
            if(child.tagName === "path"){
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
    const nameField = document.getElementById("name") as HTMLInputElement;
    const mapField = document.getElementById("map-field") as HTMLInputElement;
    const form = document.getElementById("submit-form") as HTMLFormElement;
    const selectedNationText = document.getElementById("nation-name") as HTMLParagraphElement;
    const nameStatus = document.getElementById("name-status") as HTMLParagraphElement;
    const submitStatus = document.getElementById("submit-status") as HTMLParagraphElement;

    let selectedGroup: SVGGElement = undefined;

    nationGroups.forEach(nation => {
        nation.addEventListener("click", () => {
            selectedGroup = nation;
            const name = nameFromId(selectedGroup.id);
            if(name !== undefined){
                selectedNationText.textContent = name;
            }
            else{
                selectedNationText.textContent = "unassigned";
            }
        });
    });

    enterNameButton.onclick = () => {
        if(selectedGroup == undefined){
            nameStatus.textContent = "No Nation Selected";
            return;
        }
        if(nameField.value == "") {
            nameStatus.textContent = "No name entered";
            return;
        }
        const id = idFromName(nameField.value);
        if(id == undefined){
            nameStatus.textContent = "Can not find name";
            return;
        }
        selectedGroup.id = id.toString();
        Array.from(selectedGroup.children)
            .map(region => region as HTMLElement)
            .forEach((region) => {
                region.style.removeProperty("fill");
            });
        nameStatus.textContent = `Saved name ${nameField.value}`;
    };

    submitButton.onclick = async () => {

        const unassigned = nationGroups.filter(nation => {
            return nameFromId(nation.id) === undefined;
        });

        if(unassigned.length > 0){
            unassigned.forEach(nation => Array.from(nation.children)
                .map(region => region as HTMLElement)
                .forEach((region) => {
                    region.style.fill = "blue";
                }));

            submitStatus.textContent = `Blue regions are not assigned a nation`;
            return;
        }

        const unassignedNames: string[] = nations.reduce((previous, current) =>
        {
            const search = document.getElementById(current.nationId.toString());
            if(search === null){
                previous.push(current.name);
            }
            return previous;
        }, [])

        if(unassignedNames.length > 0){
            submitStatus.textContent = `Following nations are unassigned: ${unassignedNames.join(", ")}`;
            return;
        }

        const serializer = new XMLSerializer();
        mapField.value = serializer.serializeToString(svg);
        form.submit();

    };
};