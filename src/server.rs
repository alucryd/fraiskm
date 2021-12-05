extern crate async_ctrlc;
extern crate async_graphql;
extern crate async_graphql_tide;
extern crate async_trait;
extern crate http_types;
extern crate rust_embed;
extern crate tide;

use super::database::*;
use super::model::*;
use async_ctrlc::CtrlC;
use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Error, Object, Result, Schema,
};
use async_std::path::Path;
use async_std::prelude::FutureExt;
use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use futures::stream::TryStreamExt;
use http_types::mime::BYTE_STREAM;
use http_types::{Mime, StatusCode};
use itertools::Itertools;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use rust_embed::RustEmbed;
use simple_error::SimpleResult;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

lazy_static! {
    static ref POOL: OnceCell<PgPool> = OnceCell::new();
}

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("server")
        .about("Launches the backend server")
        .arg(
            Arg::with_name("ADDRESS")
                .short("a")
                .long("address")
                .help("Specifies the server address")
                .required(false)
                .takes_value(true)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name("PORT")
                .short("p")
                .long("port")
                .help("Specifies the server port")
                .required(false)
                .takes_value(true)
                .default_value("8000"),
        )
}

pub struct UserLoader;

#[async_trait]
impl Loader<Uuid> for UserLoader {
    type Value = User;
    type Error = Error;

    async fn load(&self, ids: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let query = format!(
            "
        SELECT *
        FROM users
        WHERE id in ({})
        ",
            ids.iter().join(",")
        );
        Ok(sqlx::query_as(&query)
            .fetch(&mut POOL.get().unwrap().acquire().await.unwrap())
            .map_ok(|user: User| (user.id, user))
            .try_collect()
            .await?)
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self) -> Result<Vec<User>> {
        Ok(find_users(&mut POOL.get().unwrap().acquire().await.unwrap()).await)
    }
}

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

async fn serve_asset(req: tide::Request<()>) -> tide::Result {
    let file_path = req.param("path").unwrap_or("index.html");
    match Assets::get(file_path) {
        Some(file) => {
            let mime = Mime::sniff(file.data.as_ref())
                .or_else(|err| {
                    Mime::from_extension(
                        Path::new(file_path).extension().unwrap().to_str().unwrap(),
                    )
                    .ok_or(err)
                })
                .unwrap_or(BYTE_STREAM);
            Ok(tide::Response::builder(StatusCode::Ok)
                .body(tide::Body::from_bytes(file.data.to_vec()))
                .content_type(mime)
                .build())
        }
        None => Ok(tide::Response::new(StatusCode::NotFound)),
    }
}

pub async fn main(pool: PgPool, matches: &ArgMatches<'_>) -> SimpleResult<()> {
    POOL.set(pool).expect("Failed to set database pool");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(DataLoader::new(UserLoader))
        .finish();

    let ctrlc = CtrlC::new().expect("Cannot use CTRL-C handler");
    ctrlc
        .race(async {
            let mut app = tide::new();

            app.at("/").get(serve_asset);
            app.at("/*path").get(serve_asset);

            app.at("/graphql").post(async_graphql_tide::graphql(schema));

            let address = matches.value_of("ADDRESS").unwrap();
            let port = matches.value_of("PORT").unwrap();
            app.listen(format!("{}:{}", address, port))
                .await
                .expect("Failed to run server");
        })
        .await;
    Ok(())
}
