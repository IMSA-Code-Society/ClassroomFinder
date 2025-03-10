use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use shuttle_actix_web::ShuttleActixWeb;
use std::path::Path;
mod path;
mod webpages;
use webpages::{
    about, css_handler, directions, editor, home_page, image, input, save, schedule_handle,
};
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

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
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

    Ok(config.into())
}
