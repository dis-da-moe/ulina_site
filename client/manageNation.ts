declare const socials: Social[];
declare const manage: boolean;
interface Social{
    socialsId: number|undefined,
    platform: string,
    link: string,
}

const emptySocial: Social = {link: "", platform: "", socialsId: undefined};

function addField<ElementType extends HTMLElement>(element: ElementType, row: HTMLDivElement){
    row.appendChild(element);
    element.className = "col-auto";
    return element;
}

function createInput(value: string, row: HTMLDivElement, type = undefined){
    const input = document.createElement("input");
    input.value = value;
    addField(input, row);
    if(type !== undefined) input.type = type;
}

function createParagraph(value: string, row: HTMLDivElement){
    const paragraph = document.createElement("p");
    paragraph.textContent = value;
    return addField(paragraph, row);
}

function createSocial(social: Social, socialsTable, isManaging: boolean){
    const row = document.createElement("div");
    row.className = "row";

    const createField = isManaging ? createInput : createParagraph;

    createInput(social.socialsId === undefined ? "" : social.socialsId.toString(), row, "hidden");
    createField(social.platform, row);
    createField(social.link, row);

    if(isManaging){
        const deleteButton = document.createElement("button");
        deleteButton.addEventListener("click", () => {
            socialsTable.removeChild(row);
        });
        deleteButton.type = "button";
        deleteButton.textContent = "Delete";
        deleteButton.className = "col-auto";
        row.appendChild(deleteButton);
    }

    socialsTable.appendChild(row);
}

window.onload = () => {
    const socialsTable = document.getElementById("socials-table") as HTMLTableElement;
    socials.forEach((social) => createSocial(social, socialsTable, manage));

    if(!manage) return;

    const socialsField = document.getElementById("socials-field") as HTMLInputElement;
    const addSocial = document.getElementById("add-social") as HTMLButtonElement;
    const form = document.getElementById("form") as HTMLFormElement;
    const submitButton = document.getElementById("submit-button") as HTMLButtonElement;
    const removedCheckbox = document.getElementById("nation-removed") as HTMLInputElement;

    addSocial.addEventListener("click", () => createSocial(emptySocial, socialsTable, manage));

    submitButton.addEventListener("click", () => {
        const newSocials: Social[] = Array
            .from(socialsTable.children)
            .slice(1)
            .map((row => {
                const values = Array.from(row.children)
                    .slice(0,3)
                    .map(child => (child as HTMLInputElement).value);
                const id = values[0] === "" ? undefined : parseInt(values[0]);
                return {socialsId: id, platform: values[1], link: values[2]};}))
            .filter(social => social.link !== "" && social.platform != "");
        removedCheckbox.value = removedCheckbox.checked === true ? "true" : "false";
        socialsField.value = JSON.stringify(newSocials);
        form.submit();
    });
}

