use persistent;
use iron;
use iron::prelude::*;
use iron::status;
//use router::Router;
use r2d2;
use r2d2_postgres;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2::{Pool, PooledConnection};

#[derive(Copy, Clone)]
pub struct DbPool;
impl iron::typemap::Key for DbPool {
    type Value = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
}