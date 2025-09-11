// ----------------------------
// state.rs
// ----------------------------

// Import Iced widgets: Button, Text, Row, Column, TextInput, Container, etc.
use iced::widget;
use iced::widget::{text_input, Column, Row};
use iced::{Element, Length, Task};

// Import the Message enum from the sibling module
use super::message::Message;

// ----------------------------
// Define the application state
// ----------------------------
#[derive(Debug, Clone)]
pub struct TodoApp {
    // List of all todos
    todos: Vec<TodoItem>,
    // Current value of the text input
    input_value: String,
}

// Each todo item has a text and a completion state
#[derive(Debug, Clone)]
pub struct TodoItem {
    text: String,
    completed: bool,
}

// ----------------------------
// Implement methods for TodoApp
// ----------------------------
impl TodoApp {

    // Constructor: creates a new app with empty todo list and empty input
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            input_value: String::new(),
        }
    }

    // Update method: handles all messages (Elm-style)
    // Receives a message and mutates the state accordingly
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Update the input value while typing
            Message::UpdateInput(value) => {
                self.input_value = value;
            }

            // Add a new todo if the input is not empty
            Message::Add => {
                if !self.input_value.trim().is_empty() {
                    self.todos.push(TodoItem {
                        text: self.input_value.trim().to_string(),
                        completed: false, // new todos are not completed
                    });
                    self.input_value.clear(); // clear input after adding
                }
            }

            // Remove a todo by index
            Message::Remove(index) => {
                if index < self.todos.len() {
                    self.todos.remove(index);
                }
            }

            // Toggle completion of a todo by index
            Message::ToggleComplete(index) => {
                if index < self.todos.len() {
                    self.todos[index].completed = !self.todos[index].completed;
                }
            }
        }

        // Return an empty Task (no async work here)
        Task::none()
    }

    // View method: builds the UI
    pub fn view(&self) -> Element<'_, Message> {
        // ----------------------------
        // Title
        // ----------------------------
        // Wrap the text in a Container to center it horizontally
        let title = widget::container(widget::text("Todo App").size(30))
            .width(Length::Fill)  // container takes all available width
            .center_x(Length::Fill); // center the text horizontally

        // ----------------------------
        // Input row: TextInput + Add button
        // ----------------------------
        let input_row = Row::new()
            .spacing(10)  // spacing between input and button
            .push(
                text_input("Enter a todo...", &self.input_value)
                    .on_input(Message::UpdateInput) // sends Message::UpdateInput on each change
                    .width(Length::Fill),           // input expands to fill available space
            )
            .push(
                widget::button("Add")
                    .on_press(Message::Add),        // sends Message::Add on click
            );

        // ----------------------------
        // List of todos
        // ----------------------------
        // Initialize a Column for the list of todos
        let mut list_column = Column::new().spacing(10).width(Length::Fill);

        // Iterate over todos and create a row for each
        for (index, todo) in self.todos.iter().enumerate() {
            // Display [x] if completed, [ ] if not
            let todo_text = if todo.completed {
                widget::text(format!("[x] {}", todo.text))
            } else {
                widget::text(format!("[ ] {}", todo.text))
            };

            // Row for each todo: text + toggle + remove buttons
            let todo_row = Row::new()
                .spacing(10)
                .push(todo_text)
                .push(
                    widget::button(if todo.completed { "Undo" } else { "Done" })
                        .on_press(Message::ToggleComplete(index)),
                )
                .push(
                    widget::button("Remove")
                        .on_press(Message::Remove(index)),
                );

            // Add the row to the list column
            list_column = list_column.push(todo_row);
        }

        // ----------------------------
        // Main vertical Column: combine title, input row, and todo list
        // ----------------------------
        let content = Column::new()
            .spacing(20)
            .max_width(400)    // limit maximum width of the app content
            .push(title)       // title at the top
            .push(input_row)   // input and Add button
            .push(list_column);// dynamic list of todos

        // ----------------------------
        // Container for centering everything in the window
        // ----------------------------
        widget::container(content)
            .width(Length::Fixed(400.0))  // fixed container width
            .center_x(Length::Fill)       // center horizontally in the window
            .center_y(Length::Fill)       // center vertically
            .into()                        // convert to Element for rendering
    }
}
