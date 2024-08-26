use actix_files::Files;
use actix_web::web;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::path::Path;
use webpages::{
    about, css_handler, directions, editor, home_page, image, input, save, schedule_handle,
};
mod path;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Node {
    x: f64,
    y: f64,
    name: String,
    id: usize,
    #[serde(rename = "neighbor_nodes")]
    nodes: Vec<(usize, f64)>,
    #[serde(default)]
    dist: f64,
    #[serde(default)]
    previous: Option<usize>,
}

mod file_utils {
    use super::{from_reader, Node, Path};
    use serde::de::Error;
    use std::fs::File;

    pub fn read_nodes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Node>, serde_json::Error> {
        let file = File::open(path).map_err(|e| serde_json::Error::custom(e.to_string()))?;
        from_reader(file)
    }
}

mod pathfinding {
    use super::Node;
    use std::f64;

    pub fn time_path(start_id: usize, end_id: usize, nodes: &mut [Node]) -> Vec<usize> {
        initialize_nodes(nodes, start_id);

        let mut unvisited_nodes: Vec<usize> = (0..nodes.len()).collect();

        while let Some(current_index) = unvisited_nodes
            .iter()
            .min_by(|&&a, &&b| nodes[a].dist.partial_cmp(&nodes[b].dist).unwrap())
        {
            let current_index = *current_index;

            if nodes[current_index].dist == f64::INFINITY {
                println!("Node {current_index} is unreachable, stopping.");
                break;
            }

            unvisited_nodes.retain(|&x| x != current_index);

            for &(neighbor_id, weight) in &nodes[current_index].nodes {
                if let Some(neighbor_index) = get_index(nodes, neighbor_id) {
                    let distance = nodes[current_index].dist + weight;

                    if distance < nodes[neighbor_index].dist {
                        nodes[neighbor_index].dist = distance;
                        nodes[neighbor_index].previous = Some(nodes[current_index].id);
                    }
                }
            }
        }

        construct_path(start_id, end_id, nodes)
    }

    fn initialize_nodes(nodes: &mut [Node], start_id: usize) {
        for node in nodes.iter_mut() {
            node.dist = f64::INFINITY;
            node.previous = None;
        }
        nodes[start_id].dist = 0.0;
    }

    fn construct_path(start_id: usize, end_id: usize, nodes: &[Node]) -> Vec<usize> {
        let mut path = vec![];
        let mut current_id = end_id;
        while let Some(prev_id) = nodes[current_id].previous {
            path.push(current_id);
            current_id = prev_id;
            if current_id == start_id {
                path.push(start_id);
                break;
            }
        }
        path.reverse();
        path
    }

    fn get_index(nodes: &[Node], id: usize) -> Option<usize> {
        nodes.iter().position(|node| node.id == id)
    }
}

mod webpages {

    use crate::{
        file_utils, name_to_id, path::get_schedule, path::path, pathfinding, reset_nodes, Node,
    };
    use crate::{node_find_func, DailyNode, EnterExit};
    use actix_multipart::Multipart;
    use actix_web::{web, HttpResponse, Responder, Result};
    use futures::StreamExt as _;
    use futures::TryStreamExt;
    use tokio::io::AsyncWriteExt;

    pub async fn home_page() -> impl Responder {
        serve_html("assets/path/home.html")
    }

    pub async fn input() -> impl Responder {
        serve_html("assets/input/index.html")
    }

    pub async fn editor() -> impl Responder {
        serve_html("assets/editor/editor.html")
    }
    pub async fn about() -> impl Responder {
        serve_html("assets/about/about.html")
    }

    pub async fn image() -> impl Responder {
        HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(std::fs::read("assets/imsa_hallway.jpg").unwrap())
    }

    pub async fn css_handler() -> impl Responder {
        let css = tokio::fs::read_to_string("assets/home.css").await.unwrap();
        HttpResponse::Ok().content_type("text/css").body(css)
    }

