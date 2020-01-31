use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::database::models::{NewNodeState, NewResources, Node, NodeState, Resources};
use crate::database::schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    MysqlConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error Connecting to db url {}", db_url))
}

/*

#[macro_export]
macro_rules! execute {
    ($conn:expr, $table:expr, $val:expr) => {
        diesel::insert_into($table)
            .values($val)
            .execute($conn)
            .expect("Error Inserting to table");
    };
}

*/
