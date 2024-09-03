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
use lazy_static::lazy_static;
use regex::Regex;

use crate::{name_to_id, pathfinding, Node};
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

#[derive(Debug, Copy, Clone, serde::Serialize)]
enum Semester {
    S1,
    S2,
    Year,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, serde::Serialize)]
enum Day {
    A,
    B,
    C,
    D,
    I,
}
pub type FullClass = (Vec<usize>, (Option<Class>, Option<Class>));

#[derive(Debug, Clone, serde::Serialize)]
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

#[derive(Debug, serde::Serialize)]
pub struct DailyNode {
    pub anode: Option<Vec<FullClass>>,
    pub bnode: Option<Vec<FullClass>>,
    pub inode: Option<Vec<FullClass>>,
    pub cnode: Option<Vec<FullClass>>,
    pub dnode: Option<Vec<FullClass>>,
}

pub enum EnterExit {
    WestMain,
    EastMain,
    D13,
    D6,
}

fn split_semesters(input: &str) -> (String, String) {
    let mut sem1 = String::new();
    let mut sem2 = String::new();

    let mut in_sem2 = false;

    for line in input.lines() {
        if line.contains("Semester 2") {
            in_sem2 = true;
        }
        if in_sem2 {
            sem2.push_str(line);
            sem2.push('\n');
        } else {
            sem1.push_str(line);
            sem1.push('\n');
        }
    }

    (sem1.trim().to_string(), sem2.trim().to_string())
}

pub fn get_schedule(input: &str) -> (Result<Vec<Class>, String>, Result<Vec<Class>, String>) {
    let (sem1, sem2) = split_semesters(input);
    if sem1.is_empty() || sem2.is_empty() {
        return (
            Err("Please provide both semesters".to_owned()),
            Ok(Vec::new()),
        );
    };
    let sem_1_class = resolve_semester(&sem1);
    let sem_2_class = resolve_semester(&sem2);
    (sem_1_class, sem_2_class)
}

fn resolve_semester(input: &str) -> Result<Vec<Class>, String> {
    let mut listvec: Vec<String> = input
        .lines()
        .map(std::string::ToString::to_string)
        .collect();

    if listvec.len() <= 2 {
        return Err("Not enough lines".to_owned());
    }
    listvec.retain(|line| !line.trim().is_empty());
    listvec.retain(|line| !line.starts_with("RC") && !line.starts_with("CC"));

    if listvec[0].contains("Semester") {
        listvec = listvec[1..].to_vec();
        if listvec[0].contains("Teacher") && listvec[0].contains("Crs-Sec") {
            listvec = listvec[1..].to_vec();
        }
    } else if listvec[0].contains("Teacher") && listvec[0].contains("Crs") {
        listvec = listvec[1..].to_vec();
    }

    let mut mods: Vec<String> = Vec::new();
    let mut semester: Vec<String> = Vec::new();
    let mut short_name: Vec<String> = Vec::new();
    let mut long_name: Vec<String> = Vec::new();
    let mut teacher = Vec::new();
    let mut room: Vec<String> = Vec::new();
    let mut start: Vec<String> = Vec::new();
    let mut end: Vec<String> = Vec::new();

    for (num, line) in listvec.into_iter().enumerate() {
        let line = line.replace("    ", "\t");
        let split: Vec<String> = line
            .trim()
            .split('\t')
            .map(|s| s.trim().to_string())
            .collect();
        if split.len() == 8 {
            mods.push(split[0].clone());
            semester.push(split[1].clone());
            short_name.push(split[2].clone());
            long_name.push(split[3].clone());
            teacher.push(split[4].clone());
            room.push(split[5].clone());
            start.push(split[6].clone());
            end.push(split[7].clone());
        } else {
            return Err(format!("Not enough arguments provided in line {num}"));
        }
    }

    sort_by_day(&ScheduleInfo {
        mods,
        semester,
        short_name,
        long_name,
        teacher,
        room,
        start,
        end,
    })
}
lazy_static! {
    static ref DAY_REGEX: Regex = Regex::new(r"^[ ABCDI,-]*$").unwrap();
    static ref MODS_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9-]*$").unwrap();
    static ref CURREG_REGEX: Regex = Regex::new(r"^[\w-]+\([^()]*\)( [\w-]+\([^()]*\))*$").unwrap();
    static ref MODS_DAY_REGEX: Regex = Regex::new(r"\d+-?\d*\([^)]+\)|\d+\([^)]+\)").unwrap();
}
fn parse_day(day_str: &str) -> Result<Vec<Day>, String> {
    if !DAY_REGEX.is_match(day_str) {
        return Err(format!("Invalid day value: {day_str}"));
    }

    let mut days = vec![];
    for day in day_str.split(',').map(str::trim) {
        match day {
            "A" => days.push(Day::A),
            "B" => days.push(Day::B),
            "C" => days.push(Day::C),
            "D" => days.push(Day::D),
            "I" => days.push(Day::I),
            _ if day.contains('-') => {
                let (start, end) = day
                    .split_once('-')
                    .ok_or_else(|| format!("Invalid range: {day}"))?;
                let start = start
                    .chars()
                    .next()
                    .ok_or_else(|| format!("Invalid start day: {day}"))?;
                let end = end.chars().next().unwrap_or(start);
                days.extend((start..=end).filter_map(|d| match d {
                    'A' => Some(Day::A),
                    'B' => Some(Day::B),
                    'C' => Some(Day::C),
                    'D' => Some(Day::D),
                    'I' => Some(Day::I),
                    _ => None,
                }));
            }
            _ => return Err(format!("Unknown day pattern: {day}")),
        }
    }
    Ok(days)
}

