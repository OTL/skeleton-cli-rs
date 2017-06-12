pub struct App;

use errors::*;
use clap::ArgMatches;

impl App {
    pub fn run_command(&self, _: &ArgMatches) -> Result<()> {
        Ok(())
    }
}