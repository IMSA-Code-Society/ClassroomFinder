use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use log::error;
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
    use std::fs::File;

    pub fn read_nodes_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Node>, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        Ok(from_reader(file).map_err(|e| e.to_string())?)
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

fn name_to_id(name: &str, nodes: &[Node]) -> Result<usize, String> {
    nodes
        .iter()
        .find(|node| node.name.to_lowercase() == name.to_lowercase())
        .map(|node| node.id)
        .ok_or_else(|| {
            error!("Could not identify a node for string '{name}'");
            format!("Could not identify a node for string '{name}'").to_string()
        })
}
fn name_to_ids<'a>(name: &str, nodes: &'a [Node]) -> Vec<&'a Node> {
    let res = nodes
        .iter()
        .filter(|node| node.name.to_lowercase() == name.to_lowercase())
        .collect();

    res
}

fn closest_pair_between(
    start_name: &str,
    end_name: &str,
    nodes: &[Node],
) -> Option<(usize, usize)> {
    let starts = name_to_ids(start_name, nodes);
    let ends = name_to_ids(end_name, nodes);

    starts
        .iter()
        .flat_map(|&s| {
            ends.iter().map(move |&e| {
                let dx = s.x - e.x;
                let dy = s.y - e.y;
                let dist_sq = dx * dx + dy * dy;
                (dist_sq, s.id, e.id)
            })
        })
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .map(|(_, sid, eid)| (sid, eid))
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
