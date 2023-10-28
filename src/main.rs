pub mod cli;
pub mod commands;
pub mod config;
pub mod logger;

use std::process;

fn main() {
    let config = config::Config::default();
    let matches = cli::init().get_matches();

    match matches.subcommand() {
        Some(("connect", sub_matches)) => {
            let tag = sub_matches.get_one::<String>("TAG").unwrap();

            let target =
                config.value.connections.iter().find(|connection| connection.tag.eq(tag)).unwrap_or_else(|| {
                    eprintln!("Connection not found with \"{}\" tag", tag);
                    process::exit(0);
                });

            process::Command::new("ssh")
                .args(["-o", "ConnectTimeout=5"])
                .args(["-i", target.identity_file.to_str().unwrap()])
                .arg(format!("{}@{}", target.user, target.host))
                .args(["-p", target.port.unwrap_or(22).to_string().as_str()])
                .spawn()
                .unwrap_or_else(|err| {
                    eprintln!("Problem with connection: {}", err);
                    process::exit(0);
                })
                .wait()
                .unwrap_or_else(|err| {
                    eprintln!("Problem with waiting: {}", err);
                    process::exit(0);
                });
        },
        _ => unreachable!(),
    }
}
