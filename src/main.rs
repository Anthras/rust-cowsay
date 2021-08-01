use std::env;

fn main() {
    // collect the arguments as a string from command line
    let args: Vec<String> = env::args().collect();
    
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
    let message = args[1].clone();

    // so usage ` ./rust-cowsay "hello world!" `
    if args.len() > 2 {
        println!("usage:  ./rust-cowsay \"hello world!\"");
    } else {
        println!("{}", &message);
    }

    
}
