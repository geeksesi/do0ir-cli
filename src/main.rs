mod cli_argument;
mod make_short;
mod output;
mod validation;
mod shortkey;

fn do_it(mut link :String) {
    link = validation::check_is_url(link);
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

fn main() {
    let args: cli_argument::CliArguments = cli_argument::arguments();

    // println!("{:#?}", args);

    if args.link != "null"{
        do_it(args.link);
    }

    if args.as_service {
        shortkey::shortkey();
    }
}
