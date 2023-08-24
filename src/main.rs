mod utils;

use utils::{
    argument_handler::argument_handler,
    get_args_type,
    plot_handler::{get_path, plot_handler},
    Arguments,
};

use std::{fs, fs::File, io::BufReader, thread::Scope, vec, collections::HashMap};

use vcd::{Command, Parser, ScopeItem, IdCode, Value};

fn main() {
    let args_type = get_args_type();

    if args_type == Arguments::Version
        || args_type == Arguments::Empty
        || args_type == Arguments::Help
        || args_type == Arguments::Version
    {
        argument_handler();
    } else {
        // plot_handler();
        let file_path = get_path();
        let file = File::open(file_path).unwrap();
        let mut allow_print = true;
        // let mut parse_line_by_line = vec![];

        let mut parser = Parser::new(BufReader::new(file));

        let header = parser.parse_header().unwrap();

        let mut variable_types = Vec::new();
        let mut variable_sizes = Vec::new();
        let mut variable_references = Vec::new();
        // for temporarily holding variable indexes
        let mut variable_indexes_ref = Vec::new();
        // let mut variable_indexes = Vec::new();
        let mut variable_codes = Vec::new();
        let mut variable_values = HashMap::new();
        let mut variable_value_types = HashMap::new();

        let scope = match &header.items[0] {
            ScopeItem::Scope(sc) => sc,
            // x => panic!("Expected Scope, found {:?}", x),
            _ => {
                allow_print = false;
                panic!("Expected Scope, found something else");
            }
        };

        scope.items.iter().for_each(|x| match x {
            ScopeItem::Var(v) => {
                variable_types.push(v.var_type.to_string());
                variable_sizes.push(v.size.to_string());
                variable_references.push(v.reference.clone());
                variable_indexes_ref.push(v.index);
                variable_codes.push(v.code.to_string());
                variable_values.insert(v.code.to_string(), Vec::<String>::new());
            }
            x => panic!("Expected Var, found {:?}", x),
        });

        // for (index, element) in variable_indexes_ref.iter().enumerate() {
        //     println!("{} {} {} {} {:?} {}", index, variable_types[index], variable_sizes[index], variable_references[index], &element, variable_codes[index]);
        // }

        // variable_codes.iter().for_each(|f| {
        //     println!("{}", f);
        // });

        parser.for_each(|f| {
            match f.unwrap() {
                Command::ChangeScalar(id, value) => {
                    if value == Value::V0 {
                        variable_value_types.insert(id.to_string(), "0".to_string());
                    } else if value == Value::V1 {
                        variable_value_types.insert(id.to_string(), "1".to_string());
                    } else if value == Value::X {
                        variable_value_types.insert(id.to_string(), "X".to_string());
                    } else if value == Value::Z {
                        variable_value_types.insert(id.to_string(), "Z".to_string());
                    } else {
                        variable_value_types.insert(id.to_string(), "U".to_string());
                    }
                    // println!("{} {}", id, value);
                    variable_values.get_mut(&id.to_string()).unwrap().push(value.to_string());
                },
                _ => {
                    // println!("Something else");
                }
            }
        });

        variable_value_types.iter().for_each(|v| println!("{:?}", v));

        variable_values.iter().for_each(|(k, v)| {
            println!("{} {:?}", k, v);
        });
    }
}
