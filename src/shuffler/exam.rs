use crate::{errors::ExamReaderError, examreader, shuffler::Question};

#[derive(Debug)]
pub struct Exam {
    pub name: String,
    pub questions: Option<Vec<Question>>,
    pub ordering: Option<Vec<u32>>,
}

impl Exam {
    pub fn new(name: &str) -> Self {
        Exam {
            name: String::from(name),
            questions: None,
            ordering: None,
        }
    }
    pub fn from_tex(filename: &str, name: &str) -> Result<Exam, ExamReaderError> {
        let questions = examreader::from_tex(filename)?;
        Ok(Exam {
            name: name.to_string(),
            questions: Some(questions),
            ordering: None,
        })
    }
    pub fn from_csv(filename: &str, name: &str) -> Result<Exam, ExamReaderError> {
        let questions = examreader::from_csv(filename)?;
        Ok(Exam {
            name: name.to_string(),
            questions: Some(questions),
            ordering: None,
        })
    }
}
