// ----------------------------
// main.rs
// ----------------------------

mod counter;
mod todo;
mod app;

use app::{MainApp};
use iced::Task;

fn main() -> Result<(), iced::Error> {
    // Launch the multi-exercise app
    iced::application("Multi-Exercise App", MainApp::update, MainApp::view)
        .run_with(|| (MainApp::new(), Task::none()))
}
