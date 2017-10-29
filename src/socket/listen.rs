use ::std::io::Read;
use ::std::net::{TcpListener, TcpStream};
use ::serde_json::Value;
use ::result::*;

use super::commands;

const BUF_SIZE: usize = 1024;

/// Handles a valid JSON request
pub fn handle_json(request: Value) -> Result<()> {
    let request = request.as_object().chain_err(|| format!("Invalid request: {:?}.", request))?;

    for (ref key, ref val) in request.iter() {
        match key.as_ref() {
            "request" => commands::request(val)?,
            "ping" => println!("Pinged."),
            e => return Err(format!("Unknown key: {}", e).into())
        }
    }

    Ok(())
}

/// Each connection is considered a program.
/// Handle all requests during the session.
pub fn handle_client(stream: &mut TcpStream) -> Result<()> {
    loop {
        let mut buffer = [ 0; BUF_SIZE ];

        // TODO: Messages greater than buffer size are truncated and will cause a JSON error
        let read_size = stream.read(&mut buffer).chain_err(|| "Failed to read stream.")?;
        if read_size == 0 {
            // Either zero sized message or EOF
            break;
        }

        let read_str = ::std::str::from_utf8(&buffer[..read_size]).chain_err(|| "Failed to parse utf-8.")?.trim();

        println!("Received {}", read_str);

        let v: Value = ::serde_json::from_str(&read_str).chain_err(|| "Failed to parse JSON.")?;


        handle_json(v)?;
    }


    println!("Connection closed.");
    Ok(())
}

/// Listen and handle requests
pub fn listen(host: &str, port: &str) -> Result<()> {
    let port = port.parse::<u16>().expect("Invalid port.");
    let bind_str = format!("{}:{}", host, port);

    println!("Listening on {}", &bind_str);

    let listener = TcpListener::bind(bind_str).expect("Failed to bind.");

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
