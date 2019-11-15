
use std::env;

mod make_short;
mod validation;
mod output;

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
            let mut shorted: String = "https://do0.ir/".to_string();
            shorted.push_str(&do0_answer.short);
            println!("Short link is {}", shorted);
            output::copy_to_clipboard(shorted);
        }
    } else {
        println!("please input the valid link...and try again.");
    }
}
