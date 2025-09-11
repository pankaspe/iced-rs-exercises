// Import the Iced widgets module, which contains all the UI components like buttons, text, rows, columns, containers, etc.
// Using `widget` as a module prefix keeps the code consistent and future-proof.
use iced::widget;

// Import core Iced types:
// - Element: represents a UI component that can be rendered.
// - Length: defines sizing behavior (Fill, Shrink, Units).
// - Task: used for asynchronous operations or side-effects.
use iced::{Element, Length, Task};

// Import alignment helpers to center text and other widgets.
use iced::alignment::{Horizontal, Vertical};

// Import the Message enum from the sibling module using `super`.
// `super` refers to the parent module (`counter`), and allows `state.rs` to access `message.rs`.
use super::message::Message;

// ----------------------------
// Define the application state
// ----------------------------
#[derive(Default, Debug, Clone)]
pub struct Counter {
    // This is the single piece of state: the counter's current value.
    pub value: i32,
}

// ----------------------------
// Implement methods for Counter
// ----------------------------
impl Counter {
    // Constructor: initializes a new Counter with default values.
    // Using `Self::default()` leverages the #[derive(Default)] implementation.
    // This approach is scalable: if more fields are added later, new() stays valid.
    pub fn new() -> Self {
        Self::default()
    }

    // Update method: called whenever a Message is produced (button clicks).
    // - &mut self: we need mutable access because we are modifying `self.value`.
    // - message: the event received from the UI.
    // Returns Task<Message> for async or side-effect operations; Task::none() if no async work is needed.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.value += 1, // Increase the counter
            Message::Decrement => self.value -= 1, // Decrease the counter
            Message::Reset => self.value = 0,      // Reset the counter
        }
        Task::none() // No asynchronous task here; Task::none() is required by Iced's architecture.
    }

    // View method: generates the UI based on the current state.
    // - &self: read-only reference because view does not modify state.
    // Returns an Element<Message>, which represents a renderable UI tree.
    pub fn view(&self) -> Element<'_, Message> {
        // Create a horizontal row containing the "-" button, the counter text, and the "+" button.
        // `spacing(10)` adds 10 pixels of space between each element in the row.
        let counter_row = widget::row![
            widget::button("-").on_press(Message::Decrement),
            widget::text!("Count: {}", self.value),
            widget::button("+").on_press(Message::Increment),
        ]
        .spacing(10);

        // Create a vertical column to stack the counter_row on top and the Reset button below.
        let content = widget::column![
            counter_row,
            // Reset button with text centered inside the button.
            widget::button(
                widget::container(widget::text("Reset"))
                    .align_x(Horizontal::Center) // Center text horizontally
                    .align_y(Vertical::Center)   // Center text vertically
                    .width(Length::Fill)         // Fill the width of its container
            )
            .width(Length::Fill) // Button fills width of the column
            .on_press(Message::Reset), // Send Reset message when clicked
        ]
        .spacing(20)         // Vertical spacing between row and Reset button
        .width(Length::Shrink); // Column width adjusts to its widest child (row above)

        // Wrap everything in a container to center it in the window.
        // - center_x / center_y: center the content in both directions
        // - Length::Fill ensures the container occupies the full window size, allowing centering
        widget::container(content)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into() // Convert container into Element<Message> for Iced
    }
}
