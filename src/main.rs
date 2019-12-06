extern crate notifica;

mod cli_argument;
mod make_short;
mod output;
mod shortkey;
mod validation;

fn do_it(mut link: String) {
    link = validation::check_is_url(link);
    if link != "" {
        let do0_answer: make_short::Do0result = make_short::get_request(link).expect("Error");
        if do0_answer.error != "null" {
            notifica::notify("Do0.ir Error", "Something are wrong from do0.ir");
            eprintln!("Something are wrong from do0.ir.");
        } else {
            notifica::notify("Do0.ir", "just wait a little 😎");
            let shorted: String = output::after_success_job(do0_answer.short);
            output::copy_to_clipboard(shorted);
            notifica::notify(
                "Do0.ir successfully",
                "your link is in your clipboard just paste it.",
            );
        }
    } else {
        println!("please input the valid link...and try again.");
        notifica::notify("Do0.ir error", "please give me a valid link.");
    }
}

fn main() {
    let args: cli_argument::CliArguments = cli_argument::arguments();

    // println!("{:#?}", args);

    if args.link != "null" {
        do_it(args.link);
    }

    if args.as_service {
        notifica::notify("Do0.ir", "do0-cli is here use CTRL+ALT+d 🌍");
        println!("Do0.ir is here use CTRL+ALT+d 🌍");
        shortkey::shortkey();
    }
}
