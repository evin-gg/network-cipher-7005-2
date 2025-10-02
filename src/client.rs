mod networking_util;

#[allow(unused_imports)]


use networking_util::{
    format_send, check_valid_ip, client_response_handler
};
use::std::{process, env};
use std::os::fd::AsRawFd;



use socket2::{Socket, Domain, Type, SockAddr};
use std::net::{SocketAddrV4};

fn main() {

    // get user args
    let args: Vec<String> = env::args().collect();

    let socket =  Socket::new(Domain::IPV4, Type::STREAM, None).expect("[CLIENT] Socket Creation Error");
    println!("[CLIENT] Socket created with fd {}", socket.as_raw_fd());


    //verify ip
    match check_valid_ip(&args[3]) {
        Ok(()) => {},
        Err(e) => {
            println!("Ip address error: {}", e);
            process::exit(1);
        }
    }

    let addr = SockAddr::from(SocketAddrV4::new(args[3].parse().unwrap(), args[4].parse().unwrap()));
    

    socket.connect(&addr).expect("[CLIENT] Error Connecting to Server");
    println!("[CLIENT] Connected to server at {:?}", addr);

    // Send the formatted data
    match format_send(args, &socket) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("[CLIENT] Error Sending Data {}", e);
            process::exit(1);
        }
    };

    // Receive the response
    client_response_handler(&socket);
}
