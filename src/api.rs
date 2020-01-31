use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::sync::mpsc;
use std::thread;

//use crate::node::Node;
use crate::message::{Message, MsgType, NodeResources, StatUpdate};
use crate::node;

fn server_api_handler(
    mut stream: TcpStream,
    server_dup_tx: mpsc::Sender<String>,
    data: (String),
) -> () {
    //println!("{:?} and {:?}",stream,server_dup_tx);
    let mut buffer = [0; 512];
    let no = stream.read(&mut buffer).unwrap();
    //let buf = buffer.trim_matches(char::from(0));
    //let mut reader = BufReader::new(stream);
    //let lines = reader.lines();
    //let v = lines.map(|l| l.expect("Parse Fail")).collect()

    //let r = format!("{}", String::from_utf8_lossy(&buffer[0..no]));
    //let a = buffer[0..no].split("_:_").map(|l| l.to_string()).collect::<Vec<String>>();
    let recv_data: Message<String> = serde_json::from_slice(&buffer[0..no]).unwrap();
    //println!("{:?}", recv_data);
    match recv_data.msg_type {
        MsgType::REGISTER => {
            let rc: NodeResources = serde_json::from_str(&recv_data.content).unwrap();
            node::register(rc, data);
            //println!("REGISTER\n{:?}", rc);
        }
        MsgType::UPDATE_SYSTAT => {
            let rc: StatUpdate = serde_json::from_str(&recv_data.content).unwrap();
            node::update(rc);
            //println!("UPDATE_SYSSTAT\n{:?}", rc);
        }
    };

    //  let recv_data: Message::<NodeResources> = serde_json::from_slice(&buffer[0..no]).unwrap();

    /*
    let get = b"[[Register Node]]--";
    if buffer.starts_with(get) {
        //create new node;
        //        let n = Node::new();
        //register node;
        //        Node::register(&n);

        let put = b"New NodeClient Registered--";
        stream.write(put).unwrap();
        stream.flush().unwrap();
        server_dup_tx
            .send(format!(
                "{}",
                str::from_utf8(&buffer)
                    .unwrap()
                    .split("--")
                    .collect::<Vec<&str>>()[0]
            ))
            .unwrap();
    }
    */
}

fn client_api_handler(mut stream: TcpStream) -> () {
    // println!("{:?}",stream);
    let put = b"Hello from server--";
    stream.write(put).unwrap();
    stream.flush().unwrap();
}

pub fn server_api_main(server_tx: mpsc::Sender<String>) -> () {
    let listener = TcpListener::bind("0.0.0.0:7778").unwrap();
    println!("Waiting for connections");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let data = (stream.peer_addr().unwrap().to_string());
        println!("Received connection from IP :- {}", &data);

        // In case of browser there may be multiple requests for fetching
        // different file in a page
        let server_dup_tx = mpsc::Sender::clone(&server_tx);
        thread::spawn(move || {
            server_api_handler(stream, server_dup_tx, data);
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
