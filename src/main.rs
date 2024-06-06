use std::env;

fn main() {
    let mut args = env::args();

    args.next();
    
    let args_1 = args.next().unwrap();
    println!("{args_1}");
}
