use std::{env, io::{self, Write}};

fn main() {
    let mut args = env::args();
    let mut input = String::new();
    args.next();
    let vec_args: Vec<String> = args.collect();
    // 0 for option n, 1 for option e (if index 1 = false, it's the default E option)
    let mut is_options_enabled = (false, false); 

    
    for arg in vec_args {
        let mut word = String::new();
        if arg.contains("-") {
            let mut opts: Vec<&str> = arg.split("-").collect();
            opts.remove(0);
            
            let avail_opts = *opts.get(0).unwrap();

            // in this for loop, 
            for opt in avail_opts.split("").into_iter() {
                match opt {
                    "n" => {
                        println!("do not output the trailing newline");
                        is_options_enabled.0 = true;
                    },
                    "e" => {
                        println!("enable interpretation of backslash escapes");
                        is_options_enabled.1 = true;
                    },
                    _ => continue,
                }
            }
        } else {
            match &is_options_enabled {
                &(true, _) => {
                    if arg.contains("\\n") {
                        word.push_str(&arg.replace("\\n", ""));
                    }
                },
                &(_, true) => {
                    println!("escape character is enabled")
                },
                &(_, _) => print!(""),
            }
            input.push_str(&format!("{} ", &word));
            // println!("{arg}");   
        }
    }

    let output = input.trim_end();
    io::stdout().write(output.as_bytes()).unwrap();
}