    fn serve_html(path: &str) -> impl Responder {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(std::fs::read_to_string(path).unwrap())
    }

    pub async fn save(mut payload: Multipart) -> Result<HttpResponse> {
        println!("Saving...");
        while let Ok(Some(mut field)) = payload.try_next().await {
            if let Some("file") = field.name() {
                let data = field.next().await.unwrap().unwrap();
                let mut file = tokio::fs::File::create("assets/nodes.json").await.unwrap();
                file.write_all(&data).await.unwrap();
            }
        }
        Ok(HttpResponse::Ok().json(serde_json::json!({"status": 0})))
    }

    pub async fn directions(web::Json(request): web::Json<serde_json::Value>) -> impl Responder {
        let mut nodes: Vec<Node> =
            file_utils::read_nodes_from_file("assets/nodes.json").expect("Failed to read nodes");

        reset_nodes(&mut nodes);

        let start_room =
            name_to_id(request["start-room"].as_str().unwrap(), &nodes).unwrap_or(usize::MAX);
        let destination =
            name_to_id(request["destination"].as_str().unwrap(), &nodes).unwrap_or(usize::MAX);

        if start_room == usize::MAX || destination == usize::MAX {
            println!("Error: Invalid start or destination room");
            return HttpResponse::BadRequest().json(serde_json::json!({"status": 1}));
        }

        let shortest_path = pathfinding::time_path(start_room, destination, &mut nodes);
        let path_json = build_path_json(&shortest_path, &nodes);

        HttpResponse::Ok().json(serde_json::json!({ "path": path_json }))
    }

    fn build_path_json(path: &[usize], nodes: &[Node]) -> Vec<serde_json::Value> {
        path.iter()
            .map(|&index| {
                serde_json::json!({
                    "x": nodes[index].x,
                    "y": nodes[index].y,
                    "name": nodes[index].name,
                    "index": index
                })
            })
            .collect()
    }

    pub async fn schedule_handle(
        web::Json(request): web::Json<serde_json::Value>,
    ) -> impl Responder {
        let user_input = request["Schedule Input"].as_str().unwrap();
        let enter: EnterExit = match request["Enter"].as_str().unwrap() {
            "west" => EnterExit::WestMain,
            "east" => EnterExit::EastMain,
            "d13" => EnterExit::D13,
            "d6" => EnterExit::D6,
            _ => panic!("Malformed json"),
        };
        let exit: EnterExit = match request["Exit"].as_str().unwrap() {
            "west" => EnterExit::WestMain,
            "east" => EnterExit::EastMain,
            "d13" => EnterExit::D13,
            "d6" => EnterExit::D6,
            _ => panic!("Malformed json"),
        };
        let checked = request["LexMidday"].as_bool().unwrap();
        let (sem1, sem2) = match get_schedule(user_input) {
            (Ok(sem1), Ok(sem2)) => (sem1, sem2),
            (Ok(_), Err(err)) | (Err(err), Ok(_)) => {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": 1, "error_message": err,}))
            }
            (Err(err1), Err(err2)) => {
                let full_err = format!("Semester 1 error: {err1} \nSemester 2 error: {err2}");
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": 1, "error_message": full_err,}));
            }
        };

        let schedule_1: [[String; 8]; 5] = match path(&sem1) {
            Ok(result) => result,
            Err(err) => {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": 1, "error_message": err,}))
            }
        };
        let schedule_2: [[String; 8]; 5] = match path(&sem2) {
            Ok(result) => result,
            Err(err) => {
                return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": 1, "error_message": err,}))
            }
        };

        let mut nodes: Vec<Node> =
            file_utils::read_nodes_from_file("assets/nodes.json").expect("Failed to read nodes");

        reset_nodes(&mut nodes);
        let (path_master_vec_1, nodes) =
            match node_find_func(&schedule_1, nodes, &enter, &exit, checked) {
                Ok(result) => result,
                Err(err) => {
                    return HttpResponse::BadRequest()
                        .json(serde_json::json!({"status": 1, "error_message": err,}));
                }
            };
        let (path_master_vec_2, nodes) =
            match node_find_func(&schedule_2, nodes, &enter, &exit, checked) {
                Ok(result) => result,
                Err(err) => {
                    return HttpResponse::BadRequest()
                        .json(serde_json::json!({"status": 1, "error_message": err,}));
                }
            };
        let json = build_schedule_json(path_master_vec_1, path_master_vec_2, &nodes);

        HttpResponse::Ok().json(json)
    }

