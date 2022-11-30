use actix_session::{storage::RedisActorSessionStore, SessionMiddleware, Session};
use actix_web::{
    get, post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let private_key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("qwq.sh:6379"),
                    private_key.clone(),
                )
                .build(),
            )
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Port is already in use. (Didn't happen in testing, don't ask me why.)")
    .run()
    .await
}

#[get("/field/{id}")] // somehow doesn't work yet.
async fn hello(path: web::Path<(u32,)>, session: Session) -> impl Responder {
    if let Err(e) = session.insert("test_key", 42) {
        eprintln!("Could not insert data into database: {}", e);
        return HttpResponse::InternalServerError().body("Could not insert data into database.");
    }

    HttpResponse::Ok().body("Hello world!")
}

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
