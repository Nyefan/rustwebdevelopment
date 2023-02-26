use std::fmt::{Display, Formatter};
use std::str::FromStr;


#[derive(Debug)]
pub(crate) struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub(crate) struct QuestionId(String);

impl Question {
    pub(crate) fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f,
               "id: {}, title: \"{}\", content: \"{}\"{}",
               self.id,
               self.title,
               self.content,
               match &self.tags {
                   None => "".to_string(),
                   Some(vec) => format!(", tags: {:?}", vec)
               })
    }
}

impl Display for QuestionId {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}", self.0
        )
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "No id provided")
            )
        }
    }
}

pub(crate) fn test() {
    let question1 = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    let question2 = Question::new(
        QuestionId::from_str("2").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        None,
    );
    println!("{}", question1);
    println!("{}", question2);
    println!("{:?}", question1);
    println!("{:?}", question2);
    println!("{:#?}", question1);
    println!("{:#?}", question2);
}
