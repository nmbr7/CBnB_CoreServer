-- Your SQL goes here
CREATE TABLE Node_resources (
    node_id VARCHAR(128) PRIMARY KEY REFERENCES Node(id),
    mem_total VARCHAR(20) NOT NULL,
    mem_usage VARCHAR(20) NOT NULL,
    mem_free VARCHAR(20) NOT NULL,
    mem_available VARCHAR(20) NOT NULL,
    net_speed_up VARCHAR(20) NOT NULL,
    net_speed_down VARCHAR(20) NOT NULL,
    net_ciface VARCHAR(20) NOT NULL,
    cpu_cores INT NOT NULL,
    cpu_usage VARCHAR(20) NOT NULL,
    cpu_model VARCHAR(100) NOT NULL
);
