use crate::ipv4::{IPv4, IPv4Range};
use anyhow::Result;
use log::info;
use std::net::{SocketAddr, TcpStream};
use std::thread::JoinHandle;
use std::{panic, thread};

fn tcp_scan(mut target: IPv4, target_port: u16) -> bool {
    let _dest = target
        .to_socketaddr(target_port)
        .unwrap_or_else(|e| panic!("{}", e));

    false
    // TODO: FIX
    //     if let Ok(res) = TcpStream::connect(dest) {
    //         info!("* Got TCP ack from: {:?} | {:?}", dest, res);
    //         return true;
    //     }
    //     false
}

fn create_scan_thread(
    thread_id: u32,
    ip_range: IPv4Range,
    target_port: u16,
) -> JoinHandle<Vec<(u32, bool)>> {
    thread::spawn(move || {
        info!("Starting thread worker #{}", thread_id);
        let mut results: Vec<(u32, bool)> = Vec::new();

        // do the scan thing
        ip_range.into_iter().for_each(|id| {
            let target = IPv4::new(id as u64);
            let result = tcp_scan(target, target_port);
            results.push((id, result));
        });

        results
    })
}

fn create_scan_worker(
    thread_id: u32,
    ips_per_thread: u32,
    target_port: u16,
    ignorelist: Vec<u32>,
) -> thread::Result<Vec<(u32, bool)>> {
    let (f, t) = (
        (thread_id * ips_per_thread),
        ((thread_id + 1) * ips_per_thread),
    );
    let range = IPv4Range::new(f, t, ignorelist);
    let thread = create_scan_thread(thread_id, range, target_port);
    let result = thread.join();

    match result {
        Ok(_) => return result,
        Err(e) => panic!("** Worker panic! {:?}", e),
    }
}

fn create_scan_workers(
    from: u32,
    to: u32,
    target_port: u16,
    num_threads: u32,
    ignorelist: Option<Vec<u32>>,
) -> Vec<(u32, bool)> {
    println!("Starting wwmap...");

    let ips_per_thread = (((to - from) as f32) / num_threads as f32) as u32;
    let ips_left = (to - from) - (num_threads * ips_per_thread); // how many ips we have left after the first threads

    // container for all of our threads
    // let mut threads: Vec<JoinHandle<Vec<bool>>> = Vec::new();

    // container for all of our results
    let mut results: Vec<(u32, bool)> = Vec::new();

    for thread_id in 0..num_threads {
        let id_ignorelist = ignorelist.clone().unwrap_or_default();

        // Create a worker
        let mut worker_results =
            create_scan_worker(thread_id, ips_per_thread, target_port, id_ignorelist).unwrap();

        results.append(&mut worker_results);
    }

    // Clean up the rest
    if ips_left > 0 {
        let id_ignorelist = ignorelist.clone().unwrap_or_default();
        let mut worker_results = create_scan_worker(
            results.len() as u32 + 1,
            ips_per_thread,
            target_port,
            id_ignorelist,
        )
        .unwrap();
        results.append(&mut worker_results);
    }

    results
}

pub fn start_scan(
    from: u32,
    to: u32,
    target_port: u16,
    num_threads: u32,
    ignorelist: Option<Vec<u32>>,
) {
    let threads = create_scan_workers(from, to, target_port, num_threads, ignorelist);

    threads.iter().for_each(|t| {
        println!("{t:?}");
    });
}
