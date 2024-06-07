use std::env;

fn main() {
    let mut args = env::args();

    args.next();
    
    for arg in args {
        if arg.contains("-") {
            let mut opts: Vec<&str> = arg.split("-").collect();
            opts.remove(0);
            
            let avail_opts = *opts.get(0).unwrap();

            for opt in avail_opts.split("").into_iter() {
                match opt {
                    "n" => println!("do not output the trailing newline"),
                    "e" => println!("enable interpretation of backslash escapes"),
                    "E" => println!("disable interpretation of backslash escapes (default)"),
                    _ => continue,
                }
            }
        } else {
            println!("{arg}");   
        }
    }
}
