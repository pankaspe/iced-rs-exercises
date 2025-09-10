// Declare the module `counter`.
// This tells Rust to look for a `counter` folder or `counter.rs` file in `src/`.
// All the code related to our Counter application is organized inside this module.
// Using modules keeps your project clean and scalable as you add more exercises.
mod counter;

// Import the Counter struct from the counter module.
// We don't need Message here because the main function never directly handles messages.
// It only interacts with Counter through its update and view methods.
use counter::Counter;

// Import iced::Task, which is used for async operations or side-effects.
// Even though our counter doesn't have async tasks yet, Iced requires a Task return type for update initialization.
use iced::Task;

fn main() -> Result<(), iced::Error> {
    // Launch the Iced application using the Elm Architecture pattern.
    //
    // iced::application takes three main arguments:
    // 1. The window title as a string: "Counter Example".
    // 2. The update function: Counter::update, which handles messages and updates state.
    // 3. The view function: Counter::view, which builds the UI based on the current state.
    //
    // run_with() is used to provide the initial application state and any initial tasks.
    // In our case:
    // - Counter::new() initializes the Counter struct (value = 0).
    // - Task::none() specifies that no async tasks are running at startup.
    iced::application("Counter Example", Counter::update, Counter::view)
        .run_with(|| (Counter::new(), Task::none()))
}
