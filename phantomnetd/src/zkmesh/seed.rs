use std::{fs::File, io::{BufRead, BufReader}, thread, time::Duration};
use std::net::TcpStream;
use std::io::Write;

pub fn start_peer_seeding(peers: Vec<String>) {
    thread::spawn(move || loop {
        let tips = read_tips(10);
        let payload = format!("POST /zk/tangle/submit\n{}", tips.join("\n"));
        for peer in &peers {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let _ = stream.write_all(payload.as_bytes());
            }
        }
        thread::sleep(Duration::from_secs(30));
    });
}

fn read_tips(max: usize) -> Vec<String> {
    let file = File::open("data/zktangle.log").ok();
    let mut lines = vec![];
    if let Some(f) = file {
        lines = BufReader::new(f)
            .lines()
            .flatten()
            .rev()
            .take(max)
            .collect();
    }
    lines
}