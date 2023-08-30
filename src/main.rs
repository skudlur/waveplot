mod utils;

use utils::{
    argument_handler::argument_handler,
    get_args_type,
    plot_handler::{get_path, plot_handler},
    Arguments,
};

use std::{collections::HashMap, fs, fs::File, io::BufReader, thread::Scope};

use vcd::{Command, IdCode, Parser, ScopeItem, Value};

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
        // let mut allow_print = true;
        // // let mut parse_line_by_line = vec![];

        // let mut parser = Parser::new(BufReader::new(file));

        // let header = parser.parse_header().unwrap();

        // let mut variable_types = Vec::new();
        // let mut variable_sizes = Vec::new();
        // let mut variable_references = Vec::new();
        // // for temporarily holding variable indexes
        // let mut variable_indexes_ref = Vec::new();
        // // let mut variable_indexes = Vec::new();
        // let mut variable_codes = Vec::new();
        // let mut variable_values = HashMap::new();
        // let mut variable_value_types = HashMap::new();
        // let mut variable_time_stamps = Vec::new();
        // let mut variable_graph_coordinates = HashMap::new();

        // let scope = header.items.iter().find_map(|f| {
        //     if let ScopeItem::Scope(scope) = f {
        //         Some(scope.clone())
        //     } else {
        //         None
        //     }
        // });



        // scope.unwrap().items.iter().for_each(|x| match x {
        //     ScopeItem::Var(v) => {
        //         variable_types.push(v.var_type.to_string());
        //         variable_sizes.push(v.size.to_string());
        //         variable_references.push(v.reference.clone());
        //         variable_indexes_ref.push(v.index);
        //         variable_codes.push(v.code.to_string());
        //         variable_values.insert(v.code.to_string(), Vec::<String>::new());
        //     }
        //     x => panic!("Expected Var, found {:?}", x),
        // });

        // // if variable_values are not as long as the timestamp then we need to
        // // fill in the gaps with the last value

        // parser.for_each(|f| {
        //     match f.unwrap() {
        //         Command::ChangeScalar(id, value) => {
        //             if value == Value::V0 {
        //                 variable_value_types.insert(id.to_string(), "0".to_string());
        //             } else if value == Value::V1 {
        //                 variable_value_types.insert(id.to_string(), "1".to_string());
        //             } else if value == Value::X {
        //                 variable_value_types.insert(id.to_string(), "X".to_string());
        //             } else if value == Value::Z {
        //                 variable_value_types.insert(id.to_string(), "Z".to_string());
        //             } else {
        //                 variable_value_types.insert(id.to_string(), "U".to_string());
        //             }
        //             variable_values
        //                 .get_mut(&id.to_string())
        //                 .unwrap()
        //                 .push(value.to_string());
        //         }
        //         Command::Timestamp(time) => {
        //             variable_time_stamps.push(time);
        //         }
        //         _ => {
        //             // println!("Something else");
        //         }
        //     }
        // });

        // variable_value_types.iter().for_each(|v| {
        //     if v.1 == "0" || v.1 == "1" {
        //         // fetch its values from variable_values
        //         variable_graph_coordinates.insert(v.0, Vec::<(u64, u64)>::new());

        //         for (index, element) in variable_values.get(v.0).unwrap().iter().enumerate() {
        //             if element == "0" || element == "1" {
        //                 variable_graph_coordinates.get_mut(v.0).unwrap().push((
        //                     variable_time_stamps[index],
        //                     element.to_string().parse::<u64>().unwrap(),
        //                 ));
        //             }
        //         }
        //     }
        // });

        // variable_graph_coordinates.iter_mut().for_each(|v| {
        //     if variable_time_stamps.len() > v.1.len() {
        //         for index in v.1.len()..variable_time_stamps.len() {
        //             v.1.push((variable_time_stamps[index], v.1[v.1.len() - 1].1));
        //         }
        //     }
        // });

        // let mut variable_graphs_converted_coordinates = Vec::new();

        // variable_graph_coordinates.iter().for_each(|(key, value)| {
        //     let converted_data: Vec<(f64, f64)> =
        //         value.iter().map(|(a, b)| (*a as f64, *b as f64)).collect();

        //     variable_graphs_converted_coordinates.push(converted_data);
        // });

        // variable_graphs_converted_coordinates.iter().for_each(|v| {
        //     println!("{:?}", v);
        // });
    }
}
