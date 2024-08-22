class Day {
    static A = "A";
    static B = "B";
    static C = "C";
    static D = "D";
    static I = "I";
}

class Semester {
    static S1 = "S1";
    static S2 = "S2";
}

class Class {
    constructor(days, mods, semester, shortName, longName, teacher, room, start, end) {
        this.days = days;
        this.mods = mods;
        this.semester = semester;
        this.shortName = shortName;
        this.longName = longName;
        this.teacher = teacher;
        this.room = room;
        this.start = start;
        this.end = end;
    }
}

function getSchedule(input) {
    let lines = input.split("\n").map(line => line.trim()).filter(line => line.length > 0);

    // Skip the header lines
    if (lines.length <= 2) {
        throw new Error("Not enough data lines");
    }

    lines = lines.slice(2);

    let mods = [];
    let semester = [];
    let shortName = [];
    let longName = [];
    let teacher = [];
    let room = [];
    let start = [];
    let end = [];

    for (let line of lines) {
        // Split by tab characters; fallback to split by multiple spaces if tabs are missing
        let split = line.split(/\s{2,}|\t/).map(s => s.trim());

        if (split.length < 8) {
            throw new Error(`Unexpected format: ${line}`);
        }
        mods.push(split[0]);
        semester.push(split[1]);
        shortName.push(split[2]);
        longName.push(split[3]);
        teacher.push(split[4]);
        room.push(split[5]);
        start.push(split[6]);
        end.push(split[7]);
    }

    return sortByDay(new ScheduleInfo(mods, semester, shortName, longName, teacher, room, start, end));
}

class ScheduleInfo {
    constructor(mods, semester, shortName, longName, teacher, room, start, end) {
        this.mods = mods;
        this.semester = semester;
        this.shortName = shortName;
        this.longName = longName;
        this.teacher = teacher;
        this.room = room;
        this.start = start;
        this.end = end;
    }
}

function sortByDay(scheduleInfo) {
    return scheduleInfo.mods.map((modsDays, index) => {
        let [modStr, dayStr] = modsDays.split("(");
        dayStr = dayStr.replace(")", "");
        let days = parseDay(dayStr);
        let mods = parseMods(modStr);

        return new Class(
            days,
            mods,
            scheduleInfo.semester[index] === "S1" ? Semester.S1 : Semester.S2,
            scheduleInfo.shortName[index],
            scheduleInfo.longName[index],
            scheduleInfo.teacher[index],
            scheduleInfo.room[index],
            scheduleInfo.start[index],
            scheduleInfo.end[index]
        );
    });
}

function parseDay(dayStr) {
    let days = [];
    for (let day of dayStr.split(",").map(str => str.trim())) {
        switch (day) {
            case "A": days.push(Day.A); break;
            case "B": days.push(Day.B); break;
            case "C": days.push(Day.C); break;
            case "D": days.push(Day.D); break;
            case "I": days.push(Day.I); break;
            default:
                let [start, end] = [day.charAt(0), day.slice(2)];
                for (let d = start.charCodeAt(0); d <= end.charCodeAt(0); d++) {
                    days.push(parseDayChar(String.fromCharCode(d)));
                }
                break;
        }
    }
    return days;
}

function parseDayChar(ch) {
    switch (ch) {
        case 'A': return Day.A;
        case 'B': return Day.B;
        case 'C': return Day.C;
        case 'D': return Day.D;
        case 'I': return Day.I;
        default: throw new Error(`Unknown day char: ${ch}`);
    }
}

function parseMods(modStr) {
    let mods = [];
    for (let part of modStr.split("-")) {
        let start = parseInt(part.charAt(0));
        let end = parseInt(part.charAt(part.length - 1));
        for (let i = start; i <= end; i++) {
            mods.push(i);
        }
    }
    return mods;
}

function generatePath(weeklySchedule) {
    let weeklyPath = Array.from({ length: 5 }, () => Array(8).fill(""));

    for (let day = 0; day < 5; day++) {
        let letterDay;
        switch (day) {
            case 0: letterDay = Day.A; break;
            case 1: letterDay = Day.B; break;
            case 2: letterDay = Day.I; break;
            case 3: letterDay = Day.C; break;
            case 4: letterDay = Day.D; break;
            default: throw new Error("Unexpected day value");
        }

        for (let classObj of weeklySchedule) {
            if (classObj.days.includes(letterDay)) {
                for (let mod of classObj.mods) {
                    weeklyPath[day][mod - 1] = classObj.room;
                }
            }
        }
    }

    return weeklyPath;
}

document.addEventListener('DOMContentLoaded', () => {
    console.log('Script loaded'); // Confirm script loading
    document.getElementById('scheduleForm').addEventListener('submit', function (e) {
        e.preventDefault(); // Prevent the default form submission action
        console.log('Form submitted'); // Confirm form submission is intercepted

        const scheduleInput = document.getElementById('scheduleInput').value;
        const pathOutput = document.getElementById('pathOutput');
        const errorOutput = document.getElementById('errorOutput');

        try {
            const schedule = getSchedule(scheduleInput);
            const path = generatePath(schedule);

            let pathHtml = '<p>Path:</p>';
            path.forEach((day, i) => {
                pathHtml += `<div><p>Day ${i + 1}:</p><ul>`;
                day.forEach((room, j) => {
                    pathHtml += `<li>Module ${j + 1}: ${room || 'N/A'}</li>`;
                });
                pathHtml += `</ul></div>`;
            });

            pathOutput.innerHTML = pathHtml;
            errorOutput.innerHTML = ''; // Clear any previous errors
        } catch (error) {
            errorOutput.innerHTML = `Error: ${error.message}`;
            pathOutput.innerHTML = ''; // Clear any previous path
        }
    });
});
