// ----------------------------
// app.rs
// ----------------------------

// Import existing exercise modules
// `Counter` and `TodoApp` are separate exercises, each with its own `Message` type
use crate::counter::{Counter, Message as CounterMessage};
use crate::todo::{TodoApp, Message as TodoMessage};

use iced::alignment::Horizontal;
// Import Iced types
// - `Element` represents any GUI widget that can be rendered
// - `Length` controls sizing of widgets
// - `Task` allows async operations, used in the Elm-style update function
// - `Alignment` is used to align widgets horizontally or vertically
use iced::{Element, Length, Task, Alignment};
use iced::widget::{Column, Container, PickList, Text};

// ----------------------------
// Enum representing available exercises
// ----------------------------
// This enum lists all exercises the main app can switch between
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppChoice {
    Counter,
    Todo,
}

// Implement `Display` for PickList
// `PickList` requires the type to implement `Display` so it can render text labels
impl std::fmt::Display for AppChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppChoice::Counter => write!(f, "Counter"),
            AppChoice::Todo => write!(f, "Todo"),
        }
    }
}

// ----------------------------
// Global message type for MainApp
// ----------------------------
// Wraps messages from individual exercises and handles PickList selection
#[derive(Debug, Clone)]
pub enum AppMessage {
    SelectedApp(AppChoice),    // User selects a different app
    Counter(CounterMessage),   // Forwarded messages from Counter
    Todo(TodoMessage),         // Forwarded messages from Todo
}

// ----------------------------
// Active application enum
// ----------------------------
// Stores the currently active exercise state
#[derive(Debug, Clone)]
pub enum ActiveApp {
    Counter(Counter),
    Todo(TodoApp),
}

// ----------------------------
// Main wrapper struct
// ----------------------------
#[derive(Debug, Clone)]
pub struct MainApp {
    active: ActiveApp,               // Currently active exercise
    selected_app: AppChoice,         // Currently selected exercise in PickList
}

impl MainApp {
    // ----------------------------
    // Constructor: initialize default exercise
    // ----------------------------
    pub fn new() -> Self {
        Self {
            active: ActiveApp::Counter(Counter::new()), // default exercise
            selected_app: AppChoice::Counter,           // default selected
        }
    }

    // ----------------------------
    // Elm-style update function
    // ----------------------------
    // Handles messages and updates state
    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            // User selects a different exercise from PickList
            AppMessage::SelectedApp(app) => {
                self.selected_app = app; // update selection
                self.active = match app {
                    AppChoice::Counter => ActiveApp::Counter(Counter::new()), // reset exercise
                    AppChoice::Todo => ActiveApp::Todo(TodoApp::new()),
                };
            }

            // Forward messages to Counter
            AppMessage::Counter(msg) => {
                if let ActiveApp::Counter(c) = &mut self.active {
                    let _ = c.update(msg); // ignore returned Task, just forward
                }
            }

            // Forward messages to Todo
            AppMessage::Todo(msg) => {
                if let ActiveApp::Todo(t) = &mut self.active {
                    let _ = t.update(msg); // ignore returned Task
                }
            }
        }

        Task::none() // Return empty Task since Iced expects Task type
    }

    // ----------------------------
    // Elm-style view function
    // ----------------------------
    // Builds the GUI for the MainApp
    pub fn view(&self) -> Element<'_, AppMessage> {
        // ----------------------------
        // Header: title + subtitle, centered
        // ----------------------------
        let header = Column::new()
            .spacing(20)                   // vertical space between children
            .align_x(Alignment::Center)    // center all children horizontally
            .push(
                Text::new("ICED APP RS")   // main title
                    .size(40)
            )
            .push(
                Text::new("Select your app from the dropdown below") // subtitle
                    .size(20)
            );

        // Container for header to control width and alignment
        let header_container = Container::new(header)
            .width(Length::Fill)              // take full available width
            .align_x(Horizontal::Center);     // horizontally center content

        // ----------------------------
        // PickList to switch between exercises
        // ----------------------------
        // PickList expects:
        // 1. A slice of available options
        // 2. Option<T> for current selection
        // 3. A message constructor to send on selection
        let switcher: Element<'_, AppMessage> = PickList::new(
            &[AppChoice::Counter, AppChoice::Todo][..], // available options
            Some(self.selected_app),                    // currently selected
            AppMessage::SelectedApp,                    // message on selection
        )
        .width(Length::Fixed(200.0)) // fixed width for consistency
        .into();                     // convert PickList into Element

        // ----------------------------
        // Render the currently active exercise
        // ----------------------------
        let content: Element<'_, AppMessage> = match &self.active {
            ActiveApp::Counter(c) => c.view().map(AppMessage::Counter),
            ActiveApp::Todo(t) => t.view().map(AppMessage::Todo),
        };

        // ----------------------------
        // Combine everything in a vertical column
        // ----------------------------
        Column::new()
            .spacing(20)
            .align_x(Alignment::Center)        // center all children horizontally
            .push(Container::new(header_container).width(Length::Fill)) // header
            .push(switcher)                    // PickList
            .push(content)                     // active exercise content
            .width(Length::Fill)
            .into()                            // convert to Element
    }
}
