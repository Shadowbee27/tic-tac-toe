use std::io::*;

#[derive(PartialEq)]
enum TileState {
    Empty,
    PlayerX,
    PlayerO,
}
enum ActivePlayer {
    PlayerX,
    PlayerO,
}

fn main() {
    println!("{}", input());
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
    print_board(field);
}
fn input() -> String {
    let mut field = String::new();
    stdin().read_line(&mut field).expect("Error");
    field.trim().to_string()
}
fn input_handeler(input: String, board: [TileState; 9]) -> [TileState; 9] {
    let mut change_field = 0;
    match input {
        String::from("a1") => change_field = 1,
        String::from("a2") => change_field = 2,
        String::from("a3") => change_field = 3,
        String::from("b1") => change_field = 4,
        String::from("b2") => change_field = 5,
        String::from("b3") => change_field = 6,
        String::from("c1") => change_field = 7,
        String::from("c2") => change_field = 8,
        String::from("c3") => change_field = 9,
        _ => panic!(),
    }
    board
}
fn game(board: [TileState; 9]) {
    todo!()
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
        counter = counter + 1;
    }
    println!("{board}");
}
