// Imports
use std::thread;
use std::time::Duration;

use std::string::String as Add_str;
use ansi_term::{self, Colour};
use clap::{ValueEnum, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about = "mccart rusty version")]
struct Args {
    #[clap(short, long)]    // Quote to say
    quote: String,

    #[clap(value_enum)]   //Colour to choose
    color: Colors,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
//using enum to create the type "Colors"
enum Colors {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

// Handling inputs using matcch statement



fn input() -> (Add_str, Colour) {
    let args = Args::parse();
    // args.quote -> to access the quote
    // args.color -> to access the color
    let color_matched = match args.color {
        Colors::Red => Colour::Red,
        Colors::Green => Colour::Green,
        Colors::Yellow => Colour::Yellow,
        Colors::Blue => Colour::Blue,
        Colors::Purple => Colour::Purple,
        Colors::Cyan => Colour::Cyan,
        Colors::White => Colour::White,
    };

    // println!("Quote: {}", args.quote);
    println!("The user choose color: {:?}", color_matched);
    //return the tuple
    (args.quote, color_matched)
}

//Drawing a Ferris
fn draw(quote: &str, color: &Colour) {
    const FERRIS: &'static str = r"
    .
     .
      .
       █ █           █ █
        ▀█  ▄█████▄  █▀
         ▀▄███▀█▀███▄▀ 
         ▄▀███▀▀▀███▀▄ 
         █ ▄▀▀▀▀▀▀▀▄ █
    ";
    println!("{}", format!("\"{}\"{}", quote, color.paint(FERRIS)));
}

fn main() {
    // input();
    let (q, c) = input();
    thread::sleep(Duration::from_millis(2000));
    draw(&q, &c)
}
                                                             