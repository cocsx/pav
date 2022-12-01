
use std::thread;
use std::time::Duration;
use std::collections::{HashMap, HashSet};

use sdl2::pixels::Color;

use crate::core::{maze::Maze, node::Node};

use crate::renders::generic_renderer::Renderer;

pub trait SolvingAlgorithm {
    fn draw<T: Renderer>(&mut self, renderer: &mut T, start: Node, end: Node);
    fn solve(&mut self, maze: &Maze, start: Node, end: Node) -> &mut Self;
}

pub struct Dfs {
    solution: HashMap<Node, Node>
}

impl Dfs {
    pub fn new() -> Self {
        Self { solution: (HashMap::new()) }
    }
}

impl SolvingAlgorithm for Dfs {
    fn draw<T: Renderer>(&mut self, renderer: &mut T, start: Node, end: Node) {
        let mut curr = start;
        while curr != end {
            let choosen = self.solution.get_key_value(&curr).expect("[ERROR] No node avaliable").1;
            let dir = Node::get_dir(&curr, choosen);
            renderer.draw_solution_line(curr, dir, Color::RGB(255, 0, 0));
            curr = *choosen;
            thread::sleep(Duration::from_millis(30));
        }
    }

    fn solve(&mut self, maze: &Maze, start: Node, end: Node) -> &mut Self {
        let mut visited: HashSet<Node> = HashSet::new();
        let mut stack: Vec<Node> = Vec::new(); 

        visited.insert(end);
        stack.push(end);

        while !stack.is_empty() {
            let current: Node = stack.pop().expect("No node avaliable");
            let dirs = maze.graph().get(&current).expect("No dirs avaliable");
            for dir in dirs {
                let neigh: Node = current + *dir;
                if !visited.contains(&neigh) {
                    self.solution.insert(neigh, current);

                    visited.insert(neigh);
                    stack.push(neigh);
                }
            }
            if current == start {
                break;
            }
        }
        self
    }

}

pub struct Bfs {
    solution: HashMap<Node, Node>
}

impl Bfs {
    pub fn new() -> Self {
        Self { solution: (HashMap::new()) }
    }
}

impl SolvingAlgorithm for Bfs {
    fn draw<T: Renderer>(&mut self, renderer: &mut T, start: Node, end: Node) {
        let mut curr = start;
        while curr != end {
            let choosen = self.solution.get_key_value(&curr).expect("[ERROR] No node avaliable").1;
            let dir = Node::get_dir(&curr, choosen);
            renderer.draw_solution_line(curr, dir, Color::RGB(0, 255, 0));
            curr = *choosen;
            thread::sleep(Duration::from_millis(30));
        }
    }

    fn solve(&mut self, maze: &Maze, start: Node, end: Node) -> &mut Self {
        let mut visited: HashSet<Node> = HashSet::new();
        let mut frontier: Vec<Node> = Vec::new();  

        frontier.push(end);
       
        while !frontier.is_empty() {
            let current: Node = frontier.pop().expect("No node avaliable");
            visited.insert(current);

            let dirs = maze.graph().get(&current).expect("No dirs avaliable");
            for dir in dirs {
                let neigh: Node = current + *dir;
                if !visited.contains(&neigh) {
                    self.solution.insert(neigh, current);
                    frontier.push(neigh);
                }
            }
            if current == start {
                break;
            }
        }
        self
    }
}
