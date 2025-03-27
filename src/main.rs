use std::io::*;

#[derive(PartialEq, Clone)]
enum TileState {
    Empty,
    PlayerX,
    PlayerO,
}
#[derive(PartialEq)]
enum ActivePlayer {
    PlayerX,
    PlayerO,
}

impl Clone for ActivePlayer {
    fn clone(&self) -> ActivePlayer {
        match self {
            ActivePlayer::PlayerX => ActivePlayer::PlayerX,
            ActivePlayer::PlayerO => ActivePlayer::PlayerO,
        }
    }
}

fn main() {
    let player = ActivePlayer::PlayerX;
    let field: [TileState; 9] = [
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
        TileState::Empty,
    ];
    game(field, player)
}
fn input() -> String {
    let mut field = String::new();
    stdin().read_line(&mut field).expect("Error");
    field.trim().to_string()
}
fn input_handler(
    input: String,
    mut board: [TileState; 9],
    mut player: ActivePlayer,
) -> ([TileState; 9], ActivePlayer) {
    let mut change_field = 0;
    match input.as_str() {
        "a1" => change_field = 1,
        "a2" => change_field = 2,
        "a3" => change_field = 3,
        "b1" => change_field = 4,
        "b2" => change_field = 5,
        "b3" => change_field = 6,
        "c1" => change_field = 7,
        "c2" => change_field = 8,
        "c3" => change_field = 9,
        _ => panic!(),
    }
    change_field -= 1;
    if board[change_field] == TileState::PlayerX || board[change_field] == TileState::PlayerO {
        println!("Field is already taken");
        game(board.clone(), player.clone());
    } else if player == ActivePlayer::PlayerX {
        board[change_field] = TileState::PlayerX;
        player = ActivePlayer::PlayerO
    } else if player == ActivePlayer::PlayerO {
        board[change_field] = TileState::PlayerO;
        player = ActivePlayer::PlayerX
    }
    (board, player)
}
fn game(mut board: [TileState; 9], mut player: ActivePlayer) {
    loop {
    let field = input();
    let new_board = input_handler(field, board, player);
    board = new_board.0;
    player = new_board.1;
//    win_detection()
    print_board(board.clone())
}}

fn print_board(field: [TileState; 9]) {
    let mut board = String::new();
    let mut counter = 0;
    for t in field.iter() {
        if counter == 3 {
            board = format!("{board} \n");
            counter = 0;
        }
        if *t == TileState::Empty {
            board = format!("{board} .");
        } else if *t == TileState::PlayerX {
            board = format!("{board} X");
        } else if *t == TileState::PlayerO {
            board = format!("{board} O");
        }
        counter += 1;
    }
    println!("{board}");
}
fn win_detection() {
    todo!()
}
