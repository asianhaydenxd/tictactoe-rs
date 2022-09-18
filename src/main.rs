use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Cell {
    X,
    O,
}

type Board = [[Option<Cell>; 3]; 3];

fn draw_grid_cell(x: usize, y: usize, cell: Option<Cell>) {
    draw_rectangle(
        screen_width() / 2.0 - 150.0 + (100.0 * x as f32), 
        screen_height() / 2.0 - 150.0 + (100.0 * y as f32), 
        100.0, 100.0, 
        match cell {
            None => LIGHTGRAY,
            Some(Cell::X) => RED,
            Some(Cell::O) => BLUE,
        }
    );
}

fn render_game(board: &Board) {
    for (x, row) in board.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            draw_grid_cell(x, y, *cell);
        }
    }
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut board: Board = [[None; 3]; 3];

    loop {
        clear_background(WHITE);
        render_game(&board);
        next_frame().await;
    }
}
