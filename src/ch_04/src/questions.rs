use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use warp::reject::Reject;
use warp::{Rejection, Reply};
use crate::errors::Error;
use crate::errors::Error::{MissingParameters, ParseError};

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
pub(crate) struct QuestionId(String);

#[derive(Debug)]
pub(crate) struct InvalidId;

#[derive(Clone, Debug)]
pub(crate) struct Store {
    questions: HashMap<QuestionId, Question>,
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

impl Reject for InvalidId {}

impl Store {
    pub(crate) fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    pub(crate) fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

pub(crate) async fn get(params: HashMap<String, String>, store: Store) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.values().cloned().collect();
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
                .map_err(ParseError)?
        });
    }

    Err(MissingParameters)
}