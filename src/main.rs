mod utils;

use utils::{
    argument_handler::argument_handler,
    get_args_type,
    plot_handler::{get_path, plot_handler},
    Arguments,
};

use std::{
    fs,
    fs::File,
    io::BufReader
};

use vcd::{Command, Parser};

fn main() {
    let args_type = get_args_type();

    if args_type == Arguments::Version
        || args_type == Arguments::Empty
        || args_type == Arguments::Help
        || args_type == Arguments::Version
    {
        argument_handler();
    } else {
        let file_path = get_path();
        let file = File::open(file_path).unwrap();

        let mut parser = Parser::new(BufReader::new(file));

        let header = parser.parse_header().unwrap();

        println!("Header: {:?}", header);
    }
}
