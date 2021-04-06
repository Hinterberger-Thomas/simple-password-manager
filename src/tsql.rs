extern crate mysql_async;

use mysql_async::prelude::*;
use crate::{conn::ConnMut, Column, Conn, Error, Result, Row, Value};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[tokio::main]
pub async fn init_db() -> Result<()>{


    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    let database_url = "/* ... */";

    let pool = mysql_async::Pool::new(database_url);
    let mut conn = pool.get_conn().await?

    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )"
    ).await;

    return Ok(true);
}
