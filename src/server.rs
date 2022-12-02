use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
use crate::http::Request;

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  pub fn run(self) {
    let listener = TcpListener::bind(&self.addr).unwrap();

    println!("Server listening on {}", self.addr);

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];

          match stream.read(&mut buffer) {
            Ok(_) => {
              match Request::try_from(&buffer[..]) {
                Ok(request) => {},
                Err(e) => println!("Failed to parse a request: {}", e)
              }

              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
            },
            Err(e) => println!("Failed to read from connection: {}", e)
          }
        },
        Err(e) => {
          println!("Failed to establish a connection {}", e);
          continue;
        }
      }
    }
  }
}
