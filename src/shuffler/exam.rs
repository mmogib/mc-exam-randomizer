use serde::{Deserialize, Serialize};

use crate::{errors::ExamReaderError, examreader, shuffler::Question};

use super::ExamSetting;

#[derive(Debug, Serialize, Deserialize)]
pub struct Exam {
    pub name: String,
    pub preamble: Option<String>,
    pub questions: Option<Vec<Question>>,
    pub ordering: Option<Vec<u32>>,
}

impl Exam {
    pub fn new(name: &str) -> Self {
        Exam {
            name: String::from(name),
            preamble: None,
            questions: None,
            ordering: None,
        }
    }
    pub fn from_tex(
        filename: &str,
        name: &str,
    ) -> Result<(Exam, Option<ExamSetting>), ExamReaderError> {
        let (preamble, questions, es) = examreader::from_tex(filename)?;
        Ok((
            Exam {
                name: name.to_string(),
                questions: Some(questions),
                preamble,
                ordering: None,
            },
            es,
        ))
    }
    pub fn from_csv(filename: &str, name: &str) -> Result<Exam, ExamReaderError> {
        let questions = examreader::from_csv(filename)?;
        Ok(Exam {
            name: name.to_string(),
            questions: Some(questions),
            preamble: None,
            ordering: None,
        })
    }

    pub fn from_txt(filename: &str, name: &str) -> Result<Exam, ExamReaderError> {
        let questions = examreader::from_txt(filename)?;
        Ok(Exam {
            name: name.to_string(),
            questions: Some(questions),
            preamble: None,
            ordering: None,
        })
    }
}
