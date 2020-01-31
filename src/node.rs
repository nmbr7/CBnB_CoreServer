use crate::database::dbfunc;
use crate::database::models::{
    NewNodeState, NewResources, Node, NodeState, Resources, UpdateNodeState, UpdateResources,
};
use crate::database::schema;
use crate::message::{Message, MsgType, NodeResources, StatUpdate};
use redis::Commands;
use schema::Node_resources::dsl::*;
use schema::Node_state::dsl::*;
use schema::Nodes::dsl::*;

use diesel::prelude::*;
use uuid::Uuid;

// TODO Move this  macro to the dbfunc.rs file properly
macro_rules! insert_into {
    ($conn:expr, $table:expr, $val:expr) => {
        diesel::insert_into($table)
            .values($val)
            .execute($conn)
            .expect("Error Inserting to table");
    };
}

macro_rules! update_table {
    ($conn:expr, $table:expr, $val:expr, $uuid:expr) => {
        diesel::update($table.find($uuid))
            .set($val)
            .execute($conn)
            .expect("Error updating the table");
    };
}

pub fn update(stat: StatUpdate) -> () {
    //let res_update_model =

    //    let nodeid = Uuid::new_v4().to_string() ;
    //    let node_model = Node {
    // TODO NOT IMPLEMENTED IN THE NODE
    //ip: &node_ip,
    //    };
    let client = redis::Client::open("redis://172.28.5.3/").unwrap();
    let mut con = client.get_connection().unwrap();
    let nodeid: String = con.get(&stat.uuid).unwrap();

    let res_model = UpdateResources {
        node_id: &nodeid,
        mem_usage: &stat.mem_usage.1,
        mem_free: &stat.mem_free,
        mem_available: &stat.mem_available,
        // TODO Write a TO/FROM impl for storing net_speed as a single tuple
        net_speed_up: &stat.net.speed.0,
        net_speed_down: &stat.net.speed.1,
        net_ciface: &stat.net.current_interface,
        cpu_usage: &stat.cpu_usage,
    };

    let state_model = UpdateNodeState {
        node_id: &nodeid,
        uptime: &stat.uptime,
        // TODO NOT IMPLEMENTED IN THE NODE
        power_mode: &String::from("Battery"),
    };

    let conn = dbfunc::establish_connection();
    update_table!(&conn, Node_resources, res_model, &nodeid);
    update_table!(&conn, Node_state, state_model, &nodeid);
}

pub fn register(res: NodeResources, node_ip: String) -> () {
    let nodeid = Uuid::new_v4().to_string();
    let client = redis::Client::open("redis://172.28.5.3/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _: () = con.set(&res.uuid, &nodeid).unwrap();
    let node_model = Node {
        id: &nodeid,
        // TODO NOT IMPLEMENTED IN THE NODE
        ip: &node_ip,
    };

    let res_model = NewResources {
        node_id: &nodeid, // Default node_id set to 0 (updated based on the inserted node id)
        mem_usage: &res.mem.usage.1,
        mem_total: &res.mem.total,
        mem_free: &res.mem.free,
        mem_available: &res.mem.available,
        // TODO NOT IMPLEMENTED IN THE NODE
        cpu_cores: 4 as i32,
        // TODO Write a TO/FROM impl for storing net_speed as a single tuple
        net_speed_up: &res.net.speed.0,
        net_speed_down: &res.net.speed.1,
        net_ciface: &res.net.current_interface,
        cpu_usage: &res.cpu.usage,
        cpu_model: &res.cpu.model,
    };

    let state_model = NewNodeState {
        node_id: &nodeid, // Default node_id set to 0 (updated based on the inserted node id)
        uptime: &res.uptime,
        // TODO NOT IMPLEMENTED IN THE NODE
        power_mode: &String::from("Battery"),
    };

    #[macro_use]
    use crate::database::dbfunc;
    let conn = dbfunc::establish_connection();
    insert_into!(&conn, Nodes, node_model);
    insert_into!(&conn, Node_resources, res_model);
    insert_into!(&conn, Node_state, state_model);
    ()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let nodeid = Uuid::new_v4().to_string();
        println!("{}", nodeid); //  println!("{:?}",Message::new(MsgType::REGISTER,stat))
                                //println!("{}", Message::<sys_stat::NodeResources>::register(stat))
    }
}
