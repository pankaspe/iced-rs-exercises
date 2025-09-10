// Declare submodules for this module (counter):
// - `state.rs` contains the Counter struct and its methods (update, view).
// - `message.rs` contains the Message enum defining possible events for the Counter.
pub mod state;
pub mod message;

// Re-export the Counter struct at the module level.
// This allows other parts of the code (like main.rs) to import Counter directly via:
// `use counter::Counter;`
// without having to know the internal file structure of the counter module.
pub use state::Counter;

// The Message enum is not re-exported here because it is only used internally within state.rs.
// Keeping it private is a good practice because the main.rs does not need to know about individual messages.
// If in the future multiple modules need to share Message, you could uncomment the following line:
//
// pub use message::Message;
