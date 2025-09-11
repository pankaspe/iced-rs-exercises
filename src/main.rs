mod counter;
mod todo;

// list all exercises
use counter::Counter;
use todo::TodoApp;

// Import iced::Task, which is used for async operations or side-effects.
// Even though our counter doesn't have async tasks yet, Iced requires a Task return type for update initialization.
use iced::Task;

fn main() -> Result<(), iced::Error> {

    // switch here for other exercises
    iced::application("Todo List Example", TodoApp::update, TodoApp::view)
        .run_with(|| (TodoApp::new(), Task::none()))
}
