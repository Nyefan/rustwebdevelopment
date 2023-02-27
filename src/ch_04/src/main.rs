mod questions;
mod errors;

use warp::{
    Filter,
    filters::cors::CorsForbidden,
    http::{
        Method,
        StatusCode
    },
    Rejection,
    Reply
};
use crate::errors::Error;

#[tokio::main]
async fn main() {
    let store = questions::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(
            &[Method::PUT, Method::DELETE, Method::GET, Method::POST]
        );

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(questions::get)
        .recover(return_error);

    let routes = get_questions.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
        error.to_string(),
        StatusCode::RANGE_NOT_SATISFIABLE
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}