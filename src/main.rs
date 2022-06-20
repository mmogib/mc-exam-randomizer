#![allow(dead_code)]
use exam::exam_faker::*;
use exam::shuffler::Exam;
// use core::slice::SlicePattern;

// use rand::rngs::StdRng;
// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use rand::SeedableRng;

fn main() {
    println!("Starting... ");
    let noq = 3;
    let qs = build_random_questions(noq);
    let master = Exam {
        name: String::from("master"),
        questions: Some(qs),
        ordering: None,
    };
    let version1 = master.shuffle();
    let version2 = master.shuffle();
    let version3 = master.shuffle();

    // // let qs1 = shuffle_questions(&qs);
    // // let qs2 = shuffle_questions(&qs);
    // // let qs3 = shuffle_questions(&qs);
    // // let qs4 = shuffle_questions(&qs);
    // // println!("{:#?}", vec);
    println!("{:#?}", version1.ordering);
    println!("{:#?}", version2.ordering);
    println!("{:#?}", version3.ordering);
    // // println!("{:#?}", qs1);
    // // println!("{:#?}", qs2);
    // // println!("{:#?}", qs3);
    // // println!("{:#?}", qs4);

    // let q2 = &qs[1];
    // let q2_co = get_correct_choice(q2);
    // let ca = match q2_co {
    //     Some(num) => num as i32,
    //     None => -1,
    // };
    // print!("{}", ca);
    // let v = [1, 2, 3, 4, 5];
    // let mut vv = v.clone();
    // vv.shuffle(&mut rand::thread_rng());

    // println!("v: {:#?},\n vv: {:#?}", v, vv);
}

/*
cargo watch -q -c -x 'run -q'
*/
