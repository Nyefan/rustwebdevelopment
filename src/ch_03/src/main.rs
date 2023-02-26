mod question;

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

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(
            &[Method::PUT, Method::DELETE, Method::GET, Method::POST]
        );

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(question::get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

pub(crate) async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN
        ))
    } else if let Some(_) = r.find::<question::InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}