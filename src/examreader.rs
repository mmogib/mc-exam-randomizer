use csv::{self};
use std::fs;

use crate::{
    constants::*,
    errors::ExamReaderError,
    shuffler::{Choice, Choices, CorrectChoice, ExamSetting, Question},
};

pub fn from_tex(
    filename: &str,
) -> Result<(Option<String>, Vec<Question>, Option<ExamSetting>), ExamReaderError> {
    let filecontent = fs::read_to_string(filename);
    match filecontent {
        Ok(contnet) => match get_questions_from_tex(&contnet) {
            Ok(cntnt) => Ok((
                get_preamble_from_text(&contnet),
                cntnt,
                get_setting_from_text(&contnet),
            )),
            Err(err) => Err(ExamReaderError::TemplateError(err)),
        },
        Err(err) => Err(ExamReaderError::IOError(err)),
    }
}
fn get_setting_from_text(content: &String) -> Option<ExamSetting> {
    if let Some(s) = content.find(TEX_SETTING_START) {
        if let Some(e) = content.find(TEX_SETTING_END) {
            let sttng = content[(s + 11)..e].trim().to_string();
            let sertting_parts: Vec<(String, String)> = sttng
                .split("\n")
                .map(|s| s.trim().trim_start_matches("%").trim())
                .map(|s| {
                    let key_val: Vec<String> = s
                        .split("=")
                        .map(|ss| ss.trim())
                        .map(|v| v.to_string())
                        .map(|v| v.trim().to_string())
                        .collect();
                    let key = if let Some(ks) = key_val.get(0) {
                        let val = if let Some(vs) = key_val.get(1) {
                            (ks.to_owned(), vs.to_owned())
                        } else {
                            (ks.to_owned(), "".to_string())
                        };
                        val
                    } else {
                        ("".to_string(), "".to_string())
                    };

                    return (key.0, key.1);
                })
                .collect();
            let exm_setting = sertting_parts.iter().fold(ExamSetting::new(), |a, v| {
                ExamSetting::append_from_key_value(a, &v.0, (v.1).to_owned())
            });

            return Some(exm_setting);
        } else {
            return None;
        }
    }

    None
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
    match filecontent {
        Ok(content) => {
            let rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(true)
                .from_reader(content.as_bytes());

            match get_questions_from_csv(rdr) {
                Ok(qs) => Ok(qs),
                Err(err) => Err(ExamReaderError::TemplateError(err)),
            }
        }
        Err(err) => Err(ExamReaderError::IOError(err)),
    }
}

pub fn from_txt(filename: &str) -> Result<Vec<Question>, ExamReaderError> {
    let filecontent = fs::read_to_string(filename);
    match filecontent {
        Ok(content) => {
            let rdr = csv::ReaderBuilder::new()
                .delimiter(b'\t')
                .flexible(true)
                .has_headers(false)
                .from_reader(content.as_bytes());
            match get_questions_from_csv(rdr) {
                Ok(qs) => Ok(qs),
                Err(err) => Err(ExamReaderError::TemplateError(err)),
            }
        }
        Err(err) => Err(ExamReaderError::IOError(err)),
    }
}

fn get_questions_from_csv(mut rdr: csv::Reader<&[u8]>) -> Result<Vec<Question>, String> {
    let mut order = 0;
    let qs: Vec<Question> = rdr
        .records()
        .into_iter()
        .map(|res| match res {
            Ok(rec) => {
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
                        choices: Some(choices),
                        group,
                    }
                } else {
                    Question::from("", 0)
                }
            }
            Err(_err) => Question::from("", 0),
        })
        .filter(|q| q.text != "")
        .collect();

    if qs.len() == 0 {
        return Err("no questions were found".to_string());
    }
    Ok(qs)
}

