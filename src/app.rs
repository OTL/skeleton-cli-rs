pub struct App;

use errors::*;
use clap::ArgMatches;

impl App {
    pub fn run_command(&self, _: &ArgMatches) -> Result<()> {
        debug!("Debug messge");
        info!("Info message");
        warn!("Some data, {}", 2);
        error!("Some critical error");

        Ok(())
    }
}