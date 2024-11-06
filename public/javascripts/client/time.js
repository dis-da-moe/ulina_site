System.register(["../shared/timeUtil.js"], function (exports_1, context_1) {
    "use strict";
    var timeUtil_js_1;
    var __moduleName = context_1 && context_1.id;
    return {
        setters: [
            function (timeUtil_js_1_1) {
                timeUtil_js_1 = timeUtil_js_1_1;
            }
        ],
        execute: function () {
            window.onload = () => {
                const realTime = document.getElementById("real-time");
                const ulinaTime = document.getElementById("ulina-time");
                window.setInterval(() => {
                    const realCurrent = new Date();
                    realTime.textContent = realCurrent.toLocaleDateString();
                    ulinaTime.textContent = timeUtil_js_1.toUlinaTime(realCurrent.valueOf()).toLocaleDateString();
                }, 10);
            };
        }
    };
});
//# sourceMappingURL=time.js.map