use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::path::Path;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Node {
    x: f64,
    y: f64,
    name: String,
    id: usize,
    #[serde(rename = "neighbor_nodes")]
    nodes: Vec<(usize, f64)>, // Corresponds to "neighbor_nodes" in the JSON
    #[serde(default)]
    dist: f64, // Default to infinity
    #[serde(default)]
    previous: Option<usize>, // Default to None
}

mod file_utils {
    use serde::de::Error;

    use super::{from_reader, Node, Path};
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
                break; // Remaining nodes are unreachable
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
            node.dist = f64::INFINITY; // Set all distances to infinity
            node.previous = None; // No previous node initially
        }
        nodes[start_id].dist = 0.0; // Distance to start node is zero
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

mod web_routes {
    use super::{file_utils, pathfinding, Node};
    use axum::{
        extract::{Json, Multipart},
        http::{Response, StatusCode},
        response::{Html, IntoResponse},
    };
    use std::f64;
    use tokio::{
        fs::{self, File},
        io::AsyncWriteExt,
    };

    pub async fn home_page() -> impl IntoResponse {
        Html(std::fs::read_to_string("assets/home.html").unwrap())
    }

    pub async fn editor() -> impl IntoResponse {
        Html(std::fs::read_to_string("assets/editor.html").unwrap())
    }

    pub async fn image() -> impl IntoResponse {
        std::fs::read("assets/imsa_hallway.jpg")
            .unwrap()
            .into_response()
    }

    pub(crate) async fn css_handler() -> impl IntoResponse {
        let css = fs::read_to_string("assets/home.css").await.unwrap();
        Response::builder()
            .header("Content-Type", "text/css")
            .body(css)
            .unwrap()
    }

    pub async fn save(mut payload: Multipart) -> impl IntoResponse {
        while let Some(field) = payload.next_field().await.unwrap() {
            if field.name() == Some("file") {
                let data = field.bytes().await.unwrap();
                let mut file = File::create("nodes.json").await.unwrap();
                let _ = file.write_all(&data).await;
            }
        }
        (StatusCode::OK, Json(serde_json::json!({"status": 0})))
    }

    pub async fn directions(Json(request): Json<serde_json::Value>) -> impl IntoResponse {
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
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"status": 1})),
            );
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

        // Return the response as a tuple with a status code and the JSON data
        (
            StatusCode::OK,
            Json(serde_json::json!({ "path": path_json })),
        )
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
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(web_routes::home_page))
        .route("/editor", get(web_routes::editor))
        .route("/image", get(web_routes::image))
        .route("/home.css", get(web_routes::css_handler))
        .route("/save", post(web_routes::save))
        .route("/get_directions", post(web_routes::directions))
        .nest_service("/assets", ServeDir::new("assets"));

    Ok(router.into())
}
