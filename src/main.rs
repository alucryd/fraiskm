extern crate async_ctrlc;
extern crate async_graphql;
extern crate async_graphql_tide;
extern crate async_std;
extern crate async_trait;
extern crate blake3;
extern crate clap;
extern crate dotenv;
extern crate http_types;
extern crate lazy_static;
extern crate once_cell;
extern crate rayon;
extern crate rust_embed;
extern crate serde;
extern crate sqlx;
extern crate surf;
extern crate tide;
extern crate tindercrypt;

mod database;
mod model;
mod server;

use clap::App;
use database::*;
use dotenv::dotenv;
use simple_error::SimpleError;

type SimpleResult<T> = Result<T, SimpleError>;

#[async_std::main]
#[allow(unused_mut)]
async fn main() -> SimpleResult<()> {
    let mut subcommands = vec![server::subcommand()];
    let matches = App::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommands(subcommands)
        .get_matches();

    if matches.subcommand.is_some() {
        dotenv().ok();

        let pool = establish_connection(env!("DATABASE_URL")).await;

        match matches.subcommand_name() {
            Some("server") => {
                server::main(pool, &matches.subcommand_matches("server").unwrap()).await?
            }
            _ => (),
        }
    }

    Ok(())
}
