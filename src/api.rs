use log::{info, warn};
use serde_json::{json, Result, Value};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

//use crate::node::Node;
use crate::message::{
    Message, NodeMsgType, NodeResources, ServiceMessage, ServiceMsgType, ServiceType, StatUpdate,
};
use crate::node;

fn server_api_handler(
    mut stream: TcpStream,
    server_dup_tx: mpsc::Sender<String>,
    client_dup_tx: mpsc::Sender<(String, String)>,
    proxycount: Arc<Mutex<[u8; 2]>>,
    data: (String),
) -> () {
    //println!("{:?} and {:?}",stream,server_dup_tx);
    let mut buffer = [0; 1512];

    //Read IP
    let no = stream.read(&mut buffer).unwrap();
    stream.write_all("OK".as_bytes()).unwrap();
    stream.flush().unwrap();
    let source_ip = std::str::from_utf8(&buffer[0..no]).unwrap().to_string();
    info!(
        "Received conn from node IP :- {} via ({})\n",
        &source_ip, data
    );

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
            NodeMsgType::PROXY_REGISTRATION => {
                let rc: Value = serde_json::from_str(&node.content).unwrap();
                //            println!("REGISTER\n{:?}", rc);
                //TODO Register the proxy and insert the data in the DB
                //node::register(rc, source_ipi);
                let mut nmode = 0;
                {
                    let mut proxycountval = proxycount.lock().unwrap();
                    if proxycountval[0] == 0 {
                        proxycountval[0] += 1;
                        nmode = 0;
                    } else if proxycountval[1] == 0 {
                        proxycountval[1] += 1;
                        nmode = 1;
                    } else {
                        proxycountval[0] += 1;
                        nmode = 0;
                    }
                }
                let msg = json!({
                    "response" : "OK",
                        "mode" : nmode
                })
                .to_string();

                stream.write_all(&msg.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            NodeMsgType::REGISTER => {
                let rc: NodeResources = serde_json::from_str(&node.content).unwrap();
                //            println!("REGISTER\n{:?}", rc);
                let memory = rc.mem.total;
                let addr = format!("{}:7777", &source_ip.split(':').collect::<Vec<&str>>()[0]);
                node::register(rc, source_ip);
                if memory > 3.072 {
                    //thread::sleep(Duration::from_secs(2));
                    client_dup_tx.send((addr, data)).unwrap();
                    // client_tx.send(addr).unwrap();
                }
            }
            NodeMsgType::UPDATE_SYSTAT => {
                let rc: StatUpdate = serde_json::from_str(&node.content).unwrap();
                //          println!("UPDATE_SYSSTAT\n{:?}", rc);
                node::update(rc);
            }
        },
        Message::Service(service) => {
            let content: Value = serde_json::from_str(&service.content.as_str()).unwrap();
            println!("{:?}", content);

            match service.msg_type {
                ServiceMsgType::SERVICEINIT => match service.service_type {
                    ServiceType::Faas => {
                        match content["request"].as_str().unwrap() {
                            "select_node" => {
                                // Query database to select node
                                let nodes = node::allocate_node();
                                println!("{:?}", nodes);
                                let msg = json!({
                                    "response" : {
                                        "node_ip" : nodes,
                                    }
                                })
                                .to_string();

                                stream.write_all(&msg.as_bytes()).unwrap();
                                stream.flush().unwrap();
                            }
                            _ => {}
                        }
                    }
                    ServiceType::Paas => {
                        match content["request"].as_str().unwrap() {
                            "select_node" => {
                                // Query database to select node
                                let nodes = node::allocate_node();
                                let msg = json!({
                                    "response" : {
                                        "node_ip" : nodes,
                                    }
                                })
                                .to_string();

                                stream.write_all(&msg.as_bytes()).unwrap();
                                stream.flush().unwrap();
                            }
                            _ => {}
                        }
                    }
                    ServiceType::Storage => {
                        match content["request"].as_str().unwrap() {
                            "select_node" => {
                                // Query database to select node
                                let nodes = node::allocate_node();
                                let msg = json!({
                                    "response" : {
                                        "node_ip" : nodes,
                                    }
                                })
                                .to_string();

                                stream.write_all(&msg.as_bytes()).unwrap();
                                stream.flush().unwrap();
                            }
                            _ => {}
                        }
                    }
                },
                ServiceMsgType::SERVICEUPDATE => {}
                ServiceMsgType::SERVICEMANAGE => {}
            }
        }
    }
}

fn client_api_handler(mut stream: TcpStream, node_addr: String) -> () {
    // println!("{:?}",stream);
    let coreserver_uuid = format!("Coreserver_unique_uuid");
    let msgcontent = Message::Service(ServiceMessage {
        uuid: coreserver_uuid,
        msg_type: ServiceMsgType::SERVICEMANAGE,
        service_type: ServiceType::Paas,
        content: json!({
            "node_ip"  : node_addr,
            "msg_type" : "start",
        })
        .to_string(),
    });
    info!("Senting Qemu start message");
    let msg = serde_json::to_string(&msgcontent)
        .unwrap()
        .as_bytes()
        .to_owned();
    stream.write_all(&msg).unwrap();
    stream.flush().unwrap();
}

pub fn server_api_main(
    server_tx: mpsc::Sender<String>,
    client_tx: mpsc::Sender<(String, String)>,
) -> () {
    let listener = TcpListener::bind("0.0.0.0:7778").unwrap();
    info!("Waiting for connections");
    let service_root = Arc::new(Mutex::new([0; 2]));
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let data = (stream
            .peer_addr()
            .unwrap()
            .to_string()
            .split(":")
            .collect::<Vec<&str>>()[0]
            .to_string());
        let proxycnt = Arc::clone(&service_root);

        // In case of browser there may be multiple requests for fetching
        // different file in a page
        let client_dup_tx = mpsc::Sender::clone(&client_tx);
        let server_dup_tx = mpsc::Sender::clone(&server_tx);
        thread::spawn(move || {
            server_api_handler(stream, server_dup_tx, client_dup_tx, proxycnt, data);
        });
    }
}

pub fn client_api_main(client_rx: mpsc::Receiver<(String, String)>) -> () {
    //let client_dup_rx = mpsc::Sender::clone(&client_rx);
    for (node_addr, proxy_addr) in client_rx {
        let proxy_addr = format!("{}:7779", proxy_addr);
        let stream = TcpStream::connect(proxy_addr).unwrap();
        thread::spawn(move || {
            client_api_handler(stream, node_addr);
        });
    }
}
