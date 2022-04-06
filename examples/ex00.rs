// Create a default reedline object to handle user input

use reedline::{DefaultPrompt, Reedline, Signal};
use std::io;

fn main() -> io::Result<()> {
    let mut line_editor = Reedline::create()?;
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt)?;
        match sig {
            Signal::Success(buffer) => {
                println!("We processed: {}", buffer);
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
            Signal::CtrlL => {
                line_editor.clear_screen().unwrap();
            }
        }
    }
}
