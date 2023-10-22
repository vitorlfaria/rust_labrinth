use crate::levels::wall_tile::WallTile;

pub struct Pathfind;

impl Pathfind {
    pub fn new() -> Self {
        Self
    }

    pub fn find_path_to(&self, initial_coords: (usize, usize), target_coords: (usize, usize), level_walls: &Vec<WallTile>) -> Vec<(usize, usize, usize)> {
        let mut best_path: Vec<(usize, usize, usize)> = Vec::new();
        let mut main_nodes: Vec<(usize, usize, usize)> = vec![(target_coords.0, target_coords.1, 0)];
        let mut visited_nodes: Vec<(usize, usize, usize)> = Vec::new();
        let mut nodes_to_check: Vec<(usize, usize, usize)> = vec![(target_coords.0, target_coords.1, 0)];

        // Search for the target node (self node)
        let mut counter = 1;
        'searchloop: loop {
            for node in nodes_to_check.iter_mut() {
                if visited_nodes.contains(node) {
                    continue;
                }
                visited_nodes.push(node.clone());

                let search_nodes = vec![
                    (node.0 + 1, node.1, node.2 + 1),
                    (node.0 - 1, node.1, node.2 + 1),
                    (node.0, node.1 + 1, node.2 + 1),
                    (node.0, node.1 - 1, node.2 + 1),
                ];

                for search_node in search_nodes.iter() {
                    if search_node.0 == initial_coords.0 && search_node.1 == initial_coords.1 {
                        best_path.push((node.0, node.1, node.2));
                        break 'searchloop;
                    }

                    if level_walls.iter().any(|wall| wall.x == search_node.0 && wall.y == search_node.1) {
                        continue;
                    }

                    if visited_nodes.contains(search_node) {
                        continue;
                    }

                    main_nodes.push(search_node.clone());
                }
            }

            nodes_to_check = Vec::new();
            for node in main_nodes.iter() {
                if node.2 == counter {
                    nodes_to_check.push(node.clone());
                }
            }
            counter += 1;
        }

        // Backtrack to find the best path
        main_nodes.reverse();
        let mut current_node = best_path[0];
        if current_node.2 == 0 {
            return vec![(initial_coords.0, initial_coords.1, 0)];
        }
        let mut search_counter = current_node.2 - 1;
        while search_counter > 0 {
            let search_nodes = vec![
                (current_node.0 + 1, current_node.1, current_node.2 - 1),
                (current_node.0 - 1, current_node.1, current_node.2 - 1),
                (current_node.0, current_node.1 + 1, current_node.2 - 1),
                (current_node.0, current_node.1 - 1, current_node.2 - 1),
            ];
            let nodes = main_nodes.iter().filter(|node| node.2 == search_counter && search_nodes.contains(node)).collect::<Vec<_>>();
            best_path.push(nodes[0].clone());
            current_node = nodes[0].clone();
            search_counter = current_node.2 - 1;
        }

        best_path
    }    
}