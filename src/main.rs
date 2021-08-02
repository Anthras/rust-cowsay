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
    
    output(cowsay.message, cowsay.lines, cowsay.character);

}

fn output(message: String, lines: u8, character: String) {
    // top speech bubble
    top_border(&message);

    // sides with message
    println!("< {} >", &message);

    // bottom speech bubble
    bottom_border(&message);

    // character
    display_character(&character);
}

fn top_border(message: &String) {
    let length = message.len();
    let mut top_string = String::from("");
    top_string.push(' ');
    for _i in 0..length + 2 {
        top_string.push('_');
    }
    println!("{}", top_string);
}

fn bottom_border(message: &String) {
    let length = message.len();
    let mut top_string = String::from("");
    top_string.push(' ');
    for _i in 0..length + 2 {
        top_string.push('-');
    }
    println!("{}", top_string);
}

fn display_character(character: &String) {
    if character == "cow" {
        println!("       \\   ^__^                 ");
        println!("        \\  (oo)\\_______         ");
        println!("           (__)\\       )\\/\\    ");
        println!("                ||----w |        ");
        println!("                ||     ||        ");
    }
}