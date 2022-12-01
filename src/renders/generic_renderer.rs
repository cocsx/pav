use sdl2::pixels::Color;
use crate::core::{direction::Direction, node::Node };

pub trait Renderer {
    fn draw_cell_line(&mut self, node: Node, direction: Direction);
    fn draw_solution_line(&mut self, node: Node, direction: Direction, color: Color);
}
