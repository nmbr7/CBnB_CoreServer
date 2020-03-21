use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::thread;

use serde_json::{json, Result, Value};

//use crate::node::Node;
use crate::message::{
    Message, NodeMsgType, NodeResources, ServiceMsgType, ServiceType, StatUpdate,
};
use crate::node;

fn server_api_handler(
    mut stream: TcpStream,
    server_dup_tx: mpsc::Sender<String>,
    client_dup_tx: mpsc::Sender<String>,
    data: (String),
) -> () {
    //println!("{:?} and {:?}",stream,server_dup_tx);
    let mut buffer = [0; 1512];

    //Read IP
    let no = stream.read(&mut buffer).unwrap();
    stream.write_all("OK".as_bytes()).unwrap();
    stream.flush().unwrap();
    let source_ip = std::str::from_utf8(&buffer[0..no]).unwrap().to_string();
    println!("Received conn from node IP :- {} via ()\n", &source_ip);

    //let buf = buffer.trim_matches(char::from(0));
    //let mut reader = BufReader::new(stream);
    //let lines = reader.lines();
    //let v = lines.map(|l| l.expect("Parse Fail")).collect()

    //let r = format!("{}", String::from_utf8_lossy(&buffer[0..no]));
    //let a = buffer[0..no].split("_:_").map(|l| l.to_string()).collect::<Vec<String>>();
    let no = stream.read(&mut buffer).unwrap();
    let recv_data: Message = serde_json::from_slice(&buffer[0..no]).unwrap();
    match recv_data {
        Message::Node(node) => match node.msg_type {
            NodeMsgType::REGISTER => {
                let rc: NodeResources = serde_json::from_str(&node.content).unwrap();
    //            println!("REGISTER\n{:?}", rc);
                node::register(rc, source_ip);
            }
            NodeMsgType::UPDATE_SYSTAT => {
                let rc: StatUpdate = serde_json::from_str(&node.content).unwrap();
      //          println!("UPDATE_SYSSTAT\n{:?}", rc);
                node::update(rc);
            }
        },
        Message::Service(service) => match service.msg_type {
            ServiceMsgType::SERVICEINIT => match service.service_type {
                ServiceType::Faas => {
                    let content: Value = serde_json::from_str(&service.content.as_str()).unwrap();
                    println!("{:?}", content);
                    match content["request"].as_str().unwrap() {
                        "select_node" => {
                            // Query database to select node

                            let msg = json!({
                                "response" : {
                                    "node_ip" : "127.0.0.1:7777",
                                }
                            })
                            .to_string();

                            stream.write_all(&msg.as_bytes());
                            stream.flush().unwrap();
                        }
                        _ => {}
                    }
                }
            },
            ServiceMsgType::SERVICESTART => {}
            ServiceMsgType::SERVICESTOP => {}
            ServiceMsgType::SERVICEUPDATE => {}
        },
    };
}

fn client_api_handler(mut stream: TcpStream) -> () {
    // println!("{:?}",stream);
    stream.write_all("sdaf".as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn server_api_main(server_tx: mpsc::Sender<String>, client_tx: mpsc::Sender<String>) -> () {
    let listener = TcpListener::bind("0.0.0.0:7778").unwrap();
    println!("Waiting for connections");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let data = (stream.peer_addr().unwrap().to_string());

        // In case of browser there may be multiple requests for fetching
        // different file in a page
        let client_dup_tx = mpsc::Sender::clone(&client_tx);
        let server_dup_tx = mpsc::Sender::clone(&server_tx);
        thread::spawn(move || {
            server_api_handler(stream, server_dup_tx, client_dup_tx, data);
        });
    }
}

pub fn client_api_main(client_rx: mpsc::Receiver<String>) -> () {
    //let client_dup_rx = mpsc::Sender::clone(&client_rx);
    for received in client_rx {
        let stream = TcpStream::connect(received).unwrap();
        thread::spawn(move || {
            client_api_handler(stream);
        });
    }
}
