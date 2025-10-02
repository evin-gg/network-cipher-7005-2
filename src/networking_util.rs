#![allow(dead_code)]

// standard
use std::os::fd::AsRawFd;
use std::net::{Ipv4Addr, IpAddr, SocketAddrV4};

// network sockets
use nix::sys::socket::{
    MsgFlags, send, recv
};
use socket2::{Socket, Domain, Type, SockAddr};

// other util
use get_if_addrs::get_if_addrs;



// ---Client Setup functions---

// Validates arg count (variable)
pub fn client_arg_validation(args: Vec<String>) -> Result<(), String> {
    if args.len() != 5 {
        return Err("[CLIENT] Usage: <message> <key> <IP address> <port>".to_string());
    }
    else {
        Ok(()) 
    }
}

// formatting into send (variable)
pub fn format_send(args: Vec<String>, sock: &Socket) -> Result<(), String> {
    let payload = format!("{}|{}", args[1], args[2]);

    match send(sock.as_raw_fd(), payload.as_bytes(), MsgFlags::empty()) {
        Ok(_bytes) => {return Ok(())},
        Err(_e) => {
            return Err("[CLIENT] Could not send data".into());
        }
    };
}

// Reading a response
pub fn client_response_handler(socket: &Socket) { 
    let mut buffer = [0u8; 1024];
    let _read_bytes = match recv(socket.as_raw_fd(), &mut buffer, MsgFlags::empty()) {
        Ok(b) => {b},
        Err(_b) => {
            println!("Bytes not received");
            return;
        }
    };

    println!("Message from server: {}", String::from_utf8_lossy(&buffer));
}
// --- END ---

// ---Server Setup functions---

// correct amount of server args
pub fn server_arg_validation(args: &Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err("Usage: <path>".into());
    }
    else {
        Ok(()) 
    }
}

pub fn setup_server(args: &Vec<String>) -> Result<Socket, String> {
    let local_ip: Ipv4Addr = args[1].parse().unwrap();
    let socket = match Socket::new(Domain::IPV4, Type::STREAM, None) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("[CLIENT] Socket creation failed: {}", e))
        }
    };

    let addr = SockAddr::from(SocketAddrV4::new(local_ip, 0));
    match socket.bind(&addr) {
        Ok(()) => {},
        Err(e) => {
            return Err(format!("[CLIENT] Bind failed: {}", e))
        }
    }

    match socket.listen(5) {
        Ok(()) => {},
        Err(e) => {
            return Err(format!("[CLIENT] Listen failed: {}", e))
        }
    }

    let local_addr = socket.local_addr().expect("[SERVER] Could not get local address");

    let std_addr = local_addr.as_socket_ipv4().unwrap();
    println!("[SERVER] Server listening on {}", std_addr);

    return Ok(socket)
}
// --- END ---


// ---Universal---

// checks for a valid ip
pub fn check_valid_ip(argpath: &String) -> Result<(), String> {

    let addr: Result<IpAddr, String> = match argpath.parse::<IpAddr>() {
        Ok(ip) => Ok(ip),
        Err(_) => {
            return Err("Invalid IP address".into());
        }
    };

    if addr?.is_loopback() {
        return Err("IP address not allowed for use".into());
    }

    Ok(())
}

// getifaddrs for rust if needed
pub fn find_address() -> Option<Ipv4Addr> {
    for interface in get_if_addrs().expect("[SERVER] Could not get network interfaces") {
        println!("[SERVER] Interface: {} - IP: {}", interface.name, interface.ip());
        if let IpAddr::V4(ipv4) = interface.ip() {
            if !ipv4.is_loopback() {
                return Some(ipv4)
            }
        }
    }

    return None
}
// --- END ---