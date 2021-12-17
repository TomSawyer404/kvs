extern crate clap;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process;

fn main() {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap().to_owned();
            let val = matches.value_of("VALUE").unwrap().to_owned();

            let store = KvStore::open("./dblog.txt");
            match store {
                Ok(mut store) => {
                    if let Err(e) = store.set(key, val) {
                        eprintln!("errors -> {:?}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("errors -> {:?}", e);
                    process::exit(1);
                }
            }

            process::exit(0);
        }
        ("get", _matches) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        ("rm", _matches) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => {
            eprintln!("Usage: kvs [FLAGS] [SUBCOMMAND]");
            process::exit(1);
        }
    }
}
