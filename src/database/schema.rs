table! {
    Node_resources (node_id) {
        node_id -> Varchar,
        mem_total -> Double,
        mem_usage -> Double,
        mem_free -> Double,
        mem_available -> Double,
        net_speed_up -> Double,
        net_speed_down -> Double,
        net_ciface -> Varchar,
        cpu_cores -> Integer,
        cpu_usage -> Double,
        cpu_model -> Varchar,
    }
}

table! {
    Node_state (node_id) {
        node_id -> Varchar,
        uptime -> Varchar,
        power_mode -> Varchar,
    }
}

table! {
    Nodes (id) {
        id -> Varchar,
        ip -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(Node_resources, Node_state, Nodes,);
