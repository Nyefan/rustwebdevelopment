use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use warp::{Rejection, Reply};
use warp::hyper::StatusCode;
use crate::questions::QuestionId;
use crate::stores::Store;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnswerId(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId
}

pub(crate) async fn create(
    store: Store,
    params: HashMap<String, String>
) -> Result< impl Reply, Rejection> {
    let answer = Answer {
        id: AnswerId("1".to_string()),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(params.get("questionId").unwrap().to_string())
    };

    store.answers.write().await.insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}