fn parse_mods(mod_str: &str) -> Result<Vec<u8>, String> {
    if !MODS_REGEX.is_match(mod_str) {
        return Err(format!("Invalid mod value: {mod_str}"));
    }
    let mut mods = vec![];
    for part in mod_str.split('-') {
        if let Some(start_char) = part.chars().next() {
            if let Some(end_char) = part.chars().last() {
                if let (Some(start), Some(end)) = (start_char.to_digit(10), end_char.to_digit(10)) {
                    for m in start..=end {
                        match u8::try_from(m) {
                            Ok(var) => mods.push(var),
                            Err(err) => return Err(err.to_string()),
                        }
                    }
                }
            }
        }
    }
    Ok(mods)
}

fn sort_by_day(schedule_info: &ScheduleInfo) -> Result<Vec<Class>, String> {
    schedule_info
        .mods
        .iter()
        .enumerate()
        .map(|(item, mods_days)| {
            let curreg = Regex::new(r"^[\w-]+\([^()]*\)( [\w-]+\([^()]*\))*$").unwrap();

            let parsed_mods_days = if curreg.is_match(mods_days) {
                let re = Regex::new(r"\d+-?\d*\([^)]+\)|\d+\([^)]+\)").unwrap();
                let vals: Vec<&str> = re.find_iter(mods_days).map(|mat| mat.as_str()).collect();
                vals.iter()
                    .map(|mods_day_val| {
                        let (mod_str, day_str) = mods_day_val.split_once('(').unwrap();
                        let days: Vec<Day> = parse_day(day_str.trim_end_matches(')'))?;
                        let mods: Vec<u8> = parse_mods(mod_str)?;
                        Ok((days, mods))
                    })
                    .collect::<Result<Vec<(Vec<Day>, Vec<u8>)>, String>>()?
            } else {
                let Some((mod_str, day_str)) = mods_days.split_once('(') else {
                    return Err(format!(
                        "There was no \"(\" token in line {item}. The problematic input was {mods_days}"
                    ));
                };
                let days: Vec<Day> = parse_day(day_str.trim_end_matches(')'))?;
                let mods: Vec<u8> = parse_mods(mod_str)?;
                vec![(days, mods)]
            };
            let (all_days, all_mods): (Vec<_>, Vec<_>) = parsed_mods_days.into_iter().fold(
                (vec![], vec![]),
                |(mut days, mut mods), (d, m)| {
                    days.extend(d);
                    mods.extend(m);
                    (days, mods)
                },
            );

            Ok(Class {
                days: all_days,
                mods: all_mods,
                semester: match schedule_info.semester[item].as_str() {
                    "S1" => Semester::S1,
                    "S2" => Semester::S2,
                    "Y24-25" => Semester::Year,
                    _ => {
                        return Err(format!(
                            "Unknown semester '{}'",
                            schedule_info.semester[item]
                        ))
                    }
                },
                short_name: schedule_info.short_name[item].clone(),
                long_name: schedule_info.long_name[item].clone(),
                teacher: schedule_info.teacher[item].clone(),
                room: schedule_info.room[item].clone(),
                start: schedule_info.start[item].clone(),
                end: schedule_info.end[item].clone(),
            })
        })
        .collect()
}
pub fn path(weekly_schedule: &Vec<Class>) -> Result<[[&Class; 8]; 5], String> {
    static DEFAULT_CLASS: Class = Class {
        days: Vec::new(),
        mods: Vec::new(),
        semester: Semester::Year,
        short_name: String::new(),
        long_name: String::new(),
        teacher: String::new(),
        room: String::new(),
        start: String::new(),
        end: String::new(),
    };

    let mut weekly_path: [[&Class; 8]; 5] = [[&DEFAULT_CLASS; 8]; 5];

    for (day, _item) in weekly_path.clone().iter_mut().enumerate() {
        let letter_day: Day = match day {
            0 => Day::A,
            1 => Day::B,
            2 => Day::I,
            3 => Day::C,
            4 => Day::D,
            _ => {
                return Err(format!(
                    "If you are seeing this, things went really bad. 
                    DEBUG VAL {day}"
                ))
            }
        };

        for class in weekly_schedule {
            if class.days.contains(&letter_day) {
                for &mod_num in &class.mods {
                    let index = mod_num as usize - 1;
                    if index < 8 {
                        weekly_path[day][index] = class;
                    } else {
                        return Err(format!("Unexpected mod value --> {mod_num}"));
                    }
                }
            }
        }
    }

    Ok(weekly_path)
}

