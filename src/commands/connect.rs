// connect subcommand
// Example: sshive connect [tag]
pub fn connect_command() -> clap::Command {
    clap::Command::new("connect")
        .about("connect to server")
        .arg(clap::arg!(<TAG> "The tag of connection to connect"))
        .arg_required_else_help(true)
}
