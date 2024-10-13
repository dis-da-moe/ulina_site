const emptySocial = { link: "", platform: "", socialsId: undefined };
function addField(element, row) {
    row.appendChild(element);
    element.className = "col-auto";
    return element;
}
function createInput(value, row, type = undefined) {
    const input = document.createElement("input");
    input.value = value;
    addField(input, row);
    if (type !== undefined)
        input.type = type;
}
function createParagraph(value, row) {
    const paragraph = document.createElement("p");
    paragraph.textContent = value;
    return addField(paragraph, row);
}
function createSocial(social, socialsTable, isManaging) {
    const row = document.createElement("div");
    row.className = "row";
    const createField = isManaging ? createInput : createParagraph;
    createInput(social.socialsId === undefined ? "" : social.socialsId.toString(), row, "hidden");
    createField(social.platform, row);
    createField(social.link, row);
    if (isManaging) {
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
    const socialsTable = document.getElementById("socials-table");
    socials.forEach((social) => createSocial(social, socialsTable, manage));
    if (!manage)
        return;
    const socialsField = document.getElementById("socials-field");
    const addSocial = document.getElementById("add-social");
    const form = document.getElementById("form");
    const submitButton = document.getElementById("submit-button");
    const removedCheckbox = document.getElementById("nation-removed");
    addSocial.addEventListener("click", () => createSocial(emptySocial, socialsTable, manage));
    submitButton.addEventListener("click", () => {
        const newSocials = Array
            .from(socialsTable.children)
            .slice(1)
            .map((row => {
            const values = Array.from(row.children)
                .slice(0, 3)
                .map(child => child.value);
            const id = values[0] === "" ? undefined : parseInt(values[0]);
            return { socialsId: id, platform: values[1], link: values[2] };
        }))
            .filter(social => social.link !== "" && social.platform != "");
        removedCheckbox.value = removedCheckbox.checked === true ? "true" : "false";
        socialsField.value = JSON.stringify(newSocials);
        form.submit();
    });
};
//# sourceMappingURL=manageNation.js.map