    fn build_schedule_json(
        path_master_vec_1: DailyNode,
        path_master_vec_2: DailyNode,
        nodes: &[Node],
    ) -> serde_json::Value {
        let day_vecs1 = [
            path_master_vec_1.anode,
            path_master_vec_1.bnode,
            path_master_vec_1.inode,
            path_master_vec_1.cnode,
            path_master_vec_1.dnode,
        ];
        let day_vecs2 = [
            path_master_vec_2.anode,
            path_master_vec_2.bnode,
            path_master_vec_2.inode,
            path_master_vec_2.cnode,
            path_master_vec_2.dnode,
        ];
        let day_names = ["aday", "bday", "iday", "cday", "dday"];

        let mut json1: serde_json::Value = serde_json::json!({});
        let mut json2: serde_json::Value = serde_json::json!({});

        for (day_name, day_vec) in day_names.iter().zip(day_vecs1.iter()) {
            if let Some(val) = day_vec {
                let day_paths: Vec<Vec<serde_json::Value>> = val
                    .iter()
                    .map(|secval| build_path_json(secval, nodes))
                    .collect();
                json1[day_name] = serde_json::json!(day_paths);
            }
        }
        for (day_name, day_vec) in day_names.iter().zip(day_vecs2.iter()) {
            if let Some(val) = day_vec {
                let day_paths: Vec<Vec<serde_json::Value>> = val
                    .iter()
                    .map(|secval| build_path_json(secval, nodes))
                    .collect();
                json2[day_name] = serde_json::json!(day_paths);
            }
        }
        let final_json = serde_json::json!(
            {
                "Semester 1": json1,
                "Semester 2": json2

            }
        );

        final_json
    }
}

fn reset_nodes(nodes: &mut [Node]) {
    for node in nodes.iter_mut() {
        node.dist = f64::INFINITY;
        node.previous = None;
    }
}

fn name_to_id(name: &str, nodes: &[Node]) -> Option<usize> {
    nodes
        .iter()
        .find(|node| node.name == name)
        .map(|node| node.id)
}
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn shuttle_main(
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Clone + Send + 'static> {
    let factory = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("")
                .route("/", web::get().to(input))
                .route("/editor", web::get().to(editor))
                .route("/image", web::get().to(image))
                .route("/home.css", web::get().to(css_handler))
                .route("/save", web::post().to(save))
                .route("/get_directions", web::post().to(directions))
                .route("/schedule-post", web::post().to(schedule_handle))
                .route("/path", web::get().to(home_page))
                .route("/about", web::get().to(about))
                .service(Files::new("/assets", "assets")),
        );
    };

    Ok(shuttle_actix_web::ActixWebService(factory))
}
#[allow(clippy::too_many_lines)]
fn node_find_func(
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
                }
            } else {
                if day[iter.saturating_sub(1)].is_some() {
                    let shortest_path = pathfinding::time_path(
                        vecpath.unwrap()[0],
                        vecpath.unwrap()[1],
                        &mut nodes,
                    );
                    dayvec.push(shortest_path);
                }
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

#[derive(Debug)]
struct DailyNode {
    anode: Option<Vec<Vec<usize>>>,
    bnode: Option<Vec<Vec<usize>>>,
    inode: Option<Vec<Vec<usize>>>,
    cnode: Option<Vec<Vec<usize>>>,
    dnode: Option<Vec<Vec<usize>>>,
}

enum EnterExit {
    WestMain,
    EastMain,
    D13,
    D6,
}
