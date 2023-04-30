use colored::Colorize;

const BOARD_SIZE : usize = 8;

#[derive(PartialEq, Eq)]
pub enum Color {
    BLACK,
    WHITE,
    NONE,
}

#[derive(PartialEq, Eq)]
enum Overlay {
    PATH,
    KILL,
    NONE,
}

#[derive(PartialEq, Eq)]
enum Piece {
    KING,
    QUEEN,
    BISHOP,
    ROOK,
    KNIGHT,
    PAWN,
    EMPTY,
}

struct Cell {
    piece : Piece,
    color : Color,
    first_move : bool,
    overlay : Overlay,
}

impl Cell {
    fn king(color: Color) -> Self {
        Cell { 
            piece : Piece::KING, 
            color : color, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn queen(color: Color) -> Self {
        Cell { 
            piece : Piece::QUEEN, 
            color : color, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn bishop(color: Color) -> Self {
        Cell { 
            piece : Piece::BISHOP, 
            color : color, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn knight(color: Color) -> Self {
        Cell { 
            piece : Piece::KNIGHT, 
            color : color, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn rook(color: Color) -> Self {
        Cell { 
            piece : Piece::ROOK, 
            color : color, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn pawn(color: Color) -> Self {
        Cell { 
            piece : Piece::PAWN, 
            color : color, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn empty() -> Self {
        Cell { 
            piece : Piece::EMPTY, 
            color : Color::NONE, 
            first_move : true,
            overlay : Overlay::NONE,
        }
    }

    fn kill(&mut self) {
        self.piece = Piece::EMPTY;
        self.color = Color::NONE;
    }
}

pub struct Board {
    b : Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        Board { 
            b: vec![vec![Cell::rook(Color::WHITE), Cell::knight(Color::WHITE), Cell::bishop(Color::WHITE), Cell::queen(Color::WHITE), Cell::king(Color::WHITE), Cell::bishop(Color::WHITE), Cell::knight(Color::WHITE), Cell::rook(Color::WHITE)],
                    vec![Cell::pawn(Color::WHITE), Cell::pawn(Color::WHITE),   Cell::pawn(Color::WHITE),   Cell::pawn(Color::WHITE),  Cell::pawn(Color::WHITE), Cell::pawn(Color::WHITE),   Cell::pawn(Color::WHITE),   Cell::pawn(Color::WHITE)],
                    vec![Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty(),             Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty()],
                    vec![Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty(),             Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty()],

                    vec![Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty(),             Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty()],
                    vec![Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty(),             Cell::empty(),            Cell::empty(),              Cell::empty(),              Cell::empty()],
                    vec![Cell::pawn(Color::BLACK), Cell::pawn(Color::BLACK),   Cell::pawn(Color::BLACK),   Cell::pawn(Color::BLACK),  Cell::pawn(Color::BLACK), Cell::pawn(Color::BLACK),   Cell::pawn(Color::BLACK),   Cell::pawn(Color::BLACK)],
                    vec![Cell::rook(Color::BLACK), Cell::knight(Color::BLACK), Cell::bishop(Color::BLACK), Cell::queen(Color::BLACK), Cell::king(Color::BLACK), Cell::bishop(Color::BLACK), Cell::knight(Color::BLACK), Cell::rook(Color::BLACK)],],
        }
    }

    pub fn select(&mut self, x: usize, y: usize) {
        match self.b[x][y].piece
        {
            Piece::PAWN => {
                self.update_cell(x+1,y);
                if self.b[x][y].first_move == true {
                    self.update_cell(x+2,y);
                }
            }
            _ => {}
        }
    }

    pub fn move_piece(&mut self, x: usize, y: usize) {
        
    }

    fn check_cell(&self, x: usize, y: usize) -> bool
    {
        if x < 0 || y < 0 || x >= BOARD_SIZE || y >= BOARD_SIZE {
            return false;
        }
        return true;
    }

    fn update_cell(&mut self, x: usize, y: usize) -> bool
    {
        if self.check_cell(x, y) {
            if self.b[x][y].piece == Piece::EMPTY {
                self.b[x][y].overlay = Overlay::PATH;
                return false;
            } else {
                self.b[x][y].overlay = Overlay::KILL;
                return true;
            }
        }
        return true;
    }

    pub fn print(&self) {
        let mut p: &str;
        println!("----------");
        println!(" abcdefgh");
        for i in 0..BOARD_SIZE {
            print!("{}", BOARD_SIZE-i);
            for j in 0..BOARD_SIZE {
                match self.b[i][j].piece {
                    Piece::KING => p = "K",
                    Piece::QUEEN => p = "Q",
                    Piece::BISHOP => p = "B",
                    Piece::ROOK => p = "R",
                    Piece::KNIGHT => p = "k",
                    Piece::PAWN => p = "P",
                    _ => p = if self.b[i][j].overlay == Overlay::PATH { "." }
                    else { " " },
                }
                let mut c : colored::ColoredString;
                if self.b[i][j].color == Color::WHITE {
                    c = p.white();
                }
                else {
                    c = p.black();
                }
                if self.b[i][j].overlay == Overlay::KILL {
                    c = c.strikethrough();
                }
                print!("{}", c)
            }
            println!("{}", BOARD_SIZE-i);
        }
        println!(" abcdefgh");
    }
}