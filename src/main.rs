// Imports
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



fn input() -> (std::string::String, Colour) {
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

    println!("The user choose color: {:?}", color_matched);
    //return the tuple
    (args.quote, color_matched)
}

fn main() {
    input();
}
                                                             