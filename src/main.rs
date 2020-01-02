#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_value;

extern crate structopt;

mod command_control;
mod toolbelt;
mod merger;

use merger::SchemaMerger;
use toolbelt::logs::RootLog;
use structopt::StructOpt;

fn main() {
    // This is the collection of settings sent from the request.
    let cli_options = command_control::CmdCtl::from_args()
        .run_command_process();

    // This should be passed to any underlying modules and follow verbose logic rules for CLI.
    let log_config = RootLog::get_logger(
        cli_options.is_verbose()
    );

    // This is a processing libary. Give it what it wants.
    let mut merger = SchemaMerger::new(cli_options.get_files()).set_logger(log_config);
    // let input =
    // let mixin = 
    // merger.merge(input, mixin);
}
