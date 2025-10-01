mod networking_util;
mod cipher;

use nix::sys::socket::*;
// use networking_util::{
//     create_socket, sigint_init, CATCH_SIGINT, server_check_validpath, server_arg_validation
// };

// use::std::{process, env};
use::std::os::fd::AsRawFd;
// use nix::unistd::*;
use std::sync::atomic::Ordering;
// use cipher::split_payload;
// use std::os::fd::OwnedFd;
// use std::mem;
use std::net::*;


use get_if_addrs::get_if_addrs;
use socket2::{Socket, Domain, Type, SockAddr};
// use std::io::Read;
use std::net::{SocketAddrV4, Ipv4Addr};


use std::sync::atomic::AtomicBool;
use ctrlc;
use std::sync::Arc;
// use std::time::Duration;
// use std::sync::atomic::Ordering;
fn main() {

    // let sig_action = sigint_init();
    // unsafe { match signal::sigaction(signal::SIGINT, &sig_action) {
    //     Ok(_sigaction) => {},
    //     Err(e) => {
    //         eprintln!("[SERVER] Error setting up Signal Handler {}", e);
    //         process::exit(1);
    //     }
    // } };

    let catch = Arc::new(AtomicBool::new(true));
    let c = catch.clone();

    ctrlc::set_handler(move || {
        c.store(false, Ordering::SeqCst);
    }).expect("[SERVER] Signal Handler Error");

    let mut local_ip: Option<Ipv4Addr> = None;
    for interface in get_if_addrs().expect("[SERVER] Could not get network interfaces") {
        println!("[SERVER] Interface: {} - IP: {}", interface.name, interface.ip());
        if let IpAddr::V4(ipv4) = interface.ip() {
            if !ipv4.is_loopback() {
                local_ip = Some(ipv4);
                break;
            }
        }
    }


    println!("[SERVER] Local IP Address: {:?}", local_ip.unwrap());






    // Get args and verify
    // let user_args: Vec<String> = env::args().collect();
    // match server_arg_validation(user_args.clone()) {
    //     Ok(())=> {},
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         process::exit(1);
    //     }
    // }

    // Check if the path is valid
    // match server_check_validpath(&user_args) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         eprintln!("{}", e);
    //     }
    // }

    // declare path and unlink to ensure
    // let path = &user_args[1];
    
    // match unlink(path.as_str()) {
    //     Ok(()) => {},
    //     Err(_e) => {}
    // };

    // make a socket 
    // let sock = match create_socket() {
    //     Ok(fd) => fd,
    //     Err(e) => {
    //         eprintln!("[SERVER] Error: {}", e);
    //         process::exit(1);
    //     }
    // };

    // println!("[SERVER] Socket created with fd {}", sock.as_raw_fd());


    let socket =  Socket::new(Domain::IPV4, Type::STREAM, None).expect("[SERVER] Socket Creation Error");
    println!("[SERVER] Socket created with fd {}", socket.as_raw_fd());

    // Create an address 
    // let addr = match UnixAddr::new(path.as_str()) {
    //     Ok(res) => res,
    //     Err(e) => {
    //         eprintln!("[SERVER] Address Error {}", e);
    //         process::exit(1);
    //     }
    // };

    let addr = SockAddr::from(SocketAddrV4::new(local_ip.unwrap(), 0));

    println!("[SERVER] Address: {:?}", addr);

    // // bind socket to the address 
    // match bind(sock.as_raw_fd(), &addr) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         eprintln!("[SERVER] Bind Error {}", e);
    //         process::exit(1);
    //     }
    // };

    socket.bind(&addr).expect("[SERVER] Bind Error");
    

    // // listen for connections 
    // match listen(&sock, Backlog::new(5).expect("Invalid backlog size")) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         eprintln!("[SERVER] Listening Error {}", e);
    //     }
    // };

    socket.listen(5).expect("[SERVER] Listen Error");

    // println!("[SERVER] Listening for connections");
    // while !CATCH_SIGINT.load(Ordering::SeqCst) {
    //     server_loop(&sock);
    // }

    let local_addr = socket.local_addr().expect("[SERVER] Could not get local address");
    let std_addr = local_addr.as_socket_ipv4().unwrap();

    println!("[SERVER] Server listening on {}", std_addr);



    while catch.load(Ordering::SeqCst) {
        let (clientfd, clientaddr) = match socket.accept() {
            Ok((fd, addr)) => {
                println!("[SERVER] Accepted connection from {:?}", addr);
                (fd, addr)
            },
            Err(e) => {
                eprintln!("[SERVER] Accept Error {}", e);
                return;
            }
        };


        println!("[SERVER] Accepted connection from {:?}", clientaddr);

        let mut buf = [0u8; 1024];
        let read_bytes = match recv(clientfd.as_raw_fd(), &mut buf, MsgFlags::empty()){
            Ok(n) => {println!("[SERVER] Received {} bytes", n); n},
            Err(e) => {println!("[SERVER] Error: {}", e); 0},
        };

        println!("[SERVER] Payload: {}", String::from_utf8_lossy(&buf[..read_bytes]));
        send(clientfd.as_raw_fd(), b"owo!", MsgFlags::empty()).expect("[SERVER] Error sending response");
    }

    println!("SIGINT caught, exiting gracefully");
    drop(socket);
    println!("[SERVER] Socket closed, exiting");
}
