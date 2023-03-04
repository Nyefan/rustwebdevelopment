use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::reject::Reject;
use warp::{Rejection, Reply};
use warp::http::StatusCode;

use crate::errors::Error;
use crate::errors::Error::{MissingParameters, ParseError};
use crate::stores::Store;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[derive(Serialize, Deserialize)]
pub(crate) struct QuestionId(pub(crate) String);

#[derive(Debug)]
pub(crate) struct InvalidId;

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

impl Reject for InvalidId {}

pub(crate) async fn create(
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status(
        "Question added",
        StatusCode::OK,
    ))
}

pub(crate) async fn read(params: HashMap<String, String>, store: Store) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect();
        Ok(warp::reply::json(&res))
    }
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(ParseError)?,
        });
    }

    Err(MissingParameters)
}

pub(crate) async fn update(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write().
        await
        .get_mut(&QuestionId(id)) {
        Some(q) => {
            *q = question;
            Ok(warp::reply::with_status(
                "Question updated",
                StatusCode::OK,
            ))
        },
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

pub(crate) async fn delete(
    id: String,
    store: Store,
) -> Result<impl Reply, Rejection> {
    match store
        .questions
        .write()
        .await
        .remove(&QuestionId(id)) {
        Some(_) => {
            Ok(
                warp::reply::with_status(
                    "Question deleted",
                    StatusCode::OK,
                )
            )
        }
        None => Err(warp::reject::custom(Error::QuestionNotFound))
    }
}