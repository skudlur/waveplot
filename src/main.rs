mod utils;

use utils::{
    argument_handler::argument_handler, get_args_type, plot_handler::plot_handler, Arguments,
};

fn main() {
    let args_type = get_args_type();

    if args_type == Arguments::Version
        || args_type == Arguments::Empty
        || args_type == Arguments::Help
        || args_type == Arguments::Version
    {
        // Handle any argument that doesn't require plotting
       let _ = argument_handler();
    } else {
        // Handle vcd files and plot
       let _ = plot_handler();
    }
}
