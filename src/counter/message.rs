// The Message enum defines all possible events (or actions) that can occur in the Counter application.
// These messages are sent by the UI (e.g., button clicks) to the `update` method of Counter.
// This follows the Elm Architecture (TEA) pattern: Model + Message + Update + View.

#[derive(Debug, Clone, Copy)]
pub enum Message {
    // Sent when the "-" button is clicked.
    Increment,   // (Yes, the name Increment is intentional, represents the action to increase the counter.)

    // Sent when the "+" button is clicked.
    Decrement,   // Represents the action to decrease the counter.

    // Sent when the "Reset" button is clicked.
    Reset,       // Resets the counter value to zero.
}

// ----------------------------
// Best Practices / Notes
// ----------------------------
// 1. Use enums for messages/events: this ensures the update function can handle all possible cases.
// 2. #[derive(Debug, Clone, Copy]):
//    - Debug: allows printing messages for debugging purposes.
//    - Clone & Copy: lightweight, makes it easy to pass messages by value.
// 3. Keep messages minimal and descriptive: each button or action in the UI should have a clear corresponding message.
// 4. Message enum is private to the counter module unless explicitly re-exported.
//    Keeping it private helps encapsulate the module and reduce external dependencies.
