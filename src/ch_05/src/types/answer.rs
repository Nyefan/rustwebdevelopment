use crate::types::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AnswerId(pub(crate) String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Answer {
    pub(crate) id: AnswerId,
    pub(crate) content: String,
    pub(crate) question_id: QuestionId,
}
