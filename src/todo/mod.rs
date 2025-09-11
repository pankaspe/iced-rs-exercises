// ----------------------------
// mod.rs
// ----------------------------

// Declare sub-modules
pub mod state;    // contains the TodoApp state and its methods
pub mod message;  // contains the Message enum

// Re-export the main app state
// This allows other files (like main.rs) to do:
// `use todo::TodoApp;` instead of `use todo::state::TodoApp;`
pub use state::TodoApp;
