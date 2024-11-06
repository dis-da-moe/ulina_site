function coordsFromRect(rect) {
    return { x: rect.x, y: rect.y, height: rect.height, width: rect.width };
}
window.onload = () => {
    document.body.style.overflow = "hidden";
    const xml = document.getElementsByTagName("svg").item(0);
    xml.style.height = "100vh";
    xml.style.width = "100vw";
    const originalCoords = coordsFromRect(xml.viewBox.baseVal);
    let scale = 1;
    const popover = document.getElementById("popover");
    const infobox = document.getElementById("infobox");
    const closeInfobox = document.getElementById("close-infobox");
    const infoName = document.getElementById("name");
    const description = document.getElementById("description");
    const continent = document.getElementById("continent");
    const socialsStack = document.getElementById("socials");
    closeInfobox.onclick = () => { infobox.style.display = "none"; };
    closeInfobox.click();
    let mouseHeld = false;
    document.onmousedown = (ev) => {
        if (ev.button == 0) {
            mouseHeld = true;
        }
    };
    document.onmouseup = (ev) => {
        if (ev.button == 0) {
            mouseHeld = false;
        }
    };
    document.onmousemove = (ev) => {
        const rect = popover.getBoundingClientRect();
        popover.style.left = (ev.x - rect.width).toString() + "px";
        popover.style.top = (ev.y - rect.height / 2).toString() + "px";
        if (mouseHeld) {
            xml.viewBox.baseVal.x -= ev.movementX * 2 * scale;
            xml.viewBox.baseVal.y -= ev.movementY * 2 * scale;
        }
    };
    document.onwheel = (ev) => {
        scale += ev.deltaY * 0.001;
        scale = Math.min(Math.max(0.2, scale), 1);
        xml.viewBox.baseVal.height = originalCoords.height * scale;
        xml.viewBox.baseVal.width = originalCoords.width * scale;
    };
    nations.forEach((nation) => {
        const element = document.getElementById(nation.nationId.toString());
        let inside = false;
        element.onclick = () => {
            infobox.style.removeProperty("display");
            infoName.textContent = nation.name;
            description.textContent = nation.description === undefined ? "" : nation.description;
            continent.textContent = nation.continent;
            Array
                .from(socialsStack.children)
                .forEach(child => socialsStack.removeChild(child));
            if (nation.socials.length === 0) {
                socialsStack.textContent = "No Socials";
            }
            else {
                nation.socials.forEach(social => {
                    const link = document.createElement("a");
                    link.textContent = social.platform;
                    link.href = social.link;
                    const row = document.createElement("div");
                    row.appendChild(link);
                    socialsStack.appendChild(row);
                });
            }
        };
        element.onmouseover = () => {
            inside = true;
            popover.textContent = nation.name;
        };
        element.onmouseout = () => {
            inside = false;
            setInterval(() => {
                if (popover.textContent === nation.name && !inside)
                    popover.textContent = "";
            }, 100);
        };
    });
};
//# sourceMappingURL=mapViewer.js.map