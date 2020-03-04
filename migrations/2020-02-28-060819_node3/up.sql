-- Your SQL goes here

CREATE TABLE Node_state (
    node_id VARCHAR(128) PRIMARY KEY REFERENCES Node(id),
    uptime VARCHAR(20) NOT NULL,
    power_mode VARCHAR(20) NOT NULL 
);
