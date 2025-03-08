use std::collections::HashSet;

use rand::seq::IteratorRandom;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Maze {
    pub width: u16,
    pub height: u16,
    pub start_node: u32,
    pub node_count: u32,
    pub edges: HashSet<(u32, u32)>,
}

impl Maze {
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
        println!("start node: {current_node}");
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
        let row = node / width as u32;
        let offset = width as u32 * row;

        // left
        if node != offset {
            let left = (node, node - 1);
            if !edges.contains(&left) && !connected_nodes.contains(&left.1) {
                possible_edges.insert(left);
            }
        }

        // right
        if node == 0 || (offset > 0 && node != offset - 1) {
            let right = (node, node + 1);
            if !edges.contains(&right)
                && !connected_nodes.contains(&right.1)
                && right.1 != (width * height) as u32
            {
                possible_edges.insert(right);
            }
        }

        // up
        if node > width as u32 {
            let up = (node, node - width as u32);
            if !edges.contains(&up) && !connected_nodes.contains(&up.1) {
                possible_edges.insert(up);
            }
        }

        // down
        if node < (width * (height - 1)) as u32 {
            let down = (node, node + width as u32);
            if !edges.contains(&down) && !connected_nodes.contains(&down.1) {
                possible_edges.insert(down);
            }
        }

        return possible_edges;
    }

    pub fn node_to_grid_coords(node: u32, width: u32) -> (u32, u32) {
        let y = node / width;
        let x = node - (y * width);
        (x, y)
    }

    pub fn node_to_scaled_coords(node: u32, scale: u8, width: u32) -> (u32, u32) {
        let (x, y) = Self::node_to_grid_coords(node, width);
        (
            x.saturating_mul(scale as u32),
            y.saturating_mul(scale as u32),
        )
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
    // #[test]
    // fn node_id_to_grid_coords() {
    //     let maze = Maze::generate(3, 3);
    //     let coords1 = maze.node_to_grid_coords(2);
    //     assert_eq!(coords1, (2, 0));

    //     let coords2 = maze.node_to_grid_coords(4);
    //     assert_eq!(coords2, (1, 1));

    //     let coords3 = maze.node_to_grid_coords(6);
    //     assert_eq!(coords3, (0, 2));

    //     let coords4 = maze.node_to_grid_coords(7);
    //     assert_eq!(coords4, (1, 2));
    // }

    #[test]
    fn maze_gen_preview() {
        let maze = Maze::generate(3, 3);
        println!("maze: {:?}", maze);
    }
}
