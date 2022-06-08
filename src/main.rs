use std::ops::Not;

use clap::{arg, ArgMatches, Command};

use crate::cfg::{init_links, restore_links};
use crate::server::run_server;

mod cfg;
mod html;
mod server;

const COMMAND_INIT: &str = "init";
const COMMAND_RUN: &str = "run";
const COMMAND_RESTORE: &str = "restore";

fn cli() -> Command<'static> {
    Command::new("ekezet-srv")
        .about("ekezet.com website")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new(COMMAND_INIT).about("Init links config from sample"))
        .subcommand(Command::new(COMMAND_RUN).about("Run webserver").args(vec![
            arg!(-H --host <ADDR>)
                .default_value("0.0.0.0")
                .required(false),
            arg!(-p --port <NUM>)
                .default_value("8484")
                .required(false),
            arg!(-N --"no-ssl")
                .required(false),
        ]))
        .subcommand(Command::new(COMMAND_RESTORE).about("Restore last correct config"))
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some((COMMAND_INIT, _)) => init_command(),
        Some((COMMAND_RUN, submatches)) => run_command(submatches),
        Some((COMMAND_RESTORE, _)) => restore_command(),
        _otherwise => {}
    }
}

fn init_command() {
    init_links()
}

fn run_command(matches: &ArgMatches) {
    let host = matches.value_of("host").unwrap_or_default();
    let port: u16 = matches
        .value_of("port")
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();
    let use_ssl: bool = matches.is_present("no-ssl").not();

    print!("addr: {}:{}", host, port);
    if use_ssl {
        print!(" (ssl)");
    }
    println!();

    run_server(host, port, use_ssl).expect("Server cannot be started");
}

fn restore_command() {
    restore_links()
}
