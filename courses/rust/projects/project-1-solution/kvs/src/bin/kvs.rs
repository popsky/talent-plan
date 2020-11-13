use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::ArgRequiredElseHelp)]
struct Foo {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
#[derive(Debug, StructOpt)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() {
    let _opt = Foo::from_args();
    eprintln!("unimplemented");
    std::process::exit(1)
}
// use clap::{App, AppSettings, Arg, SubCommand};
// use kvs::KvStore;

// fn main() {
//     let _kv = KvStore::new();
//     let matches = App::new(env!("CARGO_PKG_NAME"))
//         .setting(AppSettings::SubcommandRequiredElseHelp)
//         .version(env!("CARGO_PKG_VERSION"))
//         .author(env!("CARGO_PKG_AUTHORS"))
//         .about(env!("CARGO_PKG_DESCRIPTION"))
//         .subcommand(
//             SubCommand::with_name("set")
//                 .about("Set the value of a string key to a string")
//                 .arg(Arg::with_name("KEY").required(true))
//                 .arg(Arg::with_name("VALUE").required(true)),
//         )
//         .subcommand(
//             SubCommand::with_name("get")
//                 .about("Get the string value of a given string key")
//                 .arg(Arg::with_name("KEY").required(true)),
//         )
//         .subcommand(
//             SubCommand::with_name("rm")
//                 .about("Remove a given key")
//                 .arg(Arg::with_name("KEY").required(true)),
//         )
//         .get_matches();
//     match matches.subcommand() {
//         ("get", Some(sub_m)) => {
//             let _key = sub_m.value_of("KEY").unwrap();
//             eprintln!("unimplemented");
//             std::process::exit(1)
//         }
//         ("set", Some(sub_m)) => {
//             let _key = sub_m.value_of("KEY").unwrap();
//             let _value = sub_m.value_of("VALUE").unwrap();
//             eprintln!("unimplemented");
//             std::process::exit(1)
//         }
//         ("rm", Some(sub_m)) => {
//             let _key = sub_m.value_of("KEY").unwrap();
//             eprintln!("unimplemented");
//             std::process::exit(1)
//         }
//         _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
//     }
// }
