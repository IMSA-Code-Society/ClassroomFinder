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
lazy_static! {
    static ref DAY_REGEX: Regex = Regex::new(r"^[ ABCDI,-]*$").unwrap();
    static ref MODS_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9-]*$").unwrap();
    static ref CURREG_REGEX: Regex = Regex::new(r"^[\w-]+\([^()]*\)( [\w-]+\([^()]*\))*$").unwrap();
    static ref MODS_DAY_REGEX: Regex = Regex::new(r"\d+-?\d*\([^)]+\)|\d+\([^)]+\)").unwrap();
}
fn split_semesters(input: &str) -> (String, String) {
    let (mut sem1, mut sem2) = (Vec::new(), Vec::new());
    let mut in_sem2 = false;

    for line in input.lines() {
        if line.contains("Semester 2") {
            in_sem2 = true;
        }
        if in_sem2 {
            sem2.push(line);
        } else {
            sem1.push(line);
        }
    }

    (sem1.join("\n"), sem2.join("\n"))
}

pub fn get_schedule(input: &str) -> (Result<Vec<Class>, String>, Result<Vec<Class>, String>) {
    let (sem1, sem2) = split_semesters(input);

    if sem1.is_empty() || sem2.is_empty() {
        return (
            Err("Please provide both semesters".to_owned()),
            Ok(Vec::new()),
        );
    }

    (resolve_semester(&sem1), resolve_semester(&sem2))
}

