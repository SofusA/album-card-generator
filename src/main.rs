use std::{collections::HashMap, sync::Arc};

use axum::{Router, extract::State, response::Html, routing::get};
use clap::Parser;
use futures::future::try_join_all;
use qobuz_player_client::{client, qobuz_models::album::Album};
use tera::{Context, Tera, Value, to_value};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    album_ids: String,
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = client::new(&args.username, &args.password, client::AudioQuality::Mp3)
        .await
        .unwrap();

    let albums: Vec<_> = args.album_ids.split(' ').map(|x| client.album(x)).collect();
    let albums = try_join_all(albums).await.unwrap();

    let albums = Arc::new(AppState {
        context: TeraContext { albums },
    });

    let app = Router::new().route("/", get(handler)).with_state(albums);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct AppState {
    context: TeraContext,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct TeraContext {
    albums: Vec<Album>,
}

async fn handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut tera = Tera::new("templates/**/*.html").unwrap();
    tera.register_filter("year", release_year);
    let context = Context::from_serialize(&state.context).unwrap();

    let template = tera.render("index.html", &context).unwrap();

    Html(template)
}

fn release_year(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = tera::from_value::<String>(value.clone()).unwrap();
    let year = s.split('-').next().unwrap();
    Ok(to_value(year).unwrap())
}
