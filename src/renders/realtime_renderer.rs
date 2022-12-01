
use sdl2::{pixels::Color, video::Window, render::Canvas, rect::Point, VideoSubsystem, Sdl};

use crate::core::{direction::Direction, node::Node};
use crate::renders::generic_renderer;

pub struct RealtimeRenderer {
    canvas: Canvas<Window>,
    cell_size: i32
}

impl RealtimeRenderer {
    pub fn new(title: &str, number_of_cells: i32, cell_size: i32, sdl_context: &Sdl) -> RealtimeRenderer {
        let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

        let width: u32  = (number_of_cells * cell_size).try_into().unwrap();
        let height: u32 = width;

        let window: Window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(49, 49, 49));
        canvas.clear();
        
        return RealtimeRenderer { canvas: (canvas), cell_size: (cell_size) }
    }
}

impl generic_renderer::Renderer for RealtimeRenderer {
    fn draw_cell_line(&mut self, node: Node, direction: Direction) {
        let node = Node::new(node.row(), node.column()) * self.cell_size;

        let start: Point;
        let end: Point;

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        if direction.is_right() {
            start = Point::new(node.column() + self.cell_size, node.row());
            end   = Point::new(node.column() + self.cell_size, node.row() + self.cell_size);
        } else if direction.is_down() {
            start = Point::new(node.column()                 , node.row() + self.cell_size);
            end   = Point::new(node.column() + self.cell_size, node.row() + self.cell_size);
        } else if direction.is_top() {
            start = Point::new(node.column()                 , node.row());
            end   = Point::new(node.column() + self.cell_size, node.row());
        } else {
            start = Point::new(node.column(), node.row());
            end   = Point::new(node.column(), node.row() + self.cell_size);
        }

        let _result = self.canvas.draw_line(
            start, end
        );
    }

    fn draw_solution_line(&mut self, node: Node, direction: Direction, color: Color) {
        let mid_of_node = self.cell_size / 2;
        let node       = Node::new(node.row() * self.cell_size + mid_of_node, node.column() * self.cell_size + mid_of_node);

        let start: Point;
        let end: Point;
        
        self.canvas.set_draw_color(color);
        
        if direction.is_down() {
            start = Point::new(node.column(), node.row());
            end   = Point::new(node.column(), node.row() + self.cell_size);
        } else if direction.is_right() {
            start = Point::new(node.column()                 , node.row());
            end   = Point::new(node.column() + self.cell_size, node.row());
        } else if direction.is_top() {
            start = Point::new(node.column(), node.row());
            end   = Point::new(node.column(), node.row() - self.cell_size);
        } else {
            start = Point::new(node.column()                 , node.row());
            end   = Point::new(node.column() - self.cell_size, node.row());
        } 
        let _result = self.canvas.draw_line(
            start, end
        );
        self.canvas.present();
    }
}
