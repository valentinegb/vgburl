#[macro_use]
extern crate rocket;

use nanoid::nanoid;
use rocket::{
    http::{
        uri::{Absolute, Reference},
        Status,
    },
    request::FromParam,
    response::{status, Redirect},
    State,
};
use shuttle_persist::PersistInstance;
use shuttle_rocket::ShuttleRocket;

#[post("/", data = "<req_body>")]
fn post_url(req_body: String, state: &State<AppState>) -> (Status, String) {
    let url = Absolute::parse_owned(req_body);

    match url {
        Ok(url) => {
            let id = nanoid!(9);

            match state.persist.save::<String>(&id, url.to_string()) {
                Ok(_) => (Status::Accepted, id),
                Err(_) => (
                    Status::BadGateway,
                    "Something went wrong whilst attempting to save the requested link."
                        .to_string(),
                ),
            }
        }
        Err(_) => (
            Status::BadRequest,
            "The requested link is invalid.".to_string(),
        ),
    }
}

#[get("/")]
fn get_repository() -> Redirect {
    Redirect::to(
        Reference::parse(env!("CARGO_PKG_REPOSITORY"))
            .expect("`Cargo.toml` should contain a valid `repository` key"),
    )
}

#[get("/<id>")]
fn get_url<'a>(
    id: LinkId<'a>,
    state: &State<AppState>,
) -> Result<Redirect, status::BadRequest<&'a str>> {
    let url = state.persist.load::<String>(id.0);

    match url {
        Ok(url) => Ok(Redirect::to(
            Reference::parse_owned(url).expect("`url` should be a valid URI"),
        )),
        Err(_) => Err(status::BadRequest(Some(
            "No link was found with the requested ID.",
        ))),
    }
}

struct AppState {
    persist: PersistInstance,
}

struct LinkId<'r>(&'r str);

impl<'r> FromParam<'r> for LinkId<'r> {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        if param.len() == 9 {
            Ok(Self(param))
        } else {
            Err(param)
        }
    }
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_persist::Persist] persist: PersistInstance) -> ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![post_url, get_repository, get_url])
        .manage(AppState { persist });

    Ok(rocket.into())
}
