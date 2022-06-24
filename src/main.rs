#![allow(dead_code)]

use mc_exam_randomizer::{
    errors::ExamReaderError,
    shuffler::{shuffle_exam, Exam},
};

fn main() {
    // test_shuffle_option();
    //test_shuffle_exam();
    let filename_tex = "files/exam.tex";
    // let filename_csv = "files/exam1.csv";

    match Exam::from_tex(filename_tex, "master") {
        Ok(ex) => {
            println!("master {:#?}", ex);
            let version_1 = shuffle_exam(&ex, Some("version 1"));
            println!("ver1 {:#?}", version_1);
        }
        Err(err) => match err {
            ExamReaderError::TemplateError(err_st) => {
                println!("ERR: {:#?}", err_st);
            }
            _ => println!("Err: {:#?}", err),
        },
    };
    // let qs_csv = from_csv(filename_csv);
    // println!("question from csv {:#?}", qs_csv)
}

/*
cargo watch -q -c -x 'run -q'
*/

// fn test_shuffle_option() -> () {
//     let cs: Option<Choices> = Some(Choices(
//         vec![
//             Choice::new("option 1"),
//             Choice::new("option 2"),
//             Choice::new("option 3"),
//             Choice::new("option 4"),
//             Choice::new("option 5"),
//         ],
//         CorrectChoice(2),
//         None,
//     ));

//     let q = Question {
//         text: String::from("Dummy"),
//         choices: cs,
//         order: 7,
//     };
//     println!("{:#?},{:#?}", q, shuffle_choices(&q))
// }

// fn test_shuffle_exam() -> () {
//     let qs = Some(vec![
//         Question::from("question 1", 1),
//         Question::from("question 2", 2),
//         Question::from("question 3", 3),
//         Question::from("question 4", 4),
//         Question::from("question 5", 5),
//         Question::from("question 6", 6),
//     ]);

//     let master = Exam {
//         name: String::from("master"),
//         questions: qs,
//         ordering: None,
//     };
//     let ver1 = shuffle_exam(&master, Some("vertion 1"));

//     let ver2 = shuffle_exam(&ver1, Some("vertion 2"));
//     println!("{}: {:#?}", master.name, master.ordering);
//     println!("{}: {:#?}", ver1.name, ver1.ordering);
//     println!("{}: {:#?}", ver2.name, ver2.ordering);
// }
