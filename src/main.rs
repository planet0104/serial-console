use std::{io::{self, Read, Write}, thread::sleep, time::Duration};

use anyhow::Result;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    port: String,
    baud: u32,
}

fn main() -> Result<()>{
    let args = Cli::from_args();
    let port = args.port.clone();
    let baud = args.baud;

    loop{
        print!("Open {} {}...", port, baud);
        let mut port = serialport::new(&port, baud)
        .timeout(Duration::from_millis(500))
        .open().expect("Failed to open port");

        println!(" OK.");

        let mut serial_buf: Vec<u8> = vec![0; 1000];
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    if let Err(err) = io::stdout().write_all(&serial_buf[..t]){
                        eprintln!("write error: {:?}", err.kind());
                        break;
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => {
                    eprintln!("read error: {:?}", e.kind());
                    break;
                }
            }
        }
        sleep(Duration::from_secs(1));
    }
}
