extern crate clipboard;

use crate::make_short;
use crate::output;
use crate::validation;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use hotkey;

pub fn shortkey() {
    let mut hk = hotkey::Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
        'Q' as u32,
        || do_it(),
    )
    .unwrap();

    hk.listen();
}

fn get_link() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    return ctx.get_contents().unwrap().to_string();
}

fn do_it() {
    let mut link = get_link();

    link = validation::check_is_url(link);
    if link != "" {
        let do0_answer: make_short::Do0result = make_short::get_request(link).expect("Error");
        if do0_answer.error != "null" {
            notifica::notify("Do0.ir error", "Sorry i there is a problem with do0.ir");
            eprintln!("something did Wrong from d0.ir.");
        } else {
            notifica::notify("Do0.ir ...", "please wait...");
            let shorted: String = output::after_success_job(do0_answer.short);
            output::copy_to_clipboard(shorted);
            notifica::notify("Do0.ir successfully", "your link is in your clipboard just paste it.");
        }
    } else {
        notifica::notify("Do0.ir error", "please copy a valid link.");
        println!("please input the valid link...and try again.");
    }
}
