# Multiple Choice Exam Randomizer (mc-exam-randomizer)

## Usage
```rust

use mc_exam_randomizer::{
    errors::ExamReaderError,
    shuffler::{shuffle_exam, Exam},
};

fn main() {
    
    // use your own file
    let filename_tex = "files/exam.tex";
    
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
    
}

```