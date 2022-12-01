use std::collections::{HashMap, HashSet};
use rand::{thread_rng, Rng};

use super::node::Node;
use super::direction::Direction;
use crate::renders::generic_renderer::Renderer;

pub struct Maze {
    rows: i32,
    columns: i32,
    graph: HashMap<Node, HashSet<Direction>>,
}

impl Maze {
    pub fn new(rows: i32, columns: i32) -> Maze {
        Maze { rows: (rows), columns: (columns), graph: (HashMap::new())}.populate()
    }

    pub fn carve<T: Renderer>(&mut self, render: &mut T) {
        let first_node: Node = Node::new(self.rows / 2, self.columns / 2);
        let mut stack: Vec<Node> = Vec::new();
        let mut visited: HashSet<Node> = HashSet::new();

        stack.push(first_node);
        visited.insert(first_node);

        while !stack.is_empty() {
            let current: Option<Node> = stack.pop();

            match current {
                Some(curr) => {
                    let neighbours = self.neighbours(curr, &visited);
                    if !neighbours.is_empty() {
                        stack.push(curr);
                        let chosen: Node = neighbours[thread_rng().gen_range(0..neighbours.len())];
                        self.add_path(curr, chosen);
                        visited.insert(chosen);
                        stack.push(chosen);
                    } else {
                        let dirs = self.graph().get(&curr).expect("[FATAL] Not a good node");
                        for ind in 0..4 {
                            if !dirs.contains(&Direction::index(ind)) {
                                render.draw_cell_line(
                                    curr,
                                    Direction::index(ind)
                                );
                            }
                        }
                    }
                }
                None => ()
            }
        }
    }

    pub fn graph(&self) -> &HashMap<Node, HashSet<Direction>> {
        return &self.graph
    }

    fn neighbours(&self, node: Node, visited: &HashSet<Node>) -> Vec<Node> {
        let mut neighbours: Vec<Node> = Vec::new();
        for i in 0..4 {
            let (check, neigh) = self.check_neighbour(node, Direction::index(i));
            if check && !visited.contains(&neigh) {
                neighbours.push(neigh);
            }
        }
        return neighbours;
    }

    fn populate(mut self) -> Maze {
        for row in 0..self.rows {
            for col in 0..self.columns {
                self.graph.insert(Node::new(row, col), HashSet::new());
            }
        }
        return self;
    }

    fn check_neighbour(&self, node: Node, dir: Direction) -> (bool, Node) {
        let neigh: Node = node + dir;
        if neigh.row() >= 0 && neigh.row() < self.rows && neigh.column() >= 0 && neigh.column() < self.columns {
            return (true, neigh);
        }
        return (false, Node::new(0, 0));
    }

    fn add_path(&mut self, node: Node, neigh: Node) {
        let dir: Direction = Node::get_dir(&node, &neigh);
        let dirs = self.graph.get_mut(&node);

        match dirs {
            Some(d) => {
                d.insert(dir);
            }
            None => ()
        }

        let dirs = self.graph.get_mut(&neigh);
        match dirs {
            Some(d) => {
                d.insert(-dir);
            }
            None => ()
        }
    }
}