fn get_question_options_from_csv(options: Vec<String>) -> Choices {
    let choices: Vec<Choice> = options.into_iter().map(|o| Choice { text: o }).collect();
    Choices(choices, CorrectChoice(0), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_txt_bad_file() {
        //bad file
        let filename = "files/testing/samples.txt";
        let tex = match from_txt(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
        assert_eq!(
            tex,
            "Reading error".to_string(),
            "testing the file does not exist"
        )
    }
    #[test]
    fn read_from_txt_no_questions() {
        //bad file
        let filename = "files/testing/sample-no-questions.txt";
        let tex = match from_txt(filename) {
            Ok(_qs) => "".to_string(),
            Err(err) => err.to_string(),
        };
        assert_eq!(
            tex,
            "Your input file is badly formatted: `no questions were found`".to_string(),
            "testing no questions in csv"
        )
    }

    #[test]
    fn read_from_txt_first_is_different() {
        //bad file
        let filename = "files/testing/sample-first-options-different.txt";
        let tex = match from_txt(filename) {
            Ok(qs) => qs,
            Err(_err) => [].to_vec(),
        };
        assert_eq!(
            tex.len(),
            20,
            "testing first question with different options"
        );
        let qs1 = match tex.get(0) {
            Some(q) => match &q.choices {
                Some(op) => op.0.len(),
                None => 0,
            },
            None => 0,
        };
        assert_eq!(qs1, 6, "testing first question with different options");

        let qs2 = match tex.get(1) {
            Some(q) => match &q.choices {
                Some(op) => op.0.len(),
                None => 0,
            },
            None => 0,
        };
        assert_eq!(qs2, 5, "testing first question with different options");

        let qs3: i32 = match tex.get(2) {
            Some(q) => match &q.choices {
                Some(op) => op.0.len() as i32,
                None => 0,
            },
            None => -1,
        };
        assert_eq!(qs3, 0, "testing first question with different options")
    }

    #[test]
    fn read_from_csv_bad_file() {
        //bad file
        let filename = "files/testing/samples.csv";
        let tex = match from_csv(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
        assert_eq!(
            tex,
            "Reading error".to_string(),
            "testing the file does not exist"
        )
    }
    #[test]
    fn read_from_csv_no_questions() {
        //bad file
        let filename = "files/testing/sample-no-questions.csv";
        let tex = match from_csv(filename) {
            Ok(_qs) => "".to_string(),
            Err(err) => err.to_string(),
        };
        assert_eq!(
            tex,
            "Your input file is badly formatted: `no questions were found`".to_string(),
            "testing no questions in csv"
        )
    }

    #[test]
    fn read_from_csv_first_is_different() {
        //bad file
        let filename = "files/testing/sample-first-options-different.csv";
        let tex = match from_csv(filename) {
            Ok(qs) => qs,
            Err(_err) => [].to_vec(),
        };
        assert_eq!(
            tex.len(),
            6,
            "testing first question with different options"
        );
        let qs1 = match tex.get(0) {
            Some(q) => match &q.choices {
                Some(op) => op.0.len(),
                None => 0,
            },
            None => 0,
        };
        assert_eq!(qs1, 7, "testing first question with different options");

        let qs2 = match tex.get(1) {
            Some(q) => match &q.choices {
                Some(op) => op.0.len(),
                None => 0,
            },
            None => 0,
        };
        assert_eq!(qs2, 6, "testing first question with different options");

        let qs3: i32 = match tex.get(2) {
            Some(q) => match &q.choices {
                Some(op) => op.0.len() as i32,
                None => 0,
            },
            None => -1,
        };
        assert_eq!(qs3, 0, "testing first question with different options")
    }

    #[test]
    fn read_from_tex_bad_file() {
        //bad file
        let filename = "files/testing/templatte.tex";
        let tex = match from_tex(filename) {
            Ok(_) => "nothing".to_owned(),
            Err(err) => err.to_string(),
        };
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
        assert_eq!(
            tex,
            "Your input file is badly formatted: `No questions were found.`".to_string(),
            "testing no questions"
        );
    }
    fn read_from_tex() -> Result<(String, usize, Vec<Question>), String> {
        let filename = "files/testing/template.tex";
        match from_tex(filename) {
            Ok((preamble, qs, _)) => match preamble {
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

    #[test]
    fn read_from_tex_setting_full() {
        let filename = "files/testing/exam_setting.tex";
        let exammatch = match from_tex(filename) {
            Ok((_, _, es)) => match es {
                Some(exam_setting) => exam_setting,
                None => ExamSetting::new(),
            },
            Err(_err) => ExamSetting::new(),
        };
        assert_eq!(
            exammatch,
            ExamSetting {
                university: "KFUPM".to_string(),
                department: "MATH".to_string(),
                term: "Term 213".to_string(),
                coursecode: "MATH102".to_string(),
                examname: "Major Exam 1".to_string(),
                examdate: "2022-07-22T03:38:27.729Z".to_string(),
                timeallowed: "Two hours".to_string(),
                numberofvestions: 4,
                groups: "".to_string(),
            },
            "testing exam setting"
        );
    }

    #[test]
    fn read_from_tex_setting_partial() {
        let filename = "files/testing/exam_setting_withmissing_ones.tex";
        let exammatch = match from_tex(filename) {
            Ok((_, _, es)) => match es {
                Some(exam_setting) => exam_setting,
                None => ExamSetting::new(),
            },
            Err(_err) => ExamSetting::new(),
        };
        assert_eq!(
            exammatch,
            ExamSetting {
                university: "KFUPM".to_string(),
                department: "MATH".to_string(),
                term: "Term 213".to_string(),
                coursecode: "".to_string(),
                examname: "".to_string(),
                examdate: "".to_string(),
                timeallowed: "Two hours".to_string(),
                numberofvestions: 4,
                groups: "".to_string(),
            },
            "testing exam partial setting"
        );
    }

    #[test]
    fn read_from_tex_setting_empty() {
        let filename = "files/testing/template.tex";
        let exammatch = match from_tex(filename) {
            Ok((_, _, es)) => match es {
                Some(exam_setting) => exam_setting,
                None => ExamSetting::new(),
            },
            Err(_err) => ExamSetting::new(),
        };
        assert_eq!(
            exammatch,
            ExamSetting::new(),
            "testing exam setting is empty"
        );
    }
}
