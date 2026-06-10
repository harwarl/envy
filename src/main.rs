use std::process::{exit, Command};
use std::os::unix::process::CommandExt;
use clap::{Arg, Command as ClapCommand};
use envparser;

macro_rules! die {
    ($fmt:expr) => ({
        eprintln!($fmt);
        exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        exit(1);
    });
}

fn make_command(name: &str, args: Vec<&str>) -> Command {
    let mut command = Command::new(name);
    for arg in args {
        command.arg(arg);
    }
    command
}

fn main() {
    let matches = ClapCommand::new("envparser")
        .about("Run a command using the environment in a .env file")
        .override_usage("envparser <COMMAND> [ARGS]...")
        .allow_external_subcommands(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("FILE")
                .short('f')
                .long("file")
                .num_args(1)
                .help("Use a specific .env file (defaults to .env)"),
        )
        .get_matches();

    match matches.get_one::<String>("FILE").map(|s| s.as_str()) {
        None => envparser::dotenv(),
        Some(file) => envparser::from_filename(file),
    }
    .unwrap_or_else(|e| die!("error: failed to load environmental variables: {}", e));

    let mut command = match matches.subcommand() {
        Some((name, sub_matches)) => {
            let args = sub_matches
                .get_many::<String>("")
                .map(|v| v.map(|s| s.as_str()).collect())
                .unwrap_or(Vec::new());
            make_command(name, args)
        }
        None => die!("error: missing required argument <COMMAND>"),
    };

    if cfg!(target_os = "windows") {
        match command.spawn().and_then(|mut child| child.wait()) {
            Ok(status) => exit(status.code().unwrap_or(1)),
            Err(error) => die!("fatal: {}", error),
        };
    } else {
        let error = command.exec();
        die!("fatal: {}", error);
    };
}