use crate::database;
use crate::mutation::Mutation;
use crate::query::QueryRoot;
use async_graphql::{EmptySubscription, Schema};
use async_std::path::Path;
use async_std::sync::{Arc, Mutex};
use clap::{App, Arg, ArgMatches, SubCommand};
use http_types::mime::BYTE_STREAM;
use http_types::{Mime, StatusCode};
use rust_embed::RustEmbed;
use tide::sessions::{MemoryStore, SessionMiddleware};
use tindercrypt::cryptors::RingCryptor;

#[derive(RustEmbed)]
#[folder = "build/"]
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

async fn serve_asset(request: tide::Request<()>) -> tide::Result {
    let file_path = request.param("path").map_or("index.html", |path| {
        if path.split("/").last().unwrap().contains(".") {
            path
        } else {
            "index.html"
        }
    });
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

pub async fn main(matches: &ArgMatches<'_>) {
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription)
        .data(database::establish_connection(env!("DATABASE_URL")).await)
        .data(RingCryptor::new())
        .data(surf::client())
        .finish();

    let mut app = tide::new();

    app.with(SessionMiddleware::new(
        MemoryStore::new(),
        env!("TIDE_SECRET").as_bytes(),
    ));

    app.at("/").get(serve_asset);
    app.at("/*path").get(serve_asset);

    app.at("/graphql")
        .post(move |mut request: tide::Request<()>| {
            let schema = schema.clone();
            let session = Arc::new(Mutex::new(request.session_mut().clone()));
            async move {
                let mut graphql_request = async_graphql_tide::receive_request(request).await?;
                graphql_request = graphql_request.data(session);
                async_graphql_tide::respond(schema.execute(graphql_request).await)
            }
        });

    let address = matches.value_of("ADDRESS").unwrap();
    let port = matches.value_of("PORT").unwrap();
    app.listen(format!("{}:{}", address, port))
        .await
        .expect("Failed to run server");
}
