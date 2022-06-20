use crate::shuffler::*;
use fake::Fake;
use rand::seq::SliceRandom;

pub fn get_correct_choice(q: &Question) -> Option<u32> {
    let correct = match &q.choices {
        Some(cs) => {
            let CorrectChoice(ca) = cs.1;
            Some(ca)
        }
        None => None,
    };
    correct
}

fn build_random_question(order: u32) -> Question {
    Question::new(
        fake::faker::lorem::en::Sentence(1..5).fake(),
        order,
        Some(Choices(
            fake::vec![Choice; 5],
            CorrectChoice(
                *(vec![1, 2, 3, 1, 1])
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&1),
            ),
        )),
    )
}

pub fn build_random_questions(count: u32) -> Vec<Question> {
    (1..count + 1)
        .enumerate()
        .map(|(_, i)| build_random_question(i))
        .collect()
}
