use fake::{Dummy, Fake};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Dummy, Clone)]
pub struct Choice {
    #[dummy(faker = "20..30")]
    pub text: String,
}
#[derive(Debug, Dummy, Clone)]
pub struct CorrectChoice(pub u32);

#[derive(Debug, Dummy, Clone)]
pub struct Choices(pub Vec<Choice>, pub CorrectChoice);

#[derive(Debug, Dummy, Clone)]
pub struct Question {
    #[dummy(faker = "5")]
    pub text: String,
    pub order: u32,
    pub choices: Option<Choices>,
}

impl Question {
    pub fn new(text: String, order: u32, choices: Option<Choices>) -> Self {
        Question {
            text,
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
#[derive(Debug)]
pub struct Exam {
    pub name: String,
    pub questions: Option<Vec<Question>>,
    pub ordering: Option<Vec<u32>>,
}
impl Exam {
    pub fn new(name: String) -> Self {
        Exam {
            name,
            questions: None,
            ordering: None,
        }
    }

    pub fn shuffle(&self) -> Self {
        let name = &self.name;

        if let Some(qs) = &self.questions {
            let noq = qs.len() as u32;
            let mut ordering: Vec<u32> = (0..noq).collect();
            ordering.shuffle(&mut thread_rng());
            Exam {
                name: name.to_string(),
                questions: Some(qs.to_vec()),
                ordering: Some(ordering),
            }
        } else {
            Exam {
                name: name.to_string(),
                questions: None,
                ordering: None,
            }
        }
    }
}
pub fn shuffle_questions(qs: &Vec<Question>) -> Vec<&Question> {
    let noq = qs.len() as u32;
    let mut vec: Vec<u32> = (0..noq).collect();
    vec.shuffle(&mut thread_rng());
    let qs2: Vec<&Question> = vec
        .iter()
        .enumerate()
        .map(|(_i, ord)| qs.get(*ord as usize).unwrap())
        .collect();
    qs2
}
