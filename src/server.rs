use core::panic;
use std::io::Read;
use crate::http::Request;
use crate::http::Method; // equivalent to crate::http::Method as for the server module, http is a sibling module
use std::net::{ TcpListener, TcpStream };

pub struct Server {
    addr: String,
}

impl Server {
    #![allow(unused_doc_comments)]
    // private static method
    fn listen(listener: TcpListener) {
        // listen for requests continuously
        loop {
            // accept() is blocking
            match listener.accept() {
                // tcp_stream is mutable for read() to work
                Ok((mut tcp_stream, sock_addr)) => {
                    /// REDO: ideally `buffer` should be a Vec<u8>
                    // array type is BOTH type and size
                    let mut buffer: [u8; 1024] = [0; 1024];
 
                    match tcp_stream.read(&mut buffer) {
                        Ok(bytes_rd) => {
                            println!("Read {} number of bytes.", bytes_rd);
                            println!("REQUEST received (from {:?}):\n{:?}", sock_addr, String::from_utf8_lossy(&buffer));
                            println!("\nListening for requests...");

                            // match Request::try_from(&buffer as &[u8]) {
                            //     Ok(_) => println!(""),
                            //     Err(_) => println!("Error occured while parsing request"),
                            // }
                        }
                        Err(e) => println!("{}", e)
                    }
                }
                Err(e) => print!("An error occured while trying to accept a request: {}", e)
            }
        }
        // loop {
        //     let result = listener.accept();

        //     if result.is_err() {
        //         println!("An error occured while trying to listen for requests: {}", result.err().unwrap()); // err() takes ownership of result here
        //     }

        //     let (tcp_stream, sock_addr) = result.unwrap();
        // }
    }

    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) { // transfer ownership of the callee to run() as run() will execute infinitely
        let listener: TcpListener; // no value assigned, so can be rebinded

        match TcpListener::bind(&self.addr) {
            Ok(value) => {
                listener = value;
                println!("Server up and running on {:?}...", self.addr);
            }
            Err(_) => {
                println!("Error binding socket to the address!");
                panic!("Unable to establish connection. Terminating...");
            }
        };

        Self::listen(listener);
    }
}