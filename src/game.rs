pub mod board;

pub fn run() {
    let cmds = vec![
        ("select", 1, 0)
    ];
    let mut board = board::Board::new();
    let mut player: board::Color = board::Color::WHITE;
    let mut i = 0;
    /*'change_player: loop {
        player = if player == board::Color::WHITE {
            board::Color::BLACK
        } else {
            board::Color::WHITE
        };
        'play: loop {

        }
        i += 1;
    }*/
    for cmd in cmds {
        board.print();
        match cmd {
            ("select", _, _) => board.select(cmd.1, cmd.2),
            _ => continue,
        }
    }
    board.print();
}