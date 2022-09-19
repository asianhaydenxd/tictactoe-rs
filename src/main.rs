use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Cell {
    X,
    O,
}

struct Game { 
    board: [[Option<Cell>; 3]; 3],
    turn: Cell,
}

fn draw_grid_cell(x: usize, y: usize, cell: Option<Cell>) {
    draw_rectangle(
        screen_width() / 2.0 - 145.0 + (100.0 * x as f32), 
        screen_height() / 2.0 - 145.0 + (100.0 * y as f32), 
        90.0, 90.0, 
        match cell {
            None => GRAY,
            Some(Cell::X) => RED,
            Some(Cell::O) => BLUE,
        }
    );
}

fn render_game(game: &Game) {
    draw_rectangle(screen_width() / 2.0 - 25.0, screen_height() / 2.0 - 250.0, 50.0, 50.0, match game.turn {
        Cell::X => RED,
        Cell::O => BLUE,
    });

    for (x, row) in game.board.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            draw_grid_cell(x, y, *cell);
        }
    }
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut game: Game = Game {
        board: [[None; 3]; 3],
        turn: Cell::X,
    };

    loop {
        clear_background(WHITE);
        render_game(&game);
        next_frame().await;
    }
}
