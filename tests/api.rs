/// These tests expect the domain socket to exist
/// Currently reponses aren't automated since the client side does not receive any data.
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

const KINTE_SOCK: &'static str = "kinte.sock";

#[test]
fn invalid_json() {
    let mut stream = UnixStream::connect(KINTE_SOCK).unwrap();
    stream.write_all(r#"{ "ping": "irrelevant" }}"#.as_bytes()).unwrap();
}

#[test]
fn ping() {
    let mut stream = UnixStream::connect(KINTE_SOCK).unwrap();
    stream.write_all(r#"{ "ping": "irrelevant value" }"#.as_bytes()).unwrap();
}

#[test]
fn request_print_text() {
    let mut stream = UnixStream::connect(KINTE_SOCK).unwrap();
    stream.write_all(r#"{ "request": {"print_text": "example text."}}"#.as_bytes()).unwrap();
}

#[test]
fn invalid_request() {
    let mut stream = UnixStream::connect(KINTE_SOCK).unwrap();
    stream.write_all(r#"{ "request": "print_text" }"#.as_bytes()).unwrap();
}

