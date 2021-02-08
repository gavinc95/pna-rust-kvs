use clap::{App, Arg, SubCommand};
fn main() {
    println!("Hello, World!");
    let app = App::new("key value store")
        .version("1.0")
        .author("Gavin Chan");
    let get_subcommand = SubCommand::with_name("get")
        .about("Retrieve a key from the store")
        .arg(
            Arg::with_name("key")
                .help("The key to retrieve")
                .required(true),
        );
    let app = app.subcommand(get_subcommand);

    let matches = app.get_matches();
    match matches.subcommand() {
        ("get", Some(value)) => {
            println!("get arg: {:?}", value);
        }
        _ => unreachable!(),
    }
}
