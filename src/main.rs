#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate error_chain;

extern crate serde_json;

mod result;
use result::*;

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use serde_json::Value;

const DEFAULT_PORT: &'static str = "12345";
const BUF_SIZE: usize = 1024;

/// Each connection is considered a program.
fn handle_client(stream: &mut TcpStream) -> Result<()> {
    loop {
        let mut buffer = [ 0; BUF_SIZE ];

        // TODO: Messages greater than buffer size are truncated and will cause a JSON error
        let read_size = stream.read(&mut buffer).chain_err(|| "Failed to read stream.")?;
        if read_size == 0 {
            // Either zero sized message or EOF
            break;
        }

        let read_str = String::from(std::str::from_utf8(&buffer).chain_err(|| "Failed to parse utf-8.")?);
        let read_str = read_str.trim();

        println!("{:?}", read_str);
        println!("{}", read_str);

        //let v: Value = serde_json::from_str(&read_str).chain_err(|| "Failed to parse JSON.")?;

        //println!("VALUE: {:?}", v["key"]);
    }


    println!("Connection closed.");
    Ok(())
}

fn run(args: u16) -> Result<()> {
    println!("Running!");

    println!("Listening on port {}", args);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args)).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.chain_err(|| "Failed to open stream.")?;
        match handle_client(&mut stream).chain_err(|| "Failed handling stream.") {
            Ok(_) => {},
            Err(e) => {
                println!(" Failed to handle connection: {}", e);
            }
        }
    }

    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let listen_port = matches.value_of("port")
                             .unwrap_or(DEFAULT_PORT)
                             .parse::<u16>()
                             .expect("Invalid port");


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
