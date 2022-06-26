use serde::{ser::SerializeTupleStruct, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub order: u32,
    pub choices: Option<Choices>,
}

impl Question {
    pub fn new(text: &str, order: u32, choices: Option<Choices>) -> Self {
        Question {
            text: String::from(text),
            order,
            choices,
        }
    }
    pub fn from(text: &str, order: u32) -> Self {
        Question {
            text: String::from(text),
            order,
            choices: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choices(
    pub Vec<Choice>,
    pub CorrectChoice,
    pub Option<ChoiceOrdering>,
);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
}
impl Choice {
    pub fn new(text: &str) -> Choice {
        Choice {
            text: String::from(text),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrectChoice(pub u32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceOrdering(pub Vec<u32>);
#[cfg(test)]
mod tests {
    use super::*;

    fn question() -> Question {
        Question::new(
            "What is the meaning of life?",
            1,
            Some(Choices(
                vec![
                    Choice::new("41"),
                    Choice::new("42"),
                    Choice::new("43"),
                    Choice::new("44"),
                    Choice::new("45"),
                ],
                CorrectChoice(1),
                None,
            )),
        )
    }

    #[test]
    fn question_text() {
        let q = question();
        assert_eq!(q.text, "What is the meaning of life?");
    }

    #[test]
    fn question_oredr() {
        let q = question();
        assert_eq!(q.order, 1);
    }
    #[test]
    fn question_choices() {
        let q = question();
        let opts = Some(Choices(
            vec![
                Choice::new("41"),
                Choice::new("42"),
                Choice::new("43"),
                Choice::new("44"),
                Choice::new("45"),
            ],
            CorrectChoice(1),
            None,
        ));
        assert_eq!(q.choices, opts);
    }

    #[test]
    fn question_from() {
        let q = Question::from("question from", 2);
        let q2 = Question {
            text: String::from("question from"),
            choices: None,
            order: 2,
        };
        assert_eq!(q, q2);
    }
}
