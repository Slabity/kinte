#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate error_chain;

extern crate nix;

mod result;
use result::*;

use std::io::Write;
use std::io::Read;

mod process;

fn run() -> Result<()> {
    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

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
