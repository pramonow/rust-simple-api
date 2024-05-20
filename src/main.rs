mod entity;

use axum::{extract::Query, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;
use mysql::*;
use entity::Employee;
use mysql::prelude::*;

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct EmployeeParam {
    id: i64,
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new().route("/", get(handler))
        .route("/employee", get(fetch_employee));
    

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn fetch_employee(param: Query<EmployeeParam>) -> String {
    let mut conn = initialise_db();
    let farm = get_farm(&mut conn, param.id);

    format!("{:?}", farm)
}


async fn handler(Query(range): Query<RangeParameters>) -> String {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Use format! to create a dynamically allocated string.
    format!("Random number {}", random_number)
}

fn initialise_db() -> PooledConn {
    let url = "mysql://root:password@localhost:3306/test";
    let pool = Pool::new(url).unwrap();
    //creating a connection
    let conn = pool.get_conn().unwrap();
    conn
}

fn get_farm(conn: &mut PooledConn, id:i64) -> Vec<entity::Employee> {
    let y=format!("select id, name from employee where id= {}",id);

    let res = conn.query_map(
            y,
            |(
                id,
                name
            )| Employee {
                id: id,
                name: name,
            },
        )
        .expect("Query failed.");

    res
}