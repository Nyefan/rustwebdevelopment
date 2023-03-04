use crate::questions::{Question, QuestionId};
use crate::answers::{Answer, AnswerId};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub(crate) struct Store {
    pub(crate) questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub(crate) answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub(crate) fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init_questions())),
            answers: Arc::new(RwLock::new(Self::init_answers()))
        }
    }

    fn init_questions() -> HashMap<QuestionId, Question> {
        let file = include_str!("questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }

    fn init_answers() -> HashMap<AnswerId, Answer> {
        HashMap::new()
    }
}