fn resolve_semester(input: &str) -> Result<Vec<Class>, String> {
    let mut listvec: Vec<&str> = input
        .lines()
        .skip_while(|line| line.trim().is_empty() || line.contains("Semester"))
        .skip_while(|line| line.contains("Teacher") && line.contains("Crs-Sec"))
        .collect();

    if listvec.len() <= 2 {
        return Err("Not enough lines".to_owned());
    }

    listvec.retain(|line| {
        !line.trim().is_empty() && !line.starts_with("RC") && !line.starts_with("CC")
    });

    let (
        mut mods,
        mut semester,
        mut short_name,
        mut long_name,
        mut teacher,
        mut room,
        mut start,
        mut end,
    ) = (
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );

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

    Ok(mod_str
        .split('-')
        .map(|part| {
            let (start, end) = (part.chars().next(), part.chars().last());
            match (
                start.and_then(|c| c.to_digit(10)),
                end.and_then(|c| c.to_digit(10)),
            ) {
                (Some(start), Some(end)) => (start..=end).map(|m| m as u8).collect::<Vec<u8>>(),
                _ => vec![],
            }
        })
        .collect::<Vec<Vec<u8>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>())
}
fn sort_by_day(schedule_info: &ScheduleInfo) -> Result<Vec<Class>, String> {
    schedule_info
        .mods
        .iter()
        .enumerate()
        .map(|(item, mods_days)| {
            let parsed_mods_days = if CURREG_REGEX.is_match(mods_days) {
                MODS_DAY_REGEX.find_iter(mods_days)
                    .map(|mat| {
                        let (mod_str, day_str) = mat.as_str().split_once('(').unwrap();
                        Ok((parse_day(day_str.trim_end_matches(')'))?, parse_mods(mod_str)?))
                    })
                    .collect::<Result<Vec<_>, String>>()?
            } else {
                let (mod_str, day_str) = mods_days.split_once('(')
                    .ok_or_else(|| format!("There was no \"(\" token in line {item}. The problematic input was {mods_days}"))?;
                vec![(parse_day(day_str.trim_end_matches(')'))?, parse_mods(mod_str)?)]
            };

            let (all_days, all_mods) = parsed_mods_days.into_iter().fold(
                (Vec::new(), Vec::new()),
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
                    _ => return Err(format!("Unknown semester '{}'", schedule_info.semester[item])),
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
pub fn node_find_func(
    schedule: &[[&Class; 8]; 5],
    mut nodes: Vec<Node>,
    entrance: &EnterExit,
    exit: &EnterExit,
    checked: bool,
) -> Result<(DailyNode, Vec<Node>), String> {
    let mut master_vec: Vec<Vec<Option<FullClass>>> = Vec::new();

    for day in schedule {
        let mut day_vec: Vec<Option<FullClass>> = Vec::new();

        for (num, class) in day.iter().enumerate() {
            //(for midday)
            if num == 3 {
                day_vec.push(None);
            }

            if class.room.trim().is_empty() {
                continue;
            }

            let start_room = name_to_id(&class.room.trim().to_lowercase(), &nodes)
                .ok_or(format!("The room '{}' was not recognized", class.room))?;

            if let Some(next_class) = find_next_class(day, num) {
                let next_room = name_to_id(&next_class.room.trim().to_lowercase(), &nodes)
                    .ok_or(format!("The room '{}' was not recognized", next_class.room))?;
                if start_room != next_room {
                    day_vec.push(Some((
                        vec![start_room, next_room],
                        (Some((*class).clone()), Some(next_class.clone())),
                    )));
                }
            }
        }
        master_vec.push(day_vec);
    }

    let mut daily_node = DailyNode {
        anode: None,
        bnode: None,
        inode: None,
        cnode: None,
        dnode: None,
    };

    for (day_num, day) in master_vec.into_iter().enumerate() {
        let mut day_vec = Vec::new();

        if let Some(first_class) = day.first().and_then(|fc| fc.as_ref()) {

            let start_path = {
                let start_room_id = match entrance {
                    EnterExit::WestMain => 990,
                    EnterExit::EastMain => 989,
                    EnterExit::D13 => 692,
                    EnterExit::D6 => 693,
                };
                pathfinding::time_path(start_room_id, first_class.0[0], &mut nodes)
            };
            day_vec.push((start_path, (None, first_class.1 .0.clone())));
        }
        let mut skip = false;
        for (iter, vecpath) in day.iter().enumerate() {
            if vecpath.is_none() && checked {
                if let Some(prev_class) = day.get(iter.wrapping_sub(1)).and_then(|p| p.as_ref()) {
                    if let Some(next_class) = day.get(iter + 1).and_then(|n| n.as_ref()) {
                        let to_lex = pathfinding::time_path(prev_class.0[1], 354, &mut nodes);
                        let from_lex = pathfinding::time_path(354, next_class.0[1], &mut nodes);
                        day_vec.push((to_lex, (prev_class.1 .1.clone(), None)));
                        day_vec.push((from_lex, (None, next_class.1 .1.clone())));
                        skip = true;
                    }
                }
            } else if let Some(ref full_class) = vecpath {
                if skip {skip = false; continue;}
                let shortest_path =
                    pathfinding::time_path(full_class.0[0], full_class.0[1], &mut nodes);
                day_vec.push((shortest_path, full_class.1.clone()));
            }
        }

        if let Some(last_class) = day.last().and_then(|lc| lc.as_ref()) {
            let end_path = {
                let end_room_id = match exit {
                    EnterExit::WestMain => 990,
                    EnterExit::EastMain => 989,
                    EnterExit::D13 => 692,
                    EnterExit::D6 => 693,
                };
                pathfinding::time_path(last_class.0[1], end_room_id, &mut nodes)
            };
            day_vec.push((end_path, (last_class.1 .1.clone(), None)));
        }

        match day_num {
            0 => daily_node.anode = Some(day_vec),
            1 => daily_node.bnode = Some(day_vec),
            2 => daily_node.inode = Some(day_vec),
            3 => daily_node.cnode = Some(day_vec),
            4 => daily_node.dnode = Some(day_vec),
            _ => return Err(format!("Unexpected day_num: {day_num}")),
        }
    }

    Ok((daily_node, nodes))
}

fn find_next_class<'a>(day: &'a [&'a Class; 8], start_index: usize) -> Option<&'a Class> {
    for offset in 1..8 - start_index {
        if let Some(next_class) = day.get(start_index + offset) {
            if !next_class.room.trim().is_empty() {
                return Some(next_class);
            }
        }
    }
    None
}
