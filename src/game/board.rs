enum Color {
    White,
    Black,
}

pub trait Piece {
    fn move(&self);
}

pub struct King { color: Color, }

impl King {
    pub fn new(color: Color) -> Self {
        King { 
            color: color.into()
        }
    }
}

impl Piece for King {
    fn move(&self) {
        println!("Default implementation for PrintsUpdate...");
    }
}


pub struct Board {
    pieces: [[&Piece; 8]; 8],
}

impl Board {
    fn init(&mut self) {
        for i in &[(0,0), (0,7)] {
            self.pieces[i.0][i.1] = Null;
        }
    }

    fn print(&self) {
        
    }
}