use crate::commands;

pub fn init() -> clap::Command {
    clap::Command::new("sshive")
        .author("Abdugani Toshmukhamedov")
        .about("It helps to manage SSH connections")
        .version("0.1.0-alpha.1")
        .arg_required_else_help(true)
        .subcommand(commands::connect::connect_command())
}
