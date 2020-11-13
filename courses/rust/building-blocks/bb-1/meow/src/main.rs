use clap::{load_yaml, App};
use dotenv_codegen::dotenv;
use std::env;
use std::fmt;

fn main() {
    hoo();
    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    }
    println!("{}", dotenv!("PORT"));
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // Same as previous examples...
}

enum MyErr {
    Reason1,
    Reason2,
}
fn foo(o: Option<i32>) -> Result<(), MyErr> {
    match o {
        Some(_) => Ok(()),
        None => Err(MyErr::Reason1),
    }
}
fn hoo() {
    match foo(None) {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e), // `e` not work yet
                                     // we need `fmt` to tranlate to the message
    }
}
impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyErr::Reason1 => write!(f, " reason 1 is the error"),
            MyErr::Reason2 => write!(f, "reason 2 and ` are error"),
        }
    }
}
