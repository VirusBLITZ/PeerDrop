// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;


use std::io::Write;
use std::net::{Ipv4Addr, TcpListener, TcpStream};

use std::sync::mpsc;
use std::{env, thread};
use std::{fs::File, io::Read};


use libarp::client::ArpClient;
use libarp::interfaces::Interface;
use tauri::async_runtime::block_on;

fn main() {
    // println!("Peers: {}", scan_peers().join("\n"));
    // let server = start_server().await;
    thread::spawn(|| {
        let _ = block_on(listen());
    });


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
    let mut ips: Vec<String> = vec![];

    let (tx, rx) = mpsc::channel::<String>();

    // get subnet mask
    let own_ip = Interface::new().unwrap().get_ip().unwrap();
    for i in 0..255 {
        let ip = Ipv4Addr::new(
            own_ip.octets()[0],
            own_ip.octets()[1],
            own_ip.octets()[2],
            i,
        );
        let tx = tx.clone();
        thread::spawn(move || {
            let mut client = ArpClient::new().unwrap();
            let res = block_on(client.ip_to_mac(ip, None));
            if res.is_ok() {
                tx.send(ip.to_string()).unwrap();
            }
        });
    }

    while let Ok(ip) = rx.recv() {
        println!("Found peer: {}", ip);
        ips.push(ip);
    }
    
    ips
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
