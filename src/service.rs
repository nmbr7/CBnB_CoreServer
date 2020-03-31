use std::collections::HashMap;

struct Storage<'a> {
    pub service_id: &'a str,
    pub node_id: &'a str,
}
struct Paas<'a> {
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
    pub storages: HashMap<String, &Storage>,
    pub paas: HashMap<String, &Paas>,
    pub faas: HashMap<String, &Fas>,
}
