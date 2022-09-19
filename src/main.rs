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

fn is_coord_over_square(total: f32, mouse_coord: f32, level: usize) -> bool {
    total / 2.0 - 145.0 + (100.0 * level as f32) < mouse_coord
        && mouse_coord < total / 2.0 - 145.0 + (100.0 * level as f32) + 90.0
}

fn get_mouse_cell_coords(mouse_position: (f32, f32)) -> Option<(usize, usize)> {
    let (x, y) = mouse_position;

    let xsize: usize;
    let ysize: usize;

    if is_coord_over_square(screen_width(), x, 0) {
        xsize = 0;
    } else if is_coord_over_square(screen_width(), x, 1) {
        xsize = 1;
    } else if is_coord_over_square(screen_width(), x, 2) {
        xsize = 2;
    } else {
        return None;
    }

    if is_coord_over_square(screen_height(), y, 0) {
        ysize = 0;
    } else if is_coord_over_square(screen_height(), y, 1) {
        ysize = 1;
    } else if is_coord_over_square(screen_height(), y, 2) {
        ysize = 2;
    } else {
        return None;
    }

    Some((xsize, ysize))
}

#[macroquad::main("TicTacToe")]
async fn main() {
    let mut game: Game = Game {
        board: [[None; 3]; 3],
        turn: Cell::X,
    };

    loop {
        let mouse_cell_coords_result = get_mouse_cell_coords(mouse_position());
        if is_mouse_button_pressed(MouseButton::Left) {
            match mouse_cell_coords_result {
                Some((x, y)) => match game.board[x][y] {
                    None => {
                        game.board[x][y] = Some(game.turn);
                        game.turn = match game.turn {
                            Cell::X => Cell::O,
                            Cell::O => Cell::X,
                        };
                    },
                    _ => (),
                },
                None => (),
            }
        }

        clear_background(WHITE);
        render_game(&game);
        next_frame().await;
    }
}