#[allow(clippy::too_many_lines)]
pub fn node_find_func(
    schedule: &[[&Class; 8]; 5],
    mut nodes: Vec<Node>,
    entrance: &EnterExit,
    exit: &EnterExit,
    checked: bool,
) -> Result<(DailyNode, Vec<Node>), String> {
    let mut master_vec: Vec<Vec<Option<FullClass>>> = Vec::new();

    for day in schedule {
        let mut vec: Vec<Option<FullClass>> = Vec::new();

        for (num, class) in day.iter().enumerate() {
            let mut class_name: &String = &class.room;
            if num == 3 {
                vec.push(None);
            }
            let earlyclass = class_name.to_lowercase();
            class_name = &earlyclass;

            if class_name.is_empty() {
                continue;
            }

            let start_room = name_to_id(class_name, &nodes)
                .ok_or(format!("The room '{class_name}' was not recognized"))?;

            for offset in 1..8 - num {
                if let Some(next_class) = day.get(num + offset) {
                    let next_class_name = &next_class.room;
                    if !next_class_name.is_empty() {
                        let next_room: usize = name_to_id(&next_class_name.to_lowercase(), &nodes)
                            .ok_or(format!("The room '{next_class_name}' was not recognized"))?;
                        if start_room != next_room {
                            vec.push(Some((
                                [start_room, next_room].to_vec(),
                                (Some((*class).clone()), Some((*next_class).clone())),
                            )));
                        }
                        break;
                    }
                }
            }
        }
        master_vec.push(vec);
    }

    let mut dailynode = DailyNode {
        anode: None,
        bnode: None,
        inode: None,
        cnode: None,
        dnode: None,
    };

    for (num, day) in master_vec.into_iter().enumerate() {
        let mut dayvec: Vec<FullClass> = Vec::new();
        if day.get(num).is_some() {
            let Some(start) = day.first() else {
                return Err(format!("Couldn't take a first on day. Day value: {day:?}"));
            };
            let end_id = match start {
                Some(val) => val.0[0],
                None => 55,
            };
            let shortest_path_st = pathfinding::time_path(
                {
                    match entrance {
                        EnterExit::WestMain => 146,
                        EnterExit::EastMain => 0,
                        EnterExit::D13 => 145,
                        EnterExit::D6 => 147,
                    }
                },
                end_id,
                &mut nodes,
            );
            dayvec.push((
                shortest_path_st,
                (None, start.as_ref().map(|val| val.1 .0.clone().unwrap())),
            ));
        }

        for (iter, vecpath) in day.clone().into_iter().enumerate() {
            if vecpath.is_none() {
                if checked && day[iter.saturating_sub(1)].is_some() {
                    let to_lex: Vec<usize> = pathfinding::time_path(
                        day[iter.saturating_sub(1)].clone().unwrap().0[1],
                        55,
                        &mut nodes,
                    );
                    let from_lex: Vec<usize> =
                        pathfinding::time_path(55, day[iter + 1].clone().unwrap().0[1], &mut nodes);
                    dayvec.push((
                        to_lex,
                        (day[iter.saturating_sub(1)].clone().unwrap().1 .0, None),
                    ));
                    dayvec.push((from_lex, (None, day[iter + 1].clone().unwrap().1 .1)));
                } else if day[iter.saturating_sub(1)].is_some() {
                    let to_norm: Vec<usize> = pathfinding::time_path(
                        day[iter.saturating_sub(1)].clone().unwrap().0[1],
                        day[iter + 1].clone().unwrap().0[1],
                        &mut nodes,
                    );
                    dayvec.push((
                        to_norm,
                        (
                            day[iter.saturating_sub(1)].clone().unwrap().1 .1,
                            day[iter + 1].clone().unwrap().1 .1,
                        ),
                    ));
                }
            } else if day[iter.saturating_sub(1)].is_some() {
                let shortest_path: Vec<usize> = pathfinding::time_path(
                    vecpath.clone().unwrap().0[0],
                    vecpath.clone().unwrap().0[1],
                    &mut nodes,
                );
                dayvec.push((shortest_path, vecpath.clone().unwrap().1));
            }
        }
        if day.get(num).is_some() {
            let shortest_path_en: Vec<usize> = pathfinding::time_path(
                match day.last().unwrap() {
                    Some(result) => result.0[1],
                    None => 55,
                },
                {
                    match exit {
                        EnterExit::WestMain => 146,
                        EnterExit::EastMain => 0,
                        EnterExit::D13 => 145,
                        EnterExit::D6 => 147,
                    }
                },
                &mut nodes,
            );
            dayvec.push((
                shortest_path_en,
                (
                    {
                        match day.last().unwrap() {
                            Some(result) => result.1 .1.clone(),
                            None => None,
                        }
                    },
                    None,
                ),
            ));
        }

        match num {
            0 => dailynode.anode = Some(dayvec),
            1 => dailynode.bnode = Some(dayvec),
            2 => dailynode.inode = Some(dayvec),
            3 => dailynode.cnode = Some(dayvec),
            4 => dailynode.dnode = Some(dayvec),
            _ => return Err(format!("Unexpected num {num}")),
        }
    }

    Ok((dailynode, nodes))
}
