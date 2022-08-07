use std::{net::{TcpStream, SocketAddr}, thread::JoinHandle};
use log::info;
use std::thread;
use crate::ipv4_gen;

async fn check_ack(dest: &SocketAddr) -> bool {
    if let Ok(res) = TcpStream::connect(dest) {
        info!("Got TCP ack from: {:?}", dest);
        return true;
    }
    return false;
}

pub fn start_scan(depth: u32) {
    let threads: Vec<JoinHandle<()>> = Vec::new();

    for i in 0..depth {
        let thread = thread::spawn(|| {
            println!("hi"); 
        });
    }
}


/*

depth: u32
blacklist_ips: Vec

pre:
    # generate IPs
    ALL_IPS: Vec<Vecu8>> = ...

*/
