use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process::{Command, Stdio};

enum DmenuCommand {
    Pipe(Vec<String>),
    SetName(),
}

fn handle_streams(stream: &mut TcpStream) {
    let mut buf: Vec<u8> = Vec::with_capacity(1024);

    stream.write(b"Connection received.").unwrap();

    while match stream.read_to_end(&mut buf) {
        Ok(_size) => {
            println!("{}", std::str::from_utf8(&buf).unwrap());

            let commands = create_dmenu_commands(&buf).expect("Failed to get args.");

            true
        }
        Err(e) => {
            eprintln!("TCP Read Error: {:#?}", e);
            false
        }
    } {}

    println!("TCP stream is being shutdown");
    stream.shutdown(Shutdown::Both).unwrap();
}

fn create_dmenu_commands<'a>(buffer: &[u8]) -> Result<Vec<DmenuCommand>, std::io::Error> {
    let mut args: Vec<DmenuCommand> = Vec::new();

    if args.len() <= 0 {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not parse commands.",
        ))
    } else {
        Ok(args)
    }
}

fn main() -> std::io::Result<()> {
    let _args: Vec<String> = std::env::args().collect();

    /*
    let child = Command::new("dmenu")
            .stdin(Stdio::inherit())
            .spawn()
            .expect("Could not start dmenu-manager");
    */

    let listener = TcpListener::bind("127.0.0.1:5000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                std::thread::spawn(move || {
                    handle_streams(&mut stream);
                });
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    drop(listener);

    Ok(())
    // println!("{}", String::from_utf8(child.stdout).unwrap());
}
