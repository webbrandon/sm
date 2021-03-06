use std::path::PathBuf;
use clap::ArgSettings::Hidden;
use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use super::Commands;

#[derive(Debug, StructOpt, Default)]
#[structopt(
    global_settings = &[DisableVersion, DisableHelpSubcommand, DeriveDisplayOrder, VersionlessSubcommands],
    about = "\nMerge two schema bodies together."
)]
pub struct CmdCtl {


    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    /// Base file path.
    #[structopt(long = "input", short = "i")]
    pub input: PathBuf,

    /// File path.
    #[structopt(long = "mixin", short = "m")]
    pub mixin: PathBuf,

    /// Output to file and leave base file as is.
    ///
    /**
     Format Details
     --------------
     Best efforts will be used for format based on mime type.
     Use 'format:path/to' notation for strict assignments.
    */
    #[structopt(long = "output", short = "o")]
    pub output: PathBuf,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

impl CmdCtl {

    pub fn run_command_process(self) -> CmdCtl {
        match &self.commands {
            Some(commands) => {
                commands.process_completions();
                self
            },
            None => self
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.commands.clone() {
            Some(commands) => commands.is_verbose(),
            None => self.verbose
        }
    }

    pub fn get_files(self) -> (PathBuf, PathBuf) {
        (self.input, self.mixin)
    }

}
