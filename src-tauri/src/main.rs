// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::thread;
use std::{fs::File, io::Read, path::Path};

use tauri::async_runtime::block_on;

fn main() {
    // let server = start_server().await;
   thread::spawn(|| {
        block_on(start_server());
    });
    

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn send_file(path: String) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents: {}", contents);
}

fn scan_peers() {
    // read arp table and ping each ip
    // if ping is successful, add to list of ips
    // return list of ips

    let mut peers: Vec<String> = Vec::new();

    // run os command to get arp table
    if cfg!(windows) {
        peers.append(&mut parse_arp_win());
    } else {
        parse_arp_linux();
    }
}

fn parse_arp_win() -> Vec<String> {
    // run os command to get arp table
    let output = Command::new("arp")
        .arg("-a")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    // parse arp table
    let mut ips: Vec<String> = Vec::new();
    for line in output.lines() {
        if line.is_empty() {
            continue;
        };
        if line.contains("---") {
            continue;
        };
        if !line.contains(".") && !line.contains(":") {
            continue;
        };

        let ip = line.split(" ").collect::<Vec<&str>>()[0];

        let conn = TcpStream::connect(format!("{}:80", ip));
        if conn.is_err() {
            continue;
        }
        let mut conn = conn.unwrap();
        conn.write_all(b"Hello World!").expect("Could not write");

        ips.push(ip.to_string());
    }
    ips
}

fn parse_arp_linux() {
    println!("linux");
}

async fn start_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80").expect("Could not bind");
    println!("Server started!");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.write(b"Hello World!").expect("Could not write");
    }

    Ok(())
}
