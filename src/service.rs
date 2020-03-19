use std::collections::HashMap;

struct VM<'a> {
    pub service_id: &'a str,
    pub node_id: &'a str,
}
struct Storage<'a> {
    pub service_id: &'a str,
    pub node_id: &'a str,
}
struct Docker<'a> {
    pub service_id: &'a str,
    pub node_id: &'a str,
}
struct Fas<'a> {
    pub service_id: &'a str,
    pub node_id: &'a str,
    //pub invocations: i32,
    //pub frequency: i32,
    //pub creating_date: i32,
    //pub stat: &'a str, //published or not
}

struct Service {
    pub vms: HashMap<String, &VM>,
    pub storages: HashMap<String, &Storage>,
    pub dockersapps: HashMap<String, &Docker>,
    pub faas: HashMap<String, &Fas>,
}
