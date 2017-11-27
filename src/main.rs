#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate serde_json;

use clap::App;
use result::*;

mod result;
mod process;
mod socket;


const DEFAULT_SOCK: &'static str = "kinte.sock";

fn run() -> Result<()> {
    println!("Running!");

    // TODO: define socket location
    socket::listen(&DEFAULT_SOCK)?;

    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let msg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(msg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(msg);
        }
    }
}
