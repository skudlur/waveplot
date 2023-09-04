pub mod argument_handler;
pub mod plot_handler;


use std::{
    env, 
    path::Path,
};

// derive PartialEq to apply binary operator == to Arguments
#[derive(PartialEq)]
pub enum Arguments {
    Empty,
    Version,
    Help,
    Path
}

pub fn get_args_type() -> Arguments {
    // Get the arguments passed to the program
    let envs = env::args();
    let mut args = Vec::new();

    for (i, env) in envs.enumerate() {
        // Skip the first argument, which is the program name
        if i == 0 {
            continue;
        } else {
            args.push(env);
        }
    }

    // Returns the type of argument passed
    for arg in args {
        if arg == "-v" || arg == "--version" {
            return Arguments::Version;
        } else if arg == "-h" || arg == "--help" {
            return Arguments::Help;
        } else {
            let path = Path::new(&arg);
            if path.is_file() {
                return Arguments::Path;
            } else {
                return Arguments::Empty;
            }
        }
    }

    // If no arguments are passed, returns empty
    return Arguments::Empty;
}

