use std::collections::HashSet;

use rand::seq::IteratorRandom;

/// A maze describes a graph of nodes and their interconnections.
/// This is used to connect rooms.
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Maze {
    pub width: u16,
    pub height: u16,
    pub start_node: u32,
    pub node_count: u32,
    pub edges: HashSet<(u32, u32)>,
}

impl Maze {
    #[allow(unused)]
    pub fn generate(width: u16, height: u16) -> Maze {
        let mut connected_nodes = HashSet::<u32>::new();
        let mut unconnected_nodes = HashSet::<u32>::new();
        let total: u32 = u32::from(width) * u32::from(height);
        for i in 0..total {
            unconnected_nodes.insert(i as u32);
        }

        let mut rng = rand::rng();
        let mut nodes_with_only_one_connection: Vec<u32> = vec![];
        let mut edges: HashSet<(u32, u32)> = HashSet::new();

        // randomly pick a starting node in the first row
        let start_node = rand::random_range(0..width) as u32;
        let mut current_node = start_node.clone();

        while unconnected_nodes.len() > 0 {
            unconnected_nodes.remove(&current_node);
            let candidate_edges =
                Self::get_possible_edges(current_node, &connected_nodes, &edges, width, height);
            match candidate_edges.iter().choose(&mut rng) {
                Some(edge) => {
                    edges.insert(*edge);
                    unconnected_nodes.remove(&current_node);
                    if !connected_nodes.contains(&current_node) {
                        nodes_with_only_one_connection.push(current_node);
                        connected_nodes.insert(current_node);
                    }
                    current_node = edge.1;
                }
                None => {
                    // dead end has been reached
                    // grab last node with only one connection
                    match nodes_with_only_one_connection.pop() {
                        Some(last_node) => current_node = last_node,
                        None => break,
                    }
                }
            }
        }

        return Maze {
            width,
            height,
            edges,
            start_node,
            node_count: total,
        };
    }

    // Given a node ID, get the possible edges that it can connect to
    pub(self) fn get_possible_edges(
        node: u32,
        connected_nodes: &HashSet<u32>,
        edges: &HashSet<(u32, u32)>,
        width: u16,
        height: u16,
    ) -> HashSet<(u32, u32)> {
        let mut possible_edges: HashSet<(u32, u32)> = HashSet::new();
        let y = node / width as u32;
        let x = node - (y * width as u32);
        let max_width = (width - 1) as u32;
        let max_height = (height - 1) as u32;

        // left
        if x > 0 {
            let left = (node, node - 1);
            if !edges.contains(&left) && !connected_nodes.contains(&left.1) {
                possible_edges.insert(left);
            }
        }

        // right
        if x < max_width {
            let right = (node, node + 1);
            if !edges.contains(&right)
                && !connected_nodes.contains(&right.1)
                && right.1 != (width * height) as u32
            {
                possible_edges.insert(right);
            }
        }

        // up
        if y > 0 {
            let up = (node, node - width as u32);
            if !edges.contains(&up) && !connected_nodes.contains(&up.1) {
                possible_edges.insert(up);
            }
        }

        // down
        if y < max_height {
            let down = (node, node + width as u32);
            if !edges.contains(&down) && !connected_nodes.contains(&down.1) {
                possible_edges.insert(down);
            }
        }

        return possible_edges;
    }

    #[allow(unused)]
    pub fn node_to_grid_coords(&self, node: u32) -> (u32, u32) {
        let y = node / self.width as u32;
        let x = node - (y * self.width as u32);
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_possible_edges_top_left() {
        let edges = Maze::get_possible_edges(0, &HashSet::new(), &HashSet::new(), 3, 3);
        assert!(edges.contains(&(0, 1)));
        assert!(edges.contains(&(0, 3)));
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn get_possible_edges_top_right() {
        let edges = Maze::get_possible_edges(2, &HashSet::new(), &HashSet::new(), 3, 3);
        assert!(edges.contains(&(2, 1)));
        assert!(edges.contains(&(2, 5)));
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn get_possible_edges_mid_right() {
        let edges = Maze::get_possible_edges(5, &HashSet::new(), &HashSet::new(), 3, 3);
        assert!(edges.contains(&(5, 2)));
        assert!(edges.contains(&(5, 4)));
        assert!(edges.contains(&(5, 8)));
        assert_eq!(edges.len(), 3);
    }

    #[test]
    fn get_possible_edges_center() {
        let edges = Maze::get_possible_edges(4, &HashSet::new(), &HashSet::new(), 3, 3);
        assert!(edges.contains(&(4, 3)));
        assert!(edges.contains(&(4, 5)));
        assert!(edges.contains(&(4, 1)));
        assert!(edges.contains(&(4, 7)));
        assert_eq!(edges.len(), 4);
    }

    #[test]
    fn get_possible_edges_bottom_left() {
        let edges = Maze::get_possible_edges(6, &HashSet::new(), &HashSet::new(), 3, 3);
        assert!(edges.contains(&(6, 3)));
        assert!(edges.contains(&(6, 7)));
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn get_possible_edges_bottom_right() {
        let edges = Maze::get_possible_edges(8, &HashSet::new(), &HashSet::new(), 3, 3);
        assert!(edges.contains(&(8, 7)));
        assert!(edges.contains(&(8, 5)));
        assert_eq!(edges.len(), 2);
    }

    // TODO: fix
    #[test]
    fn node_id_to_grid_coords() {
        let coords1 = Maze::node_to_grid_coords(2, 3);
        assert_eq!(coords1, (2, 0));

        let coords2 = Maze::node_to_grid_coords(4, 3);
        assert_eq!(coords2, (1, 1));

        let coords3 = Maze::node_to_grid_coords(6, 3);
        assert_eq!(coords3, (0, 2));

        let coords4 = Maze::node_to_grid_coords(7, 3);
        assert_eq!(coords4, (1, 2));
    }

    #[test]
    fn maze_gen_preview() {
        let maze = Maze::generate(3, 3);
        println!("maze: {:?}", maze);
    }
}
