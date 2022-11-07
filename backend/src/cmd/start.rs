use clap::Command;

pub fn new_start_cmd() -> Command {
    clap::Command::new("start").about("start")
}
