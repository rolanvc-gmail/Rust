# Actix_App
This follows the book RUST WEB PROGRAMMING, ch 3.

To run an actix-web app, include these in cargo.tom:

    [dependencies]
    # actix="0.11.0"
    actix-web = "4.0.0-beta.3"
    actix-rt="2.0.2"

The hello-world code is as follows:
First, we `use` the necessary libraries.

    use actix_web::{web, App, HttpRequest, HttpServer, Responder};

Then we declare handlers:

    1.    async fn greet(req: HttpRequest) -> impl Responder {
    2.    let name = req.match_info().get("name").unwrap_or("World");
        format!("Hello {}!", name)
    }

Finally, we call the main function:

    1. #[actix_rt::main]
    2. async fn main() -> std::io::Result<()> {
    3.    HttpServer::new(|| {
    4.         App::new()
    5.         .route("/", web::get().to(greet))
    6.        .route("/{name}", web::get().to(greet))
        })
    7.   .bind("127.0.0.1:8500")?
    8.   .run()
    9.    .await
    }

That seems to be it.
