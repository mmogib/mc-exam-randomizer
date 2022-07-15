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
        Ok(contnet) => match get_questions_from_tex(&contnet) {
            Ok(cntnt) => Ok((get_preamble_from_text(&contnet), cntnt)),
            Err(err) => Err(ExamReaderError::TemplateError(err)),
        },
        Err(err) => Err(ExamReaderError::IOError(err)),
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

fn get_questions_from_tex(content: &String) -> Result<Vec<Question>, String> {
    let body_start = if let Some(bdy_start) = content.find(TEX_DOC_START) {
        bdy_start + 16
    } else {
        return Err("The document must have \\begin{document} tag".to_owned());
    };
    let body_end = if let Some(bdy_end) = content.find(TEX_DOC_END) {
        bdy_end
    } else {
        return Err("The document must have \\end{document} tag".to_owned());
    };
    let body = content[body_start..body_end].to_string();
    let parts: Vec<String> = body
        .split(TEX_QUESTION_START)
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
                order,
                group: 1,
            };
            order += 1;
            question
        })
        .collect();

    if qs.len() == 0 {
        return Err("No questions were found.".to_string());
    }
    Ok(qs)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_from_tex_bad_file() {
        //bad file
        let filename = "../files/testing/templatte.tex";
        let tex = match from_tex(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
        println!("{:#?}", tex);
        assert_eq!(
            tex,
            "Reading error".to_string(),
            "testing the file does not exist"
        )
    }
    #[test]
    fn read_from_tex_no_begin_doc() {
        // no begin doc, no end doc
        let filename = "files/testing/template-no-begin-doc.tex";
        let tex = match from_tex(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
        println!("{:#?}", tex);
        assert_eq!(
            tex,
            "Your input file is badly formatted: `The document must have \\begin{document} tag`"
                .to_string(),
            "testing begin document tag"
        );
    }
    #[test]
    fn read_from_tex_no_end_doc() {
        let filename = "files/testing/template-no-end-doc.tex";
        let tex = match from_tex(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
        println!("{:#?}", tex);
        assert_eq!(
            tex,
            "Your input file is badly formatted: `The document must have \\end{document} tag`"
                .to_string(),
            "testing end document tag"
        );
    }

    #[test]
    fn read_from_tex_no_questions() {
        let filename = "files/testing/template-no-questions.tex";
        let tex = match from_tex(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
        println!("{:#?}", tex);
        assert_eq!(
            tex,
            "Your input file is badly formatted: `No questions were found.`".to_string(),
            "testing no questions"
        );
    }
    fn read_from_tex() -> Result<(String, usize, Vec<Question>), String> {
        let filename = "files/testing/template.tex";
        match from_tex(filename) {
            Ok((preamble, qs)) => match preamble {
                Some(pre) => Ok((pre, qs.len(), qs)),
                None => Err("".to_string()),
            },
            Err(_err) => Err("".to_string()),
        }
    }

    #[test]
    fn read_from_tex_preamble() {
        match read_from_tex() {
            Ok(tex) => {
                assert_eq!(
                    tex.0,
                    "\\usepackage{amsfonts}".to_string(),
                    "testing preambles"
                );
            }
            Err(_err) => (),
        }
    }

    #[test]
    fn read_from_tex_number_of_qs() {
        match read_from_tex() {
            Ok(tex) => {
                assert_eq!(tex.1, 20);
            }
            Err(_err) => (),
        }
    }

    #[test]
    fn read_from_tex_number_of_options_is_zero() {
        match read_from_tex() {
            Ok(tex) => {
                let no_options_1: i32 = match tex.2.get(0) {
                    Some(op) => match &op.choices {
                        Some(opts) => opts.0.len() as i32,
                        None => 0,
                    },
                    None => -2,
                };
                assert_eq!(no_options_1, 0);
            }
            Err(_err) => (),
        }
    }

    #[test]
    fn read_from_tex_number_of_options_is_five() {
        match read_from_tex() {
            Ok(tex) => {
                let no_options_1: i32 = match tex.2.get(1) {
                    Some(op) => match &op.choices {
                        Some(opts) => opts.0.len() as i32,
                        None => 0,
                    },
                    None => -2,
                };
                assert_eq!(no_options_1, 5)
            }
            Err(_err) => (),
        }
    }
}
