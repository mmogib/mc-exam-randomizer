use csv::{self};
use std::fs;

use crate::{
    constants::*,
    errors::ExamReaderError,
    shuffler::{Choice, Choices, CorrectChoice, Question},
};

pub fn from_tex(filename: &str) -> Result<(Option<String>, Vec<Question>), ExamReaderError> {
    let filecontent = fs::read_to_string(filename);
    match filecontent {
        Ok(contnet) => {
            if let Some(rcontent) = get_questions_from_tex(&contnet) {
                Ok((get_preamble_from_text(&contnet), rcontent))
            } else {
                Err(ExamReaderError::Unknown)
            }
        }
        Err(err) => Err(ExamReaderError::TemplateError(err.to_string())),
    }
}

fn get_preamble_from_text(content: &String) -> Option<String> {
    if let Some(s) = content.find(TEX_PREAMBLE_START) {
        if let Some(e) = content.find(TEX_PREAMBLE_END) {
            let preamble = content[(s + 12)..e].trim().to_string();
            return Some(preamble);
        } else {
            return None;
        }
    }
    None
}

fn get_questions_from_tex(content: &String) -> Option<Vec<Question>> {
    let body_start = content.find(TEX_DOC_START).unwrap() + 16;
    let body_end = content.find(TEX_DOC_END).unwrap();
    let body = content[body_start..body_end].to_string();
    let parts: Vec<String> = body
        .split("%{#q}")
        .map(|p| String::from(p.trim()))
        .collect();
    let mut order: u32 = 1;
    let qs: Vec<Question> = parts
        .into_iter()
        .map(|q| {
            let body = get_question_text_from_tex(&q);
            (body, q)
        })
        .filter(|(b, _q)| b != "")
        .map(|(body, q)| {
            let opts = get_question_options_from_tex(&q);
            let question = Question {
                text: body,
                choices: opts,
                order: order,
                group: 1,
            };
            order += 1;
            question
        })
        .collect();

    if qs.len() == 0 {
        return None;
    }
    Some(qs)
}

fn get_question_text_from_tex(q: &String) -> String {
    if let Some(end_of_question_text) = q.find(TEX_QUESTION_END) {
        let text = q[..end_of_question_text].trim().to_string();
        text
    } else {
        "".to_string()
    }
}

fn get_question_options_from_tex(q: &String) -> Option<Choices> {
    let parts: Vec<Choice> = q
        .split(TEX_OPTION_START)
        .map(|f| {
            if let Some(o_end) = f.find(TEX_OPTION_END) {
                f[..o_end].trim().to_string()
            } else {
                "".to_string()
            }
        })
        .filter(|o| o != "")
        .map(|o| Choice::new(&o))
        .collect();

    if parts.len() == 0 {
        return None;
    }
    Some(Choices(parts, CorrectChoice(0), None))
}

pub fn from_csv(filename: &str) -> Result<Vec<Question>, ExamReaderError> {
    let filecontent = fs::read_to_string(filename);
    if let Ok(content) = filecontent {
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(content.as_bytes());
        if let Some(qs) = get_questions_from_csv(rdr) {
            Ok(qs)
        } else {
            Err(ExamReaderError::TemplateError("Error".to_string()))
        }
    } else {
        Err(ExamReaderError::TemplateError("some thing".to_string()))
    }
}

pub fn from_txt(filename: &str) -> Result<Vec<Question>, ExamReaderError> {
    let filecontent = fs::read_to_string(filename);
    if let Ok(content) = filecontent {
        let rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(content.as_bytes());
        if let Some(qs) = get_questions_from_csv(rdr) {
            Ok(qs)
        } else {
            Err(ExamReaderError::TemplateError("Error".to_string()))
        }
    } else {
        Err(ExamReaderError::TemplateError("some thing".to_string()))
    }
}

fn get_questions_from_csv(mut rdr: csv::Reader<&[u8]>) -> Option<Vec<Question>> {
    let mut order = 0;
    let qs = rdr
        .records()
        .into_iter()
        .map(|res| {
            if let Ok(rec) = res {
                let record: Vec<String> = rec.iter().map(|f| f.to_string()).collect();
                let choices = get_question_options_from_csv(record[2..].to_vec());
                if let Some(text) = record.get(1) {
                    order = order + 1;
                    let group: u32 = if let Some(group_str) = record.get(0) {
                        group_str.parse().unwrap_or(1)
                    } else {
                        1
                    };

                    Question {
                        text: text.to_owned(),
                        order,
                        choices,
                        group,
                    }
                } else {
                    Question::from("", 0)
                }
            } else {
                Question::from("", 0)
            }
        })
        .filter(|q| q.text != "")
        .collect();
    Some(qs)
}

fn get_question_options_from_csv(options: Vec<String>) -> Option<Choices> {
    let choices: Vec<Choice> = options.into_iter().map(|o| Choice { text: o }).collect();
    Some(Choices(choices, CorrectChoice(0), None))
}
