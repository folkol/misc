use std::collections::HashMap;

use actix_protobuf::ProtoBufResponseBuilder;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use prost::Message;
use serde::Serialize;

#[derive(Serialize, Message, Clone, Eq, PartialEq)]
struct Aspect {
    #[prost(map = "string, message", tag = "2")]
    data: HashMap<String, String>,
}

#[derive(Serialize, Message, Clone)]
struct Content {
    #[prost(map = "string, message", tag = "1")]
    aspects: HashMap<String, Aspect>,
}

#[get("/json")]
async fn json(data: web::Data<Content>) -> web::Json<Content> {
    web::Json(data.get_ref().clone())
}

#[get("/proto")]
async fn proto(data: web::Data<Content>) -> impl Responder {
    HttpResponse::Ok().protobuf(data.get_ref().clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(|| {
        let envelope = Content {
            aspects: HashMap::from([
                ("aspect1".to_owned(), Aspect {
                    data: HashMap::from([
                        ("key1".to_owned(), "value1".to_owned()),
                        ("key2".to_owned(), "value2".to_owned()),
                        ("key3".to_owned(), "value3".to_owned()),
                        ("key4".to_owned(), "value4".to_owned())
                    ])
                }),
                ("aspect2".to_owned(), Aspect {
                    data: HashMap::from([
                        ("key1".to_owned(), "value1".to_owned()),
                        ("key2".to_owned(), "value2".to_owned()),
                        ("key3".to_owned(), "value3".to_owned()),
                        ("key4".to_owned(), "value4".to_owned())
                    ])
                }),
                ("aspect3".to_owned(), Aspect {
                    data: HashMap::from([
                        ("key1".to_owned(), "value1".to_owned()),
                        ("key2".to_owned(), "value2".to_owned()),
                        ("key3".to_owned(), "value3".to_owned()),
                        ("key4".to_owned(), "value4".to_owned())
                    ])
                }),
                ("aspect4".to_owned(), Aspect {
                    data: HashMap::from([
                        ("key1".to_owned(), "value1".to_owned()),
                        ("key2".to_owned(), "value2".to_owned()),
                        ("key3".to_owned(), "value3".to_owned()),
                        ("key4".to_owned(), "value4".to_owned())
                    ])
                }),
            ])
        };
        App::new()
            .app_data(web::Data::new(envelope))
            .service(json)
            .service(proto)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}