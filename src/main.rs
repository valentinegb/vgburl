use actix_web::{
    get,
    http::Uri,
    put,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use shuttle_actix_web::ShuttleActixWeb;

#[put("/")]
async fn put_url(req_body: String) -> impl Responder {
    let url = Uri::try_from(req_body);

    match url {
        Ok(url) => {
            todo!("attempt to put url in database");
        }
        Err(_) => HttpResponse::BadRequest(),
    }
}

#[get("/{id}")]
async fn get_url(id: web::Path<String>) -> impl Responder {
    todo!("search for url with id and attempt to redirect");

    HttpResponse::NotFound()
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(put_url).service(get_url);
    };

    Ok(config.into())
}
