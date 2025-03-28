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
    let player_x: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let player_o: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
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
    game(field, player, player_x, player_o)
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
    mut player_o: [i8; 8],
    mut player_x: [i8; 8],
) -> ([TileState; 9], ActivePlayer, [i8; 8], [i8; 8]) {
    let mut win = Vec::new();
    let mut change_field;
    match input.as_str() {
        "a1" => {
            change_field = 1;
            win.push(0);
            win.push(3);
            win.push(6);
        }
        "a2" => {
            change_field = 2;
            win.push(1);
            win.push(3);
        }
        "a3" => {
            change_field = 3;
            win.push(2);
            win.push(3);
            win.push(7);
        }
        "b1" => {
            change_field = 4;
            win.push(0);
            win.push(4);
        }
        "b2" => {
            change_field = 5;
            win.push(1);
            win.push(4);
            win.push(6);
            win.push(7);
        }
        "b3" => {
            change_field = 6;
            win.push(2);
            win.push(4);
        }
        "c1" => {
            change_field = 7;
            win.push(0);
            win.push(5);
            win.push(7);
        }
        "c2" => {
            change_field = 8;
            win.push(1);
            win.push(5);
        }
        "c3" => {
            change_field = 9;
            win.push(2);
            win.push(5);
            win.push(6);
        }
        _ => {
            println!("field doesn't exist");
            change_field = 0;
            game(board.clone(), player.clone(), player_o, player_x);
        }
    }
    change_field -= 1;
    if board[change_field] == TileState::PlayerX || board[change_field] == TileState::PlayerO {
        println!("Field is already taken");
        game(board.clone(), player.clone(), player_o, player_x);
    } else if player == ActivePlayer::PlayerX {
        board[change_field] = TileState::PlayerX;
        player_x = win_detection_handler(win, player_x);
        player = ActivePlayer::PlayerO
    } else if player == ActivePlayer::PlayerO {
        board[change_field] = TileState::PlayerO;
        player_o = win_detection_handler(win, player_o);
        player = ActivePlayer::PlayerX
    }
    (board, player, player_x, player_o)
}
fn game(
    mut board: [TileState; 9],
    mut player: ActivePlayer,
    mut player_o: [i8; 8],
    mut player_x: [i8; 8],
) {
    loop {
        let field = input();
        let new_board = input_handler(field, board, player, player_o, player_x);
        board = new_board.0;
        player = new_board.1;
        player_x = new_board.2;
        player_o = new_board.3;
        win_detection(player_o, player_x, board.clone());
        print_board(board.clone())
    }
}

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

fn win_detection_handler(input_fields: Vec<i32>, mut player: [i8; 8]) -> [i8; 8] {
    for p in input_fields {
        player[p as usize] += 1
    }
    println!("player= {:?}", player);
    player
}
fn win_detection(player_x: [i8; 8], player_o: [i8; 8], board: [TileState; 9]) {
    for i in player_x {
        println!("i = {i}");
        if i == 3 {
            println!("Player X won");
            print_board(board.clone());
            main()
        }
    }
    for x in player_o {
        if x == 3 {
            println!("Player X won");
            print_board(board.clone());
            main()
        }
    }
}
