mod utils;

use utils::{
    Arguments,
    get_args_type,
    plot_handler::plot_handler,
    argument_handler::argument_handler
};

fn main() {
    let args_type = get_args_type();


    if args_type == Arguments::Version || 
       args_type == Arguments::Empty ||
       args_type == Arguments::Help ||
       args_type == Arguments::Version {
        argument_handler();
       } 
    else {
        plot_handler();
    }
}
