use eframe::egui;
use egui::{Color32, Rect, Stroke, Vec2};
#[derive(PartialEq, Clone)]
enum ActivePlayer {
    PlayerX,
    PlayerO,
}
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic Tac Toe game",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Custom buttons with different outlined sides:");

            let button_size = Vec2::new(180.0, 40.0);
            let spacing = 10.0;

            // Top only
            if draw_custom_button(ui, "Top Border", button_size, BorderSides::top()).clicked() {
                self.counter += 1;
            }

            ui.add_space(spacing);

            // Left + Right
            if draw_custom_button(ui, "Left & Right", button_size, BorderSides::left().with_right()).clicked() {
                self.counter += 1;
            }

            ui.add_space(spacing);

            // All sides
            if draw_custom_button(ui, "All Sides", button_size, BorderSides::all()).clicked() {
                self.counter += 1;
            }

            ui.add_space(spacing);

            // Top + Bottom only
            if draw_custom_button(ui, "Top & Bottom", button_size, BorderSides::top().with_bottom()).clicked() {
                self.counter += 1;
            }

            ui.add_space(spacing * 2.0);
            ui.label(format!("Counter: {}", self.counter));
        });
    }
}

/// Describes which sides of a button should have borders.
#[derive(Default)]
struct BorderSides {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl BorderSides {
    fn top() -> Self {
        Self { top: true, ..Default::default() }
    }
    fn bottom() -> Self {
        Self { bottom: true, ..Default::default() }
    }
    fn left() -> Self {
        Self { left: true, ..Default::default() }
    }
    fn right() -> Self {
        Self { right: true, ..Default::default() }
    }
    fn all() -> Self {
        Self { top: true, bottom: true, left: true, right: true }
    }

    fn with_top(mut self) -> Self {
        self.top = true;
        self
    }
    fn with_bottom(mut self) -> Self {
        self.bottom = true;
        self
    }
    fn with_left(mut self) -> Self {
        self.left = true;
        self
    }
    fn with_right(mut self) -> Self {
        self.right = true;
        self
    }
}

fn draw_custom_button(
    ui: &mut egui::Ui,
    text: &str,
    size: Vec2,
    borders: BorderSides,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let painter = ui.painter();

    let stroke = Stroke::new(2.0, Color32::LIGHT_BLUE);

    if borders.top {
        painter.line_segment([rect.left_top(), rect.right_top()], stroke);
    }
    if borders.bottom {
        painter.line_segment([rect.left_bottom(), rect.right_bottom()], stroke);
    }
    if borders.left {
        painter.line_segment([rect.left_top(), rect.left_bottom()], stroke);
    }
    if borders.right {
        painter.line_segment([rect.right_top(), rect.right_bottom()], stroke);
    }

    // Optional: hover background
    if response.hovered() {
        painter.rect_filled(rect, 0.0, Color32::from_gray(30));
    }

    // Draw centered text
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(16.0),
        Color32::WHITE,
    );

    response
}

/*
#[derive(PartialEq, Clone)]
enum TileState {
    Empty,
    PlayerX,
    PlayerO,
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
    player
}
fn win_detection(player_x: [i8; 8], player_o: [i8; 8], board: [TileState; 9]) {
    for i in player_x {
        if i == 3 {
            println!("Player X won");
            print_board(board.clone());
            main()
        }
    }
    for x in player_o {
        if x == 3 {
            println!("Player O won");
            print_board(board.clone());
            main()
        }
    }
}
*/
