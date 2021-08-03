use clap::Clap;

/// Prints stdin in speech bubble for ascii character.
#[derive(Clap, Debug)]
#[clap(name = "\nRust implementation of cowsay:")]
struct Message {
    /// Message for ascii character to say
    #[clap()]
    message: String,

    /// Top of the bubble character
    #[clap(short, long, default_value = "_")]
    top_bubble_character: char,

    /// Bottom of the bubble character
    #[clap(short, long, default_value = "-")]
    bottom_bubble_character: char,

    /// Number of lines for speech bubble
    #[clap(short, long, default_value = "1")]
    lines: u8,

    /// Ascii character
    #[clap(short, long, default_value = "cow")]
    character: String,
}

fn main() {
    let rust_cowsay = Message::parse();

    display_border(&rust_cowsay.message, &rust_cowsay.top_bubble_character);
    println!("< {} >", &rust_cowsay.message);
    display_border(&rust_cowsay.message, &rust_cowsay.bottom_bubble_character);
    display_character(&rust_cowsay.character);

}

// 
// consider moving these functions to another file, lib.rs
// 

fn display_border(message: &String, character: &char) {
    // displays part of the speech bubble
    // based on message length
    let length = message.len();
    let mut border_string = String::from(" ");
    for _i in 0..length + 2 {
        border_string.push(*character);
    }
    println!("{}", border_string);
}

// 
// consider making these enum type to accept different inputs,
// like "cow", "clip", "crab", "man"
// and other to make a 404 character not found
//

fn display_character(character: &String) {
    // displays characters based on the character flag
    
    if character == "cow" {
        println!("       \\   ^__^                 ");
        println!("        \\  (oo)\\_______         ");
        println!("           (__)\\       )\\/\\    ");
        println!("                ||----w |        ");
        println!("                ||     ||        ");
    }
    
}
