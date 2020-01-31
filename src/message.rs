use serde::{Deserialize, Serialize};
use serde_json::Result;

struct VM {}
struct Storage {}
struct Docker {}
struct Fas {}

struct service {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatUpdate {
    pub uuid: String,
    pub cpu_usage: String,
    pub mem_usage: (String, String),
    pub mem_free: String,
    pub mem_available: String,
    pub net: NetInfo,
    pub uptime: String,
    //ram: u64,
    //cpu_frequency: f32,
    //core_count: u9,
    //net_speed_up: u64,
    //net_speed_down: f32,
    //disk_storage: f32,
    //gpu: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResources {
    pub uuid: String,
    pub cpu: CpuInfo,
    pub mem: MemInfo,
    pub net: NetInfo,
    pub uptime: String,
    //ram: u64,
    //cpu_frequency: f32,
    //core_count: u9,
    //net_speed_up: u64,
    //net_speed_down: f32,
    //disk_storage: f32,
    //gpu: bool,
}

// Struct to store the cpu details of the system - output similar to lscpu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    //arch: String,
    //op_model: String,
    //byte_order: String,
    //cpus: u8,
    //virtualization: String,
    pub model: String,
    pub cputime: (i64, i64),
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetInfo {
    //interfaces: String,
    pub current_interface: String,
    pub speed: (String, String),
    //ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemInfo {
    pub usage: (String, String),
    pub total: String,
    //used: String,
    pub free: String,
    pub available: String,
    //swap: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgType {
    // Received from the node
    REGISTER,
    UPDATE_SYSTAT,
    // Send to the node
    // CHECKSYSTAT,
    // SERVICESTART,
    // SERVICESTOP,
    // CUSTOM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<T> {
    pub msg_type: MsgType,
    pub content: T, //sys_stat::Resources,
}

impl<T> Message<T> {
    fn new(msg_type: MsgType, content: T) -> Self {
        Self { msg_type, content }
    }
    //pub fn parse(data: [u8; 512]) -> Result<T> {
    //let msg: Message::<>= serde_json::from_slice(&data)?;
    //serde_json::to_string(&msg).unwrap()
    //Ok(msg)
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        //  println!("{:?}",Message::new(MsgType::REGISTER,stat))
        //println!("{}", Message::<sys_stat::NodeResources>::register(stat))
    }
}
