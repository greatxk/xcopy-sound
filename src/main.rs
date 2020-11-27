#![windows_subsystem = "windows"]
use std::{io::Cursor, path::PathBuf, path::Path};
use xclipboard_notify::ClipboardNotify;

use single_instance::SingleInstance;

fn main() {
    
    let inst = SingleInstance::new(".xcopy-sound").unwrap();
    if !inst.is_single() {
        return;
    }

    let data = include_bytes!("../sound/copy.wav");
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let notify = ClipboardNotify::new();
    notify.listen(|_msg| {
        let cursor = Cursor::new(data);
        let sink = handle.play_once(cursor).unwrap();
        sink.sleep_until_end();
    });
}
