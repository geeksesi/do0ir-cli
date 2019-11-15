extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub fn copy_to_clipboard(shorted: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(shorted).unwrap();
}
