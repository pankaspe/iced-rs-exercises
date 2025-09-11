// ----------------------------
// app.rs
// ----------------------------

// Import modules declared in the crate root (main.rs)
use crate::counter::{Counter, Message as CounterMessage};
use crate::todo::{TodoApp, Message as TodoMessage};

// Import Iced types
use iced::{Element, Length, Task};
use iced::widget::{Column, Row, Button, Text};

// ----------------------------
// Global message type for the multi-exercise app
// ----------------------------
#[derive(Debug, Clone)]
pub enum AppMessage {
    Counter(CounterMessage), // Wrap messages from Counter
    Todo(TodoMessage),       // Wrap messages from Todo
    SwitchToCounter,
    SwitchToTodo,
}

// ----------------------------
// Enum to store the currently active exercise
// ----------------------------
#[derive(Debug, Clone)]
pub enum ActiveApp {
    Counter(Counter),
    Todo(TodoApp),
}

// ----------------------------
// Main wrapper struct for the app
// ----------------------------
#[derive(Debug, Clone)]
pub struct MainApp {
    active: ActiveApp,
}

impl MainApp {
    // Initialize the app with Counter active by default
    pub fn new() -> Self {
        Self {
            active: ActiveApp::Counter(Counter::new()),
        }
    }

    // Elm-style update function
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::SwitchToCounter => {
                self.active = ActiveApp::Counter(Counter::new());
            }
            AppMessage::SwitchToTodo => {
                self.active = ActiveApp::Todo(TodoApp::new());
            }
            AppMessage::Counter(msg) => {
                if let ActiveApp::Counter(c) = &mut self.active {
                    let _ = c.update(msg); // <- ignore the returned Task
                }
            }
            AppMessage::Todo(msg) => {
                if let ActiveApp::Todo(t) = &mut self.active {
                    let _ = t.update(msg); // <- ignore the returned Task
                }
            }
        }

        Task::none()
    }


    // Elm-style view function
    pub fn view(&self) -> Element<'_, AppMessage> {
        // ----------------------------
        // Header: buttons to switch between exercises
        // ----------------------------
        let header = Row::new()
            .spacing(20)
            .push(
                Button::new(Text::new("Counter"))
                    .on_press(AppMessage::SwitchToCounter)
            )
            .push(
                Button::new(Text::new("Todo"))
                    .on_press(AppMessage::SwitchToTodo)
            );

        // ----------------------------
        // Render the active exercise
        // ----------------------------
        let content: Element<'_, AppMessage> = match &self.active {
            ActiveApp::Counter(c) => c.view().map(AppMessage::Counter),
            ActiveApp::Todo(t) => t.view().map(AppMessage::Todo),
        };

        // ----------------------------
        // Combine header + active content in a vertical column
        // ----------------------------
        Column::new()
            .spacing(20)
            .push(header)
            .push(content)
            .width(Length::Fill)
            .into()
    }
}
