use bytes::Bytes;
use clap::{Parser, Subcommand};
use colored_json::ToColoredJson;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{Method, Request, Uri, header::CONTENT_TYPE};
use hyper_util::rt::TokioIo;
use serde_json::json;
use tokio::net::TcpStream;
use yansi::Paint;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Parser)]
struct Cli {
    url: hyper::Uri,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List,
    Create {
        body: String,
    },
    Read {
        id: i64,
    },
    Update {
        id: i64,
        body: String,
        #[arg(short, long)]
        completed: bool,
    },
    Delete {
        id: i64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut uri_builder = Uri::builder();

    if let Some(scheme) = cli.url.scheme() {
        uri_builder = uri_builder.scheme(scheme.clone())
    }

    if let Some(authority) = cli.url.authority() {
        uri_builder = uri_builder.authority(authority.clone());
    }

    match cli.command {
        Commands::List => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Delete { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::DELETE,
                None,
            )
            .await
        }
        Commands::Update {
            id,
            body,
            completed,
        } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::PUT,
                Some(json!({"body":body, "completed":completed}).to_string()),
            )
            .await
        }
        Commands::Create { body } => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::POST,
                Some(json!({"body": body}).to_string()),
            )
            .await
        }
        Commands::Read { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::GET,
                None,
            )
            .await
        }
    }
}

async fn request(uri: hyper::Uri, method: Method, body: Option<String>) -> Result<()> {
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);

    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = uri.authority().unwrap().clone();

    let req = Request::builder()
        .uri(uri)
        .method(method)
        .header("content-type", "application/json")
        .header(hyper::header::HOST, authority.as_str())
        .body(match body {
            Some(s) => Full::new(Bytes::from(s)).boxed(),
            None => Empty::<Bytes>::new().boxed(),
        })?;

    let mut res = sender.send_request(req).await?;

    let mut buf = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            buf.extend_from_slice(&chunk);
        }
    }

    let s = String::from_utf8(buf)?;
    eprintln!("Status: {}", Paint::green(&res.status()));
    if res.headers().contains_key(CONTENT_TYPE) {
        let content_type = res.headers()[CONTENT_TYPE].to_str()?;
        eprintln!("Content-type: {}", Paint::green(content_type));
        if content_type.starts_with("application/json") {
            println!("{}", &s.to_colored_json_auto()?);
        } else {
            println!("{}", &s);
        }
    } else {
        println!("{}", &s);
    }

    Ok(())
}
