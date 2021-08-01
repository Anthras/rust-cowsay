use std::env;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "cowsay")]
struct Message {
    /// Message for ascii character to say
    #[clap(short, long)]
    msg: String,

    /// Number of lines for speech bubble
    #[clap(short, long, default_value = "1")]
    number_of_lines: u8,

    /// Ascii character
    #[clap(short, long, default_value = "cow")]
    ascii_character: String,
}


fn main() {
    let cowsay = Message::parse();
    println!("{}", cowsay.msg);




    // collect the arguments as a string from command line
    /* let args: Vec<String> = env::args().collect(); */
    
    // for now lets have the usage be to only accept one string.
    // args[0] is the name of the program and each argument 
    // afterwards is added to the vector list, so that
    // `./rust-cowsay "hello world" this is a test` would appear
    // as ["./rust-cowsay", "hello world", "this", "is", "a", "test"]
    // where:
    // args[0] = "./rust-cowsay"
    // args[1] = "hello world"
    // args[2] = "this"
    // args[3] = "is"
    // args[4] = "a"
    // args[5] = "test"
    // message = args[1].clone();

    // so usage ` ./rust-cowsay "hello world!" `
    /*
    if args.len() > 2 {
        println!("usage:  ./rust-cowsay \"hello world!\"");
    } else {
        println!("{}", &message);
    } */
    
}
