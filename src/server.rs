use super::database::*;
use super::model::*;
use async_ctrlc::CtrlC;
use async_graphql::{ComplexObject, Context, EmptySubscription, Object, Result, Schema};
use async_std::path::Path;
use async_std::prelude::FutureExt;
use clap::{App, Arg, ArgMatches, SubCommand};
use http_types::mime::BYTE_STREAM;
use http_types::{Mime, StatusCode};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use rust_embed::RustEmbed;
use simple_error::SimpleResult;
use sqlx::postgres::PgPool;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

lazy_static! {
    static ref CRYPTOR: RingCryptor<'static> = RingCryptor::new();
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

#[ComplexObject]
impl User {
    async fn people(&self, _ctx: &Context<'_>) -> Result<Vec<Person>> {
        Ok(
            find_people_by_user_id(&mut POOL.get().unwrap().acquire().await.unwrap(), &self.id)
                .await,
        )
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self) -> Result<Vec<User>> {
        Ok(find_users(&mut POOL.get().unwrap().acquire().await.unwrap()).await)
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn signup(&self, username: String, password: String) -> Result<Uuid> {
        Ok(create_user(
            &mut POOL.get().unwrap().acquire().await.unwrap(),
            &username,
            &blake3::hash(password.as_bytes()).to_hex(),
        )
        .await)
    }

    async fn login(&self, username: String, password: String) -> Result<bool> {
        let user =
            find_user_by_username(&mut POOL.get().unwrap().acquire().await.unwrap(), &username)
                .await;
        let password_hash = blake3::hash(password.as_bytes()).to_hex();
        Ok(user.password_hash == password_hash.to_string())
    }

    async fn add_person(&self, name: String, user_id: Uuid, password: String) -> Result<Uuid> {
        Ok(create_person(
            &mut POOL.get().unwrap().acquire().await.unwrap(),
            &CRYPTOR.seal_with_passphrase(password.as_bytes(), name.as_bytes())?,
            &user_id,
        )
        .await)
    }
}

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, Mutation, EmptySubscription>,
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

    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription).finish();

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
