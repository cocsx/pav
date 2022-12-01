mod core;
mod renders;
mod algorithms;

use sdl2::{Sdl, event::Event, keyboard::Keycode};

use crate::algorithms::solving_algorithm::{SolvingAlgorithm, Dfs, Bfs};
use crate::core::{maze::Maze, node::Node};
use crate::renders::realtime_renderer::RealtimeRenderer;

fn main() {
    let num_of_cells: i32 = 40;
    let cell_size: i32 = 20;
    
    let sdl_context: Sdl = sdl2::init().unwrap();
    let mut renderer = RealtimeRenderer::new("PAV", num_of_cells, cell_size, &sdl_context);

    let mut maze: Maze = Maze::new(num_of_cells, num_of_cells);
    maze.carve(&mut renderer);

    let mut start = Node::new(0, 0);
    let mut end = Node::new(num_of_cells - 1, num_of_cells - 1);

    let mut solving_algorithm = Dfs::new();
    solving_algorithm.solve(&maze, start, end)
        .draw(&mut renderer, start, end);

    start = Node::new(0, num_of_cells - 1);
    end = Node::new(num_of_cells - 1, 0);
    let mut solving_algorithm2 = Bfs::new();
    solving_algorithm2.solve(&maze, start, end)
        .draw(&mut renderer, start, end);


    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
