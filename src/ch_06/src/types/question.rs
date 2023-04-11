use serde::{Deserialize, Serialize};
use warp::reject::Reject;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Question {
    pub(crate) id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub(crate) struct QuestionId(pub(crate) String);

#[derive(Debug)]
pub(crate) struct InvalidId;

impl Reject for InvalidId {}
