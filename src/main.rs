use actix_web::{
    get,
    http::{header, Uri},
    put,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use nanoid::nanoid;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_persist::PersistInstance;

#[put("/")]
async fn put_url(req_body: String, state: web::Data<AppState>) -> impl Responder {
    let url = Uri::try_from(req_body);

    match url {
        Ok(url) => {
            let id = nanoid!(9);

            match state.persist.save::<String>(&id, url.to_string()) {
                Ok(_) => HttpResponse::Ok().body(id),
                Err(_) => HttpResponse::BadGateway().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[get("/{id}")]
async fn get_url(id: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
    let url = state.persist.load::<String>(id.as_str());

    match url {
        Ok(url) => HttpResponse::MovedPermanently()
            .append_header((header::LOCATION, url))
            .finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

struct AppState {
    persist: PersistInstance,
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(put_url)
            .service(get_url)
            .app_data(web::Data::new(AppState { persist }));
    };

    Ok(config.into())
}
