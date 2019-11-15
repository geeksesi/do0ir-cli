use std::env;

mod make_short;
mod output;
mod validation;

fn main() {
    let _args = env::args();
    let mut i = 0;
    let mut link: String = "".to_string();
    for argument in _args {
        if i == 1 {
            link = argument;
        }
        i += 1;
    }

    let link: String = validation::check_is_url(link);
    if link != "" {
        let do0_answer: make_short::Do0result = make_short::get_request(link).expect("Error");
        if do0_answer.error != "null" {
            eprintln!("something did Wrong from d0.ir.");
        } else {
            let shorted: String = output::after_success_job(do0_answer.short);
            output::copy_to_clipboard(shorted);
        }
    } else {
        println!("please input the valid link...and try again.");
    }
}
