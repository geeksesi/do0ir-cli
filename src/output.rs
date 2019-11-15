extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub fn copy_to_clipboard(shorted: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(shorted).unwrap();
}

pub fn after_success_job(short: String) -> String{
    let mut shorted: String = "https://do0.ir/".to_string();
    shorted.push_str(&short);
    println!("Short link is {}", shorted);
    return shorted;
}
