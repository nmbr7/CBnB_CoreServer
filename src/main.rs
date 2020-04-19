#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate log;
extern crate redis;
extern crate uuid;

mod api;
mod database;
mod message;
mod node;

use diesel::prelude::*;
use dotenv::dotenv;
use log::{info, warn};

use std::env;

use self::database::models::{NewNodeState, NewResources, Node, NodeState, Resources};
use self::database::schema;

use std::sync::mpsc;
use std::thread;
//use std::time::Duration;
use api::{client_api_main, server_api_main};

fn main() -> () {
    env_logger::init();
    let (client_tx, client_rx) = mpsc::channel();
    let (server_tx, server_rx) = mpsc::channel();
    let _server_thread = thread::spawn(move || {
        server_api_main(server_tx, client_tx);
    });
    let _client_thread = thread::spawn(move || {
        client_api_main(client_rx);
    });

    loop {
        let received = server_rx.try_recv();
        match received {
            Ok(s) => {
                info!("Received from Node Client: {}", &s);
                //let ip = s; // Get Node client IP address from the core server api
                //thread::sleep(Duration::from_secs(2));
                //let addr = format!("{}:7777", ip);
                //client_tx.send(addr).unwrap();

                // let addr = String::from("127.0.0.1:6666");
                // client_tx.send(addr).unwrap();
            }
            Err(_) => (),
        };
        //  break;
    }
    //_client_thread.join().unwrap();
    //_server_thread.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    /*    #[test]
        fn run() {
            dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let conn = MysqlConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
            //    println!("{:?}",a);

            use schema::Nodes::dsl::*;
            use schema::Node_resources::dsl::*;
            use schema::Node_state::dsl::*;

            let sn = Nodes.order(id.desc()).first::<Node>(&conn).unwrap().id;
            //println!("{:?}",sn);
            let id1: i32 = sn+1;

            let node = NewNode {
                id:  id1,
                ip: &String::from("127.0.0.1"),
            };

            let result = diesel::insert_into(Nodes)
                .values(node)
                .execute(&conn)
                .expect("Error saving new post");

            let ns = NewNodeState {
                node_id: id1,
                uptime: &String::from("10 hours"),
                power_mode: &String::from("Battery"),
            };
            //let v = vec![&ns; 32767];

            let result = diesel::insert_into(Node_state)
                .values(ns)
                .execute(&conn)
                .expect("Error saving new post");

            let res = NewResources {
                node_id: id1,
                mem_total: &String::from("4GB"),
                mem_usage: &String::from("20%"),
                net_speed_up: &String::from("5mbps"),
                net_speed_down: &String::from("10mbps"),
                net_ciface: &String::from("wlp2s0"),
                cpu_cores: 4 as i32,
                cpu_usage: &String::from("30%"),
                cpu_model: &String::from("Intel"),
            };

            let result = diesel::insert_into(Node_resources)
                .values(res)
                .execute(&conn)
                .expect("Error saving new post");

            //let v = vec![new_post,new_post1];
            let me = Nodes
                .find(10)
                .first::<Node>(&conn)
                .expect("Error loading user");

            let my_photos = Resources::belonging_to(&me)
                .load::<Resources>(&conn)
                .expect("Error loading photos");

            let my_photos1 = Node_state
                .order(s_id)
                .load::<NodeState>(&conn)
                .expect("Error loading photos");

            println!("");

            for i in my_photos {
                println!("{:?}", i);
            }

            //let results = Node.filter(id.eq(1)).limit(1).load::<node>(&conn).expect("Error");
            //for i in results {
            //  println!("{}",i.id);
            //    users::table.order(users::id.desc()).first(&conn).unwrap();
            //}
        }
    */
}
