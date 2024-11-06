System.register([], function (exports_1, context_1) {
    "use strict";
    var oldTime, newTime;
    var __moduleName = context_1 && context_1.id;
    function calculateUlina(timePeriod, realTime) {
        const realTimePassed = realTime - timePeriod.realTime;
        const ulinaTimePassed = realTimePassed * timePeriod.timeDifference;
        return new Date(ulinaTimePassed + timePeriod.ulinaTime);
    }
    function isValidDate(year, month, day) {
        if (day < 0)
            return false;
        if (month < 1 || month > 12)
            return false;
        if (day < 1 || day > 31)
            return false;
        if (month == 2) {
            if (((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0))
                return (day <= 29);
            else
                return (day <= 28);
        }
        if ([4, 6, 9, 11].includes(month)) {
            return day <= 30;
        }
        else {
            return true;
        }
    }
    exports_1("isValidDate", isValidDate);
    function realTimeInRange(realTime) {
        return realTime.valueOf() >= oldTime.realTime;
    }
    exports_1("realTimeInRange", realTimeInRange);
    function toUlinaTime(realTime) {
        const timePeriod = realTime >= newTime.realTime ? newTime : oldTime;
        return calculateUlina(timePeriod, realTime);
    }
    exports_1("toUlinaTime", toUlinaTime);
    function ulinaTimeNow() {
        const realCurrent = new Date();
        return calculateUlina(newTime, realCurrent.valueOf());
    }
    exports_1("ulinaTimeNow", ulinaTimeNow);
    return {
        setters: [],
        execute: function () {
            oldTime = {
                realTime: Date.UTC(2019, 5),
                ulinaTime: Date.UTC(2026, 0),
                timeDifference: 12
            };
            newTime = {
                realTime: Date.UTC(2021, 0),
                ulinaTime: Date.UTC(2045, 0),
                timeDifference: 4
            };
        }
    };
});
//# sourceMappingURL=timeUtil.js.map