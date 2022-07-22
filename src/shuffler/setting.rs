use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExamSetting {
    pub university: String,
    pub department: String,
    pub term: String,
    pub coursecode: String,
    pub examname: String,
    pub examdate: String,
    pub timeallowed: String,
    pub numberofvestions: u32,
    pub groups: String,
}

impl ExamSetting {
    pub fn new() -> Self {
        ExamSetting {
            university: String::new(),
            department: String::new(),
            term: String::new(),
            coursecode: String::new(),
            examname: String::new(),
            examdate: String::new(),
            timeallowed: String::new(),
            numberofvestions: 0,
            groups: String::new(),
        }
    }
    pub fn append_from_key_value(
        exam_setting: ExamSetting,
        key: &str,
        value: String,
    ) -> ExamSetting {
        if key == "university" {
            ExamSetting {
                university: value,
                ..exam_setting
            }
        } else if key == "department" {
            ExamSetting {
                department: value,
                ..exam_setting
            }
        } else if key == "term" {
            ExamSetting {
                term: value,
                ..exam_setting
            }
        } else if key == "coursecode" {
            ExamSetting {
                coursecode: value,
                ..exam_setting
            }
        } else if key == "examname" {
            ExamSetting {
                examname: value,
                ..exam_setting
            }
        } else if key == "examdate" {
            ExamSetting {
                examdate: value,
                ..exam_setting
            }
        } else if key == "timeallowed" {
            ExamSetting {
                timeallowed: value,
                ..exam_setting
            }
        } else if key == "numberofvestions" {
            ExamSetting {
                numberofvestions: value.parse::<u32>().unwrap_or_default(),
                ..exam_setting
            }
        } else if key == "groups" {
            ExamSetting {
                groups: value,
                ..exam_setting
            }
        } else {
            exam_setting
        }
    }
}
