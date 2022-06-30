pub mod exam;
pub mod question;

pub use exam::*;
pub use question::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

pub fn shuffle_exam(ex: &Exam, name: Option<&str>) -> Exam {
    let name = if let Some(nm) = name { nm } else { &ex.name };

    if let Some(qs) = &ex.questions {
        let qs_shuffled: Vec<Question> = qs.into_iter().map(|q| shuffle_choices(q)).collect();
        let noq = qs.len() as u32;
        let mut ordering: Vec<u32> = (0..noq).collect();
        ordering.shuffle(&mut thread_rng());
        Exam {
            name: name.to_string(),
            preamble: ex.preamble.to_owned(),
            questions: Some(qs_shuffled),
            ordering: Some(ordering),
        }
    } else {
        Exam {
            name: name.to_string(),
            preamble: None,
            questions: None,
            ordering: None,
        }
    }
}

pub fn shuffle_choices(qs: &Question) -> Question {
    if let Some(cs) = &qs.choices {
        let Choices(vcs, CorrectChoice(crrct), _) = cs;
        let nocs = vcs.len() as u32;
        let mut ordering: Vec<u32> = (0..nocs).collect();
        ordering.shuffle(&mut thread_rng());
        let new_order = (&ordering)
            .iter()
            .position(|o| o == crrct)
            .unwrap_or(*crrct as usize);
        let choice_ordering = ChoiceOrdering(ordering.to_vec());
        let new_choices = Choices(
            vcs.to_vec(),
            CorrectChoice(new_order as u32),
            Some(choice_ordering),
        );
        Question {
            text: (qs.text).to_string(),
            order: qs.order,
            choices: Some(new_choices),
            group: qs.group,
        }
    } else {
        qs.to_owned()
    }
}
