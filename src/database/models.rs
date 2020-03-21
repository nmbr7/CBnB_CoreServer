/*
   TODO
   ----
   This file needs to be completely revised; since it tries to match the    \
   schema of the table explicitly. ToSql and FromSql traits needs to be    \
   implemented on the structs in the src/nodes.rs file which is the real   \
   structure of all the structs in this file, so that we can directly read \
   and write to the database without using explicit foreign keyed structs!
*/

use crate::schema::{Node_resources, Node_state, Nodes};

//###############################################################
#[derive(Debug, Queryable, Associations, Identifiable)]
#[primary_key(node_id)]
#[table_name = "Node_resources"]
#[belongs_to(parent = "Node")]
pub struct Resources {
    pub node_id: String,
    mem_total: f64,
    mem_usage: f64,
    mem_free: f64,
    mem_available: f64,
    net_speed_up: f64,
    net_speed_down: f64,
    net_ciface: String,
    cpu_cores: i32,
    cpu_usage: f64,
    cpu_model: String,
    //disk_storage: f32,
    //gpu: bool,
    // Add additional fields like VM count, docker machine details and details
    // about the services running on the node.
    //fitness: f32,
}

#[table_name = "Node_resources"]
#[derive(Insertable)]
pub struct NewResources<'a> {
    pub node_id: &'a str,
    pub mem_total: f64,
    pub mem_usage: f64,
    pub mem_free: f64,
    pub mem_available: f64,
    pub net_speed_up: f64,
    pub net_speed_down: f64,
    pub net_ciface: &'a str,
    pub cpu_cores: i32,
    pub cpu_usage: f64,
    pub cpu_model: &'a str,
    //disk_storage: f32,
    //gpu: bool,
    // Add additional fields like VM count, docker machine details and details
    // about the services running on the node.
    //fitness: f32,
}

#[table_name = "Node_resources"]
#[derive(AsChangeset)]
pub struct UpdateResources<'a> {
    pub node_id: &'a str,
    pub mem_usage: f64,
    pub mem_free: f64,
    pub mem_available: f64,
    pub net_speed_up: f64,
    pub net_speed_down: f64,
    pub net_ciface: &'a str,
    pub cpu_usage: f64,
    //disk_storage: f32,
    //gpu: bool,
    // Add additional fields like VM count, docker machine details and details
    // about the services running on the node.
    //fitness: f32,
}
//###############################################################
#[derive(Debug, Queryable, Associations, Identifiable)]
#[table_name = "Node_state"]
#[primary_key(node_id)]
#[belongs_to(parent = "Node")]
pub struct NodeState {
    pub node_id: String,
    uptime: String,
    power_mode: String,
}
// TODO Remove unwanted code here
#[table_name = "Node_state"]
#[derive(Insertable)]
pub struct NewNodeState<'a> {
    pub node_id: &'a str,
    pub uptime: &'a str,
    pub power_mode: &'a str,
}

#[table_name = "Node_state"]
#[derive(AsChangeset)]
pub struct UpdateNodeState<'a> {
    pub node_id: &'a str,
    pub uptime: &'a str,
    pub power_mode: &'a str,
}

//###############################################################
#[derive(Debug,  Insertable, Queryable, Associations, Identifiable)]
#[table_name = "Nodes"]
pub struct NewNode<'a> {
    pub id: &'a str,
    pub ip: &'a str,
    //    location: Option<String>,
    //    score: Option<f32>, //to rate a node
}

#[derive(Debug, Insertable, Queryable, Associations, Identifiable)]
#[table_name = "Nodes"]
pub struct Node {
    pub id: String,
    pub ip: String,
    //    location: Option<String>,
    //    score: Option<f32>, //to rate a node
}
/*
#[table_name = "Nodes"]
#[derive(Insertable)]
pub struct NewNode<'a> {
    pub id: &'a str,
    pub ip: &'a str,
}*/
//###############################################################
/*
impl Node {
    pub fn new() -> Node {
        unimplemented!("Create new node");
    }

    pub fn update() -> bool {
        unimplemented!("Update node");
    }

    pub fn register(&self) -> usize {
        unimplemented!("Register node");
    }
}

*/
