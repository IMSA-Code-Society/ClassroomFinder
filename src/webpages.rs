use crate::path::{node_find_func, Class, DailyNode, EnterExit, FullClass};
use crate::{
    file_utils, name_to_id, path::get_schedule, path::path, pathfinding, reset_nodes, Node,
};

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
    let css: String = tokio::fs::read_to_string("assets/home.css").await.unwrap();
    HttpResponse::Ok().content_type("text/css").body(css)
}

fn serve_html(path: &str) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read_to_string(path).unwrap())
}

pub async fn save(mut payload: Multipart) -> Result<HttpResponse> {
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some("file") = field.name() {
            let data: web::Bytes = field.next().await.unwrap().unwrap();
            let mut file: tokio::fs::File =
                tokio::fs::File::create("assets/nodes.json").await.unwrap();
            file.write_all(&data).await.unwrap();
        }
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": 0})))
}

pub async fn directions(web::Json(request): web::Json<serde_json::Value>) -> impl Responder {
    let mut nodes: Vec<Node> =
        file_utils::read_nodes_from_file("assets/nodes.json").expect("Failed to read nodes");

    reset_nodes(&mut nodes);

    let start_room: usize =
        name_to_id(request["start-room"].as_str().unwrap(), &nodes).unwrap_or(usize::MAX);
    let destination: usize =
        name_to_id(request["destination"].as_str().unwrap(), &nodes).unwrap_or(usize::MAX);

    if start_room == usize::MAX || destination == usize::MAX {
        println!("Error: Invalid start or destination room");
        return HttpResponse::BadRequest().json(serde_json::json!({"status": 1}));
    }

    let shortest_path: Vec<usize> = pathfinding::time_path(start_room, destination, &mut nodes);
    let path_json: Vec<serde_json::Value> = build_direct_json(&shortest_path, &nodes);

    HttpResponse::Ok().json(serde_json::json!({ "path": path_json }))
}

fn build_path_json(path: &FullClass, nodes: &[Node]) -> serde_json::Value {
    let nodes: serde_json::Value = path
        .0
        .iter()
        .map(|&index| {
            serde_json::json!({
                "x": nodes[index].x,
                "y": nodes[index].y,
                "name": nodes[index].name,
                "index": index
            })
        })
        .collect();
    let real_json = serde_json::json!({
        "info": path.1.clone(),
        "nodes": nodes,
    });
    real_json
}
fn build_direct_json(path: &[usize], nodes: &[Node]) -> Vec<serde_json::Value> {
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

pub async fn schedule_handle(web::Json(request): web::Json<serde_json::Value>) -> impl Responder {
    let user_input: &str = request["Schedule Input"].as_str().unwrap();
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
    let checked: bool = request["LexMidday"].as_bool().unwrap();
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

    let schedule_1: [[&Class; 8]; 5] = match path(&sem1) {
        Ok(result) => result,
        Err(err) => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": 1, "error_message": err,}))
        }
    };

    let schedule_2: [[&Class; 8]; 5] = match path(&sem2) {
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
    let json: serde_json::Value = build_schedule_json(path_master_vec_1, path_master_vec_2, &nodes);

    HttpResponse::Ok().json(json)
}

fn build_schedule_json(
    path_master_vec_1: DailyNode,
    path_master_vec_2: DailyNode,
    nodes: &[Node],
) -> serde_json::Value {
    
    let day_vecs1: [Option<Vec<FullClass>>; 5] = [
        path_master_vec_1.anode,
        path_master_vec_1.bnode,
        path_master_vec_1.inode,
        path_master_vec_1.cnode,
        path_master_vec_1.dnode,
    ];

    let day_vecs2: [Option<Vec<FullClass>>; 5] = [
        path_master_vec_2.anode,
        path_master_vec_2.bnode,
        path_master_vec_2.inode,
        path_master_vec_2.cnode,
        path_master_vec_2.dnode,
    ];
    let day_names: [&str; 5] = ["aday", "bday", "iday", "cday", "dday"];

    let mut json1: serde_json::Value = serde_json::json!({});
    let mut json2: serde_json::Value = serde_json::json!({});

    for (day_name, day_vec) in day_names.iter().zip(day_vecs1.iter()) {
        if let Some(val) = day_vec {
            let day_paths: Vec<serde_json::Value> = val
                .iter()
                .map(|secval: &FullClass| build_path_json(secval, nodes))
                .collect();
            json1[day_name] = serde_json::json!(day_paths);
        }
    }
    for (day_name, day_vec) in day_names.iter().zip(day_vecs2.iter()) {
        if let Some(val) = day_vec {
            let day_paths: Vec<serde_json::Value> = val
                .iter()
                .map(|secval| build_path_json(secval, nodes))
                .collect();
            json2[day_name] = serde_json::json!(day_paths);
        }
    }
    let final_json: serde_json::Value = serde_json::json!(
        {
            "Semester 1": json1,
            "Semester 2": json2

        }
    );

    final_json
}
