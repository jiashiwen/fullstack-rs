use clap::Command;

pub fn new_stop_cmd() -> Command {
    clap::Command::new("stop").about("stop")
}
