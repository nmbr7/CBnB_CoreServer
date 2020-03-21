-- Your SQL goes here
CREATE TABLE Node_resources (
    node_id VARCHAR(128) PRIMARY KEY REFERENCES Node(id),
    mem_total DOUBLE NOT NULL,
    mem_usage DOUBLE NOT NULL,
    mem_free DOUBLE NOT NULL,
    mem_available DOUBLE NOT NULL,
    net_speed_up DOUBLE NOT NULL,
    net_speed_down DOUBLE NOT NULL,
    net_ciface VARCHAR(20) NOT NULL,
    cpu_cores INT NOT NULL,
    cpu_usage DOUBLE NOT NULL,
    cpu_model VARCHAR(100) NOT NULL
);
