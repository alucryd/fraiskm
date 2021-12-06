extern crate async_graphql;
extern crate async_graphql_tide;
extern crate async_std;
extern crate async_trait;
extern crate base64;
extern crate blake3;
extern crate clap;
extern crate dotenv;
extern crate http_types;
extern crate rayon;
extern crate ring;
extern crate rust_embed;
extern crate serde;
extern crate sqlx;
extern crate surf;
extern crate tide;
extern crate tindercrypt;

mod database;
mod model;
mod mutation;
mod query;
mod server;
mod util;

use clap::App;
use database::*;
use dotenv::dotenv;
use simple_error::SimpleError;

type SimpleResult<T> = Result<T, SimpleError>;

#[async_std::main]
async fn main() -> SimpleResult<()> {
    let subcommands = vec![server::subcommand()];
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
