use ::std::io::Read;
use ::std::net::{TcpListener, TcpStream};
use ::serde_json::Value;
use ::result::*;

const BUF_SIZE: usize = 1024;

/// Each connection is considered a program.
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


        let v: Value = ::serde_json::from_str(&read_str).chain_err(|| "Failed to parse JSON.")?;

        println!("VALUE: {:?}", v["key"]);
    }


    println!("Connection closed.");
    Ok(())
}

pub fn listen(host: &str, port: &str) -> Result<()> {
    let port = port.parse::<u16>().expect("Invalid port.");
    let bind_str = format!("{}:{}", host, port);


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
