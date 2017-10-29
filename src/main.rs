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


const HOST: &'static str = "127.0.0.1";
const DEFAULT_PORT: &'static str = "12345";

fn run(port: &str) -> Result<()> {
    println!("Running!");

    socket::listen(HOST, port)?;

    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let listen_port = matches.value_of("port")
                             .unwrap_or(DEFAULT_PORT);

    if let Err(ref e) = run(listen_port) {
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
