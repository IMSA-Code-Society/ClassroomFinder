use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder, Result};
use futures::StreamExt as _;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::path::Path;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

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
        println!("Starting pathfinding from node {start_id} to node {end_id}");

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

        let path = construct_path(start_id, end_id, nodes);
        println!("Constructed path: {path:?}");
        path
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

async fn home_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read_to_string("assets/home.html").unwrap())
}

async fn editor() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read_to_string("assets/editor.html").unwrap())
}

async fn image() -> impl Responder {
    HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(std::fs::read("assets/imsa_hallway.jpg").unwrap())
}

async fn css_handler() -> impl Responder {
    let css = tokio::fs::read_to_string("assets/home.css").await.unwrap();
    HttpResponse::Ok().content_type("text/css").body(css)
}

async fn save(mut payload: Multipart) -> Result<HttpResponse> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some("file") = field.name() {
            let data = field.next().await.unwrap().unwrap();
            let mut file = tokio::fs::File::create("nodes.json").await.unwrap();
            file.write_all(&data).await.unwrap();
        }
    }
    Ok(HttpResponse::Ok().json(serde_json::json!({"status": 0})))
}

async fn directions(web::Json(request): web::Json<serde_json::Value>) -> impl Responder {
    let mut nodes: Vec<Node> =
        file_utils::read_nodes_from_file("assets/nodes.json").expect("Failed to read nodes");

    reset_nodes(&mut nodes);

    let start_room =
        name_to_id(request["start-room"].as_str().unwrap(), &nodes).unwrap_or(usize::MAX);
    let destination =
        name_to_id(request["destination"].as_str().unwrap(), &nodes).unwrap_or(usize::MAX);

    println!("Received request: start-room = {start_room}, destination = {destination}");

    if start_room == usize::MAX || destination == usize::MAX {
        println!("Error: Invalid start or destination room");
        return HttpResponse::BadRequest().json(serde_json::json!({"status": 1}));
    }

    let shortest_path = pathfinding::time_path(start_room, destination, &mut nodes);

    let path_json: Vec<serde_json::Value> = shortest_path
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

    println!("Returning path JSON: {path_json:?}");

    HttpResponse::Ok().json(serde_json::json!({ "path": path_json }))
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
                .route("/", web::get().to(home_page))
                .route("/editor", web::get().to(editor))
                .route("/image", web::get().to(image))
                .route("/home.css", web::get().to(css_handler))
                .route("/save", web::post().to(save))
                .route("/get_directions", web::post().to(directions))
                .route("/input", web::get().to(input))
                
                .service(Files::new("/assets", "assets")),
        );
    };

    Ok(shuttle_actix_web::ActixWebService(factory))
}
async fn input() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(std::fs::read_to_string("assets/input/index.html").unwrap())
}
