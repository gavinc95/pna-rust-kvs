use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));
    let get_subcommand = SubCommand::with_name("get")
        .about("Retrieve a key from the store")
        .arg(
            Arg::with_name("key")
                .help("The key to retrieve")
                .required(true),
        );
    let rm_subcommand = SubCommand::with_name("rm")
        .about("Remove a key from the store")
        .arg(
            Arg::with_name("key")
                .help("The key to remove")
                .required(true),
        );
    let set_subcommand = SubCommand::with_name("set")
        .about("set a key-value pair in the store")
        .arg(
            Arg::with_name("key")
                .help("The key to remove")
                .required(true),
        )
        .arg(
            Arg::with_name("value")
                .help("The value for the key")
                .required(true),
        );
    let version_arg = Arg::with_name("-V")
        .help("Print the version of the key-value store")
        .required(false);
    let app = app.subcommand(get_subcommand);
    let app = app.subcommand(set_subcommand);
    let app = app.subcommand(rm_subcommand);
    let app = app.arg(version_arg);

    let matches = app.get_matches();

    match matches.subcommand() {
        ("get", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
