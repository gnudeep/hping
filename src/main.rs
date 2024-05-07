use std::env;
use std::time::{Instant, Duration};
use reqwest::blocking::Client;

fn emulate_ping(url: &str) {
    let start_time = Instant::now();
    match Client::new().get(url).send() {
        Ok(response) => {
            let end_time = Instant::now();
            let rtt = end_time.duration_since(start_time).as_secs_f64() * 1000.0;
            println!("Response from {}: {}, RTT: {:.2} ms", url, response.status(), rtt);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn construct_url(domain: &str) -> String {
    // Decide on the default protocol (e.g., "http://")
    let protocol = "https://";
    // Construct the full URL
    format!("{}{}", protocol, domain)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <domain name>", args[0]);
        std::process::exit(1);
    }

	let domain = &args[1];
    let target_url = construct_url(domain);
    println!("Pinging {} indefinitely...", target_url);
    loop {
        emulate_ping(&target_url);
        std::thread::sleep(Duration::from_secs(1)); // Adjust sleep time as needed
    }
}
