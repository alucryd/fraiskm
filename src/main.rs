extern crate async_graphql;
extern crate async_graphql_tide;
extern crate async_std;
extern crate async_trait;
extern crate base64;
extern crate bigdecimal;
extern crate blake3;
extern crate chrono;
extern crate clap;
extern crate dotenv;
extern crate env_logger;
extern crate http_types;
extern crate mailchecker;
extern crate rayon;
extern crate ring;
extern crate rust_embed;
extern crate serde;
extern crate sqlx;
extern crate surf;
extern crate tide;
extern crate tindercrypt;
extern crate uuid;

mod database;
mod model;
mod mutation;
mod query;
mod server;
mod util;
mod validator;

use clap::App;
use dotenv::dotenv;

#[async_std::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let subcommands = vec![server::subcommand()];
    let matches = App::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommands(subcommands)
        .get_matches();

    if matches.subcommand.is_some() {
        if let Some("server") = matches.subcommand_name() {
            server::main(matches.subcommand_matches("server").unwrap()).await
        }
    }
}
