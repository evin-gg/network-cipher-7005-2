#![allow(dead_code)]

use std::os::fd::OwnedFd;
use nix::errno::Errno;
use nix::sys::socket::{
    socket, AddressFamily, SockType, SockFlag, MsgFlags, send
};

use std::os::fd::AsRawFd;

use std::path::Path;

use socket2::{Socket};


//-------------------------------------------------------------------
// pub static CATCH_SIGINT: AtomicBool = AtomicBool::new(false);

// extern "C" fn sigint_handler(_sig: i32) {
//     CATCH_SIGINT.store(true, Ordering::SeqCst);
// }

// pub fn sigint_init() -> SigAction {
//     return signal::SigAction::new(
//         signal::SigHandler::Handler(sigint_handler),
//         signal::SaFlags::empty(),
//         signal::SigSet::empty());
// }
//-------------------------------------------------------------------




// Client Setup functions
pub fn client_check_validpath(argpath: &Vec<String>) -> Result<(), String> {
    let path = Path::new(&argpath[3]);
    if path.exists() {
        Ok(())
    } else {
        Err("[CLIENT] Socket does not exist".to_string())
    }
}

pub fn client_arg_validation(args: Vec<String>) -> Result<(), String> {
    if args.len() != 5 {
        return Err("[CLIENT] Usage: <message> <key> <IPv4 address> <port>".to_string());
    }
    else {
        Ok(()) 
    }
}

pub fn create_socket() -> Result<OwnedFd, Errno> {
    return socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None);
}

pub fn format_send(args: Vec<String>, sock: &Socket) -> Result<(), Errno> {
    let payload = format!("{}|{}", args[1], args[2]);

    match send(sock.as_raw_fd(), payload.as_bytes(), MsgFlags::empty()) {
        Ok(_bytes) => {return Ok(())},
        Err(e) => {
            return Err(e);
        }
    };
}

// Server Setup functions



pub fn server_check_validpath(argpath: &Vec<String>) -> Result<(), String> {

    let path = Path::new(&argpath[1]);
    if path.exists() {
        Err("[SERVER] Old socket found, it will be unlinked and replaced".to_string())
    } else {
        Ok(())
    }
}

pub fn server_arg_validation(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err("[SERVER] Usage: <socketpath>".to_string());
    }
    else {
        Ok(()) 
    }
}
