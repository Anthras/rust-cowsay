use clap::Clap;

/// Prints stdin in speech bubble for ascii character.
#[derive(Clap, Debug)]
#[clap(name = "\nRust implementation of cowsay:")]
struct Message {
    /// Message for ascii character to say
    #[clap()]
    message: String,

    /// Number of lines for speech bubble
    #[clap(short, long, default_value = "1")]
    lines: u8,

    /// Ascii character
    #[clap(short, long, default_value = "cow")]
    character: String,
}

fn main() {
    let cowsay = Message::parse();
    
    // print the output to stdout
    output(cowsay.message, cowsay.lines, cowsay.character);

}

// 
// consider moving these functions to another file, lib.rs
// 

fn output(message: String, lines: u8, character: String) {
    // prints the message to stdout
    
//
// consolidate the top_border() and bottom_border() functions since they are the same
// function text except for the underscore (top) vs hyphen (bottom)
//
    
    // top speech bubble
    top_border(&message);

    // sides with message
    // this may need to be a separate function to parse the input
    // length and automatically separate on different lines based on length
    println!("< {} >", &message);

    // bottom speech bubble
    bottom_border(&message);

    // character
    display_character(&character);
}

fn top_border(message: &String) {
    // displays the top of the speech bubble
    // based on message length
    let length = message.len();
    let mut top_string = String::from("");
    top_string.push(' ');
    for _i in 0..length + 2 {
        top_string.push('_');
    }
    println!("{}", top_string);
}

fn bottom_border(message: &String) {
    // displays the bottom of the speech bubble
    // based on message length
    let length = message.len();
    let mut bottom_string = String::from("");
    bottom_string.push(' ');
    for _i in 0..length + 2 {
        bottom_string.push('-');
    }
    println!("{}", bottom_string);
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
