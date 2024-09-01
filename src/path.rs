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

#[derive(Debug)]
pub struct DailyNode {
    pub anode: Option<Vec<Vec<usize>>>,
    pub bnode: Option<Vec<Vec<usize>>>,
    pub inode: Option<Vec<Vec<usize>>>,
    pub cnode: Option<Vec<Vec<usize>>>,
    pub dnode: Option<Vec<Vec<usize>>>,
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

    let mut mods = Vec::new();
    let mut semester = Vec::new();
    let mut short_name = Vec::new();
    let mut long_name = Vec::new();
    let mut teacher = Vec::new();
    let mut room = Vec::new();
    let mut start = Vec::new();
    let mut end = Vec::new();

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
    let re = Regex::new(r"^[ ABCDI,-]*$").unwrap();
    if !re.is_match(day_str) {
        return Err(format!("This day value is invalid: {day_str}"));
    }

    let mut days = vec![];
    for day in day_str.split(',').map(str::trim) {
        match day {
            "A" => days.push(Day::A),
            "B" => days.push(Day::B),
            "C" => days.push(Day::C),
            "D" => days.push(Day::D),
            "I" => days.push(Day::I),
            _ => {
                if let Some((start_char, rest)) = day.split_once('-') {
                    let start = start_char.chars().next().unwrap();
                    let end = rest.chars().next().unwrap_or(start);
                    for d in start..=end {
                        if let Some(result) = match d {
                            'A' => Some(Day::A),
                            'B' => Some(Day::B),
                            'C' => Some(Day::C),
                            'D' => Some(Day::D),
                            'I' => Some(Day::I),
                            _ => None,
                        } {
                            days.push(result);
                        } else {
                            return Err(format!("Unknown day pattern: {day}"));
                        }
                    }
                } else {
                    return Err(format!("Unknown day pattern: {day}"));
                }
            }
        }
    }
    Ok(days)
}

fn parse_mods(mod_str: &str) -> Result<Vec<u8>, String> {
    let re = Regex::new(r"^[a-zA-Z0-9-]*$").unwrap();

    if !re.is_match(mod_str) {
        return Err(format!("This mod value is invalid: {mod_str}"));
    }
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
                let (mod_str, day_str) = mods_days.split_once('(').unwrap();
                let days: Vec<Day> = parse_day(day_str.trim_end_matches(')'))?;
                let mods: Vec<u8> = parse_mods(mod_str)?;
                vec![(days, mods)]
            };

            // Flatten all the parsed days and mods into one structure
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

pub fn path(weekly_schedule: &Vec<Class>) -> Result<[[String; 8]; 5], String> {
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
            _ => {
                return Err(format!(
                    "If you are seeing this, things went really bad. 
            DEBUG VAL {day}"
                ))
            }
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

    Ok(weekly_path)
}

#[allow(clippy::too_many_lines)]
pub fn node_find_func(
    schedule: &[[String; 8]; 5],
    mut nodes: Vec<Node>,
    entrance: &EnterExit,
    exit: &EnterExit,
    checked: bool,
) -> Result<(DailyNode, Vec<Node>), String> {
    let mut master_vec: Vec<Vec<Option<[usize; 2]>>> = Vec::new();

    for day in schedule {
        let mut vec: Vec<Option<[usize; 2]>> = Vec::new();

        for (num, mut class) in day.iter().enumerate() {
            if num == 3 {
                vec.push(None);
            }
            let earlyclass = class.to_lowercase();
            class = &earlyclass;

            if class.is_empty() {
                continue;
            }

            let start_room = name_to_id(class, &nodes)
                .ok_or(format!("The room '{class}' was not recognized"))?;

            for offset in 1..8 - num {
                if let Some(next_class) = day.get(num + offset) {
                    if !next_class.is_empty() {
                        let next_room = name_to_id(&next_class.to_lowercase(), &nodes)
                            .ok_or(format!("The room '{next_class}' was not recognized"))?;
                        if start_room != next_room {
                            vec.push(Some([start_room, next_room]));
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
        let mut dayvec: Vec<Vec<usize>> = Vec::new();
        if day.get(num).is_some() {
            let shortest_path_st = pathfinding::time_path(
                {
                    match entrance {
                        EnterExit::WestMain => 146,
                        EnterExit::EastMain => 0,
                        EnterExit::D13 => 145,
                        EnterExit::D6 => 147,
                    }
                },
                match day.first().unwrap() {
                    Some(val) => val[0],
                    None => 55,
                },
                &mut nodes,
            );
            dayvec.push(shortest_path_st);
        }

        for (iter, vecpath) in day.clone().into_iter().enumerate() {
            if vecpath.is_none() {
                if checked && day[iter.saturating_sub(1)].is_some() {
                    let to_lex: Vec<usize> = pathfinding::time_path(
                        day[iter.saturating_sub(1)].unwrap()[1],
                        55,
                        &mut nodes,
                    );
                    let from_lex: Vec<usize> =
                        pathfinding::time_path(55, day[iter + 1].unwrap()[1], &mut nodes);
                    dayvec.push(to_lex);
                    dayvec.push(from_lex);
                } else if day[iter.saturating_sub(1)].is_some() {
                    let to_norm: Vec<usize> = pathfinding::time_path(
                        day[iter.saturating_sub(1)].unwrap()[1],
                        day[iter + 1].unwrap()[1],
                        &mut nodes,
                    );
                    dayvec.push(to_norm);
                }
            } else if day[iter.saturating_sub(1)].is_some() {
                let shortest_path =
                    pathfinding::time_path(vecpath.unwrap()[0], vecpath.unwrap()[1], &mut nodes);
                dayvec.push(shortest_path);
            }
        }
        if day.get(num).is_some() {
            let shortest_path_en = pathfinding::time_path(
                match day.last().unwrap() {
                    Some(result) => result[1],
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
            dayvec.push(shortest_path_en);
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
