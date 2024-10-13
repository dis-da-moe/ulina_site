interface TimePeriod{
    realTime: number,
    ulinaTime: number,
    timeDifference: number
}

const oldTime: TimePeriod = {
    realTime:  Date.UTC(2019, 5),
    ulinaTime: Date.UTC(2026, 0),
    timeDifference: 12
}

const newTime: TimePeriod = {
    realTime: Date.UTC(2021, 0),
    ulinaTime: Date.UTC(2045, 0),
    timeDifference: 4
}

function calculateUlina(timePeriod: TimePeriod, realTime: number) : Date{
    const realTimePassed = realTime - timePeriod.realTime;
    const ulinaTimePassed = realTimePassed * timePeriod.timeDifference;
    return new Date(ulinaTimePassed + timePeriod.ulinaTime);
}

export function isValidDate(year: number, month: number, day: number) : boolean{
    if (day < 0) return false;
    if (month < 1 || month > 12) return false;
    if (day < 1 || day > 31) return false;

    if (month == 2){
        if (((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)) return (day <= 29);
        else return (day <= 28);
    }
    if ([4, 6, 9, 11].includes(month)){
        return day <= 30;
    }
    else{
        return true;
    }
}

export function realTimeInRange(realTime: Date): boolean {
    return realTime.valueOf() >= oldTime.realTime;
}

export function toUlinaTime(realTime: number): Date{
    const timePeriod = realTime >= newTime.realTime ? newTime : oldTime;
    return calculateUlina(timePeriod, realTime);
}

export function ulinaTimeNow(): Date{
    const realCurrent = new Date();
    return calculateUlina(newTime,realCurrent.valueOf());
}