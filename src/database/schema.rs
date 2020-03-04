table! {
    Node_resources (node_id) {
        node_id -> Varchar,
        mem_total -> Varchar,
        mem_usage -> Varchar,
        mem_free -> Varchar,
        mem_available -> Varchar,
        net_speed_up -> Varchar,
        net_speed_down -> Varchar,
        net_ciface -> Varchar,
        cpu_cores -> Integer,
        cpu_usage -> Varchar,
        cpu_model -> Varchar,
    }
}
