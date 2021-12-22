use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Result, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct ServeOptions {
    pub port: u16,
}

static NOTFOUND: &[u8] = b"Not Found";
static HELLOWORLD: &[u8] = b"Hello World";

async fn handle(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(static_page(&HELLOWORLD)),
        (&Method::GET, _) => {
            let subpath = &req.uri().path()[1..];
            file_serve(subpath).await
        }
        _ => Ok(not_found()),
    }
}

async fn file_serve(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.
    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        let mut res = Response::new(body);
        let guess = mime_guess::from_path(filename);
        if let Some(mime) = guess.first() {
            res.headers_mut()
                .insert(header::CONTENT_TYPE, header::HeaderValue::from_str(mime.essence_str()).unwrap());
        }
        return Ok(res);
    }
    Ok(not_found())
}

fn static_page(static_body: &'static [u8]) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(static_body.into())
        .unwrap()
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

pub async fn serve(opts: ServeOptions) -> Result<()> {
    println!("Serving http://localhost:{}...", opts.port);

    let addr = SocketAddr::from(([127, 0, 0, 1], opts.port));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_service);

    server.await?;

    Ok(())
}