window.onload = () => {
    const searchField = document.getElementById("search-field");
    const searchButton = document.getElementById("search-button");
    const clearButton = document.getElementById("clear-button");
    const currentTable = document.getElementById("nations-current").firstChild;
    const oldTable = document.getElementById("nations-old").firstChild;
    function getNationsFromTable(tableBody, nationsExist) {
        if (!nationsExist)
            return [];
        else {
            return Array.from(tableBody.children)
                .slice(1)
                .map(row => { return { name: row.firstElementChild.textContent, row: row }; });
        }
    }
    const nations = getNationsFromTable(currentTable, newNations)
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
            if (!nation.name.toLowerCase().includes(searchValue)) {
                nation.row.style.display = "none";
            }
        });
    });
    if (searchField.value !== "") {
        searchButton.click();
    }
};
//# sourceMappingURL=manageNations.js.map