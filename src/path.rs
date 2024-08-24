//as an example, below is a schedule that could be entered in by the user,
/*
Y24-25 Semester 1
Exp    Trm    Crs-Sec    Course Name    Teacher    Room    Enroll    Leave
2(A,C-D)    S1    ENG201b-1    Literary Explorations III: British    Townsend, Tracy A    A113    08/19/2024    01/19/2025
4(A-D)    S1    FAR110-1    Wind Ensemble    McCarthy, Mary Beth C    D107    08/19/2024    01/19/2025
5(A-B,D)    S1    HSS201b-3    Conflict in World History    Eysturlid, Lee    A147    08/19/2024    01/19/2025
6(A-D)    S1    WLG250-101    Spanish V    Kaluza, Marta J    A131    08/19/2024    01/19/2025
7(A-D)    S1    MAT473-2    Linear Algebra    Brummet, Evan    A135    08/19/2024    01/19/2025
8(A-D)    S1    MAT474-1    Abstract Algebra    Fogel, Micah    A155    08/19/2024    01/19/2025
*/
pub fn get_schedule(input: &str) -> Result<Vec<Class>, &str> {
    let mut listvec: Vec<String> = input
        .lines()
        .map(std::string::ToString::to_string)
        .collect();

    if listvec.len() <= 2 {
        return Err("Not enough arguments");
    }

    listvec = listvec[2..].to_vec();

    let mut mods = Vec::new();
    let mut semester = Vec::new();
    let mut short_name = Vec::new();
    let mut long_name = Vec::new();
    let mut teacher = Vec::new();
    let mut room = Vec::new();
    let mut start = Vec::new();
    let mut end = Vec::new();

    for line in listvec {
        let line = line.replace("    ", "\t");
        let split: Vec<String> = line.split('\t').map(|s| s.trim().to_string()).collect();
        assert!(split.len() >= 8, "Unexpected format: {line}");
        mods.push(split[0].clone());
        semester.push(split[1].clone());
        short_name.push(split[2].clone());
        long_name.push(split[3].clone());
        teacher.push(split[4].clone());
        room.push(split[5].clone());
        start.push(split[6].clone());
        end.push(split[7].clone());
    }

    Ok(sort_by_day(&ScheduleInfo {
        mods,
        semester,
        short_name,
        long_name,
        teacher,
        room,
        start,
        end,
    }))
}

fn parse_day(day_str: &str) -> Vec<Day> {
    let mut days = vec![];
    for day in day_str.split(',').map(str::trim) {
        match day {
            "A" => days.push(Day::A),
            "B" => days.push(Day::B),
            "C" => days.push(Day::C),
            "D" => days.push(Day::D),
            "I" => days.push(Day::I),
            _ => {
                let (start, end) = day.split_at(1);
                if let Some(start) = start.chars().next() {
                    let end = end.trim_start_matches('-').chars().next();
                    'here: for d in start..=end.unwrap_or(start) {
                        let Some(result) = parse_day_char(d) else {
                            break 'here;
                        };
                        days.push(result);
                    }
                } else {
                    panic!("Unknown day pattern: {day}")
                }
            }
        }
    }
    days
}

fn parse_day_char(ch: char) -> Option<Day> {
    match ch {
        'A' => Some(Day::A),
        'B' => Some(Day::B),
        'C' => Some(Day::C),
        'D' => Some(Day::D),
        'I' => Some(Day::I),
        _ => None,
    }
}

fn parse_mods(mod_str: &str) -> Vec<u8> {
    let mut mods = vec![];
    for part in mod_str.split('-') {
        if let Some(start_char) = part.chars().next() {
            if let Some(end_char) = part.chars().last() {
                if let (Some(start), Some(end)) = (start_char.to_digit(10), end_char.to_digit(10)) {
                    for m in start..=end {
                        mods.push(u8::try_from(m).unwrap());
                    }
                }
            }
        }
    }
    mods
}

fn sort_by_day(schedule_info: &ScheduleInfo) -> Vec<Class> {
    schedule_info
        .mods
        .iter()
        .enumerate()
        .map(|(item, mods_days)| {
            let (mod_str, day_str) = mods_days.split_once('(').unwrap();
            let days = parse_day(day_str.trim_end_matches(')'));
            let mods = parse_mods(mod_str);

            Class {
                days,
                mods,
                semester: match schedule_info.semester[item].as_str() {
                    "S1" => Semester::S1,
                    "S2" => Semester::S2,
                    "Y24-25" => Semester::Year,
                    _ => panic!("Unknown semester: {}", schedule_info.semester[item]),
                },
                short_name: schedule_info.short_name[item].clone(),
                long_name: schedule_info.long_name[item].clone(),
                teacher: schedule_info.teacher[item].clone(),
                room: schedule_info.room[item].clone(),
                start: schedule_info.start[item].clone(),
                end: schedule_info.end[item].clone(),
            }
        })
        .collect()
}

pub fn path(weekly_schedule: &Vec<Class>) -> [[String; 8]; 5] {
    let mut weekly_path: [[String; 8]; 5] = [
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    ];

    for day in 0..5 {
        let letter_day: Day = match day {
            0 => Day::A,
            1 => Day::B,
            2 => Day::I,
            3 => Day::C,
            4 => Day::D,
            _ => unreachable!("i hope this is unreachable"),
        };

        for class in weekly_schedule {
            if class.days.contains(&letter_day) {
                for mods in &class.mods {
                    match mods {
                        1 => weekly_path[day][0].clone_from(&class.room),
                        2 => weekly_path[day][1].clone_from(&class.room),
                        3 => weekly_path[day][2].clone_from(&class.room),
                        4 => weekly_path[day][3].clone_from(&class.room),
                        5 => weekly_path[day][4].clone_from(&class.room),
                        6 => weekly_path[day][5].clone_from(&class.room),
                        7 => weekly_path[day][6].clone_from(&class.room),
                        8 => weekly_path[day][7].clone_from(&class.room),
                        _ => panic!("Unexpected val --> {mods}"),
                    }
                }
            }
        }
    }

    weekly_path
}
struct ScheduleInfo {
    mods: Vec<String>,
    semester: Vec<String>,
    short_name: Vec<String>,
    long_name: Vec<String>,
    teacher: Vec<String>,
    room: Vec<String>,
    start: Vec<String>,
    end: Vec<String>,
}

#[derive(Debug)]
enum Semester {
    S1,
    S2,
    Year,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Day {
    A,
    B,
    C,
    D,
    I,
}

#[derive(Debug)]
pub struct Class {
    days: Vec<Day>,
    mods: Vec<u8>,
    semester: Semester,
    short_name: String,
    long_name: String,
    teacher: String,
    room: String,
    start: String,
    end: String,
}
