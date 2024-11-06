declare const oldNations: boolean;
declare const newNations: boolean;

interface NationRow {
    name: string,
    row: HTMLTableRowElement
}

window.onload = () => {
    const searchField = document.getElementById("search-field") as HTMLInputElement;
    const searchButton = document.getElementById("search-button") as HTMLButtonElement;
    const clearButton = document.getElementById("clear-button") as HTMLButtonElement;
    const currentTable = document.getElementById("nations-current").firstChild as HTMLElement;
    const oldTable = document.getElementById("nations-old").firstChild as HTMLElement;

    function getNationsFromTable(tableBody: HTMLElement, nationsExist: boolean): NationRow[]{
        if(!nationsExist) return [];
        else{
            return Array.from(tableBody.children)
                .slice(1)
                .map(row => {return {name: row.firstElementChild.textContent, row:row as HTMLTableRowElement}});
        }
    }

    const nations: NationRow[] = getNationsFromTable(currentTable, newNations)
        .concat(getNationsFromTable(oldTable, oldNations));

    const restoreTable = () => nations.forEach(nation => nation.row.style.removeProperty("display"));

    clearButton.addEventListener("click", () => {
        restoreTable();
        searchField.value = "";
    });

    searchButton.addEventListener("click", () => {
        restoreTable();
        const searchValue = searchField.value.toLowerCase();
        nations.forEach(nation => {
            if(!nation.name.toLowerCase().includes(searchValue)){
                nation.row.style.display = "none";
            }
        });
    });

    if(searchField.value !== ""){
        searchButton.click();
    }
}