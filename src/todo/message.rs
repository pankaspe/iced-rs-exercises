// Define all possible user interactions (messages/events)
#[derive(Debug, Clone)]
pub enum Message {
    UpdateInput(String),   // User types in the text input
    Add,                   // Add a new todo
    Remove(usize),         // Remove todo at index
    ToggleComplete(usize), // Toggle todo completion
}
