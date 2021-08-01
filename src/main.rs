use std::env;

fn main() {
    // collect the arguments as a string from command line
    let args: Vec<String> = env::args().collect();
    
    // for now lets have the usage be to only accept one string
    let message = &args[1];

    // so usage ` ./rust-cowsay "hello world!" `
    if args.len() > 2 {
        println!("usage:  ./rust-cowsay \"hello world!\"");
    } else {
        println!("{}", &message);
    }

    
}
