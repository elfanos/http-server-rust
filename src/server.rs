use std::net::TcpListener;
use std::convert::TryFrom;
use std::io::Read;
use crate::http::Request;

pub struct Server {
    address: String,

}

fn arr() {}
impl Server {
    pub fn new(address:String) -> Self {
        Self {
            address
        }
    }
    pub fn run(self){
        println!("Listening on {} ", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0;1024];

                   match stream.read(&mut buffer){
                       Ok(_) => {
                           println!("Received a connection , {}", String::from_utf8_lossy(&buffer));
                           match Request::try_from(&buffer[..]){
                               Ok(request) => {},
                               Err(error) => println!("Request failed to parse {}", error)
                           }
                        },
                       Err(error)=> println!("Failed to read from connection: {}", error)
                   }
                },
                Err(error) => println!("Failed to handle the connection: {}", error)

            }

        }

    }

}