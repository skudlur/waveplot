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
    io::BufReader, thread::Scope, vec
};

use vcd::{Command, Parser, ScopeItem};

fn main() {
    let args_type = get_args_type();

    if args_type == Arguments::Version
        || args_type == Arguments::Empty
        || args_type == Arguments::Help
        || args_type == Arguments::Version
    {
        argument_handler();
    } else {
        plot_handler();
        // let file_path = get_path();
        // let file = File::open(file_path).unwrap();

        // let mut variable_types = Vec::new();
        // let mut variable_sizes = Vec::new();
        // let mut variable_references = Vec::new();
        // // for temporarily holding variable indexes
        // let mut variable_indexes_ref = Vec::new();
        // let mut variable_indexes = Vec::new();


        // let mut parser = Parser::new(BufReader::new(file));

        // let header = parser.parse_header().unwrap();

        // let scope = match &header.items[0] {
        //     ScopeItem::Scope(sc) => sc,
        //     x => panic!("Expected Scope, found {:?}", x),
        // };

        // scope.items.iter().for_each(|x| match x {
        //     ScopeItem::Var(v) => {
        //         variable_types.push(v.var_type.clone());
        //         variable_sizes.push(v.size);
        //         variable_references.push(v.reference.clone());
        //         variable_indexes_ref.push(v.index);
        //     },
        //     x => panic!("Expected Var, found {:?}", x),
        // });

        // variable_indexes_ref.iter().for_each(|x| {
        //     if x.is_some() {
        //         variable_indexes.push(x.unwrap().to_string());
        //     } else {
        //         variable_indexes.push("None".to_string());
        //     }
        // }
        // );
    }
}
