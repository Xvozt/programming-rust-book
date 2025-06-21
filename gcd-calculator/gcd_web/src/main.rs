use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use gcd_lib::gcd;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Server is serving http://localhost:3001 ...");
    server
        .bind("127.0.0.1:3001").expect("error binding server to address")
        .run()
        .await
}


async fn get_index() -> impl Responder {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").body("Computing the GCD with zero is not allowed.")
    }

    let response = 
    format!("The greatest common divisor of {} and {} is <b>{}</b>\n", 
    form.n, form.m, gcd(form.n, form.m).unwrap());

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}