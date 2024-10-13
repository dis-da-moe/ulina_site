import {toUlinaTime} from "../shared/timeUtil.js";

window.onload = () => {
    const realTime = document.getElementById("real-time");
    const ulinaTime = document.getElementById("ulina-time");

    window.setInterval(() => {
        const realCurrent = new Date();
        realTime.textContent = realCurrent.toLocaleDateString();
        ulinaTime.textContent = toUlinaTime(realCurrent.valueOf()).toLocaleDateString();
    }, 10);
}