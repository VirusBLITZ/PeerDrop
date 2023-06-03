// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;

use std::io::Write;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::process::{Command, ExitStatus};
use std::sync::mpsc;
use std::time::Duration;
use std::{env, fs, thread};
use std::{fs::File, io::Read};

use libarp::arp::ArpMessage;
use libarp::client::ArpClient;
use tauri::async_runtime::block_on;

fn main() {
    // println!("Peers: {}", scan_peers().join("\n"));
    // let server = start_server().await;
    thread::spawn(|| {
        block_on(listen());
    });

    network::get_subnetmask("eth0");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_file, scan_peers])
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

#[tauri::command]
async fn scan_peers() -> Vec<String> {
    let ips: Vec<String> = vec![];

    let (tx, rx) = mpsc::channel::<String>();

    let scanner = thread::spawn(move || {
        match match env::consts::OS {
            "windows" => Command::new("ping").arg("192.168.0.24").status(),
            "linux" => ArpClient,
            // Command::new("ping")
            //     .arg("-c 4")
            //     .arg("192.168.0.24")
            //     .status(),
            _ => panic!("Unsupported OS"),
        } {
            Ok(status) => {
                if status.success() {
                    println!("Success!");
                }
            }
            Err(e) => {
                println!("Failed to execute process: {}", e);
            }
        }
    });

    // let peers: Vec<String> = Vec::new();
    // for ip in &ips {
    //     let conn = TcpStream::connect(format!("{}:{}", ip, 25569));
    //     if conn.is_err() {
    //         continue;
    //     }
    //     let mut conn = conn.unwrap();
    //     conn.write_all(b"?PD").expect("Could not write");
    //     let mut buffer = [0; 1024];
    //     conn.read(&mut buffer).expect("Could not read");
    // }
    scanner.join().unwrap();
    return ips;
}

async fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", 25569)).expect("Could not bind");
    println!("Server started!");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    stream.write(b"Hello Client!").expect("Could not write");

    loop {
        let mut buffer = [0; 1024];

        let res = stream.read(&mut buffer);
        if res.is_err() {
            println!("Client disconnected");
            return;
        }

        // rspond with: echo: {buffer}
        stream
            .write(format!("Received: {}", String::from_utf8_lossy(&buffer)).as_bytes())
            .expect("Could not write");
        println!("Request: {}\n", String::from_utf8_lossy(&buffer[..]));
    }
}
