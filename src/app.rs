use cli_clipboard::{ClipboardContext, ClipboardProvider};
use tui::widgets::{ListState, TableState};

use crate::CrashContext;

// App struct
pub struct App<'a> {
    pub title: &'a str,
    pub window_state: Window,
    pub context_state: TableState,
    pub path_state: ListState,
    pub contexts: Vec<CrashContext>,
    pub should_quit: bool,
    clipboard_context: ClipboardContext,
}

pub enum Window {
    Contexts,
    Paths,
}

// State trait to use generic methods for state selection
trait State {
    fn selected(&self) -> Option<usize>;
    fn select(&mut self, index: Option<usize>);
}

impl State for ListState {
    fn selected(&self) -> Option<usize> {
        self.selected()
    }

    fn select(&mut self, index: Option<usize>) {
        self.select(index)
    }
}

impl State for TableState {
    fn selected(&self) -> Option<usize> {
        self.selected()
    }

    fn select(&mut self, index: Option<usize>) {
        self.select(index)
    }
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, crash_contexts: Vec<CrashContext>) -> App<'a> {
        // Get default indices
        let context_index = if !crash_contexts.is_empty() {
            Some(0)
        } else {
            None
        };

        let mut app = App {
            title,
            window_state: Window::Contexts,
            context_state: TableState::default(),
            path_state: ListState::default(),
            contexts: crash_contexts,
            should_quit: false,
            clipboard_context: ClipboardContext::new().unwrap(),
        };

        // Select first item if existing
        app.context_state.select(context_index);

        app
    }

    pub fn on_up(&mut self) {
        match self.window_state {
            Window::Contexts => {
                App::previous_item(&self.contexts, &mut self.context_state);
                self.path_state.select(None);
            }
            Window::Paths => App::previous_item(
                &self.contexts[self.context_state.selected().unwrap()].paths,
                &mut self.path_state,
            ),
        }
    }

    pub fn on_down(&mut self) {
        match self.window_state {
            Window::Contexts => {
                App::next_item(&self.contexts, &mut self.context_state);
                self.path_state.select(None);
            }
            Window::Paths => App::next_item(
                &self.contexts[self.context_state.selected().unwrap()].paths,
                &mut self.path_state,
            ),
        }
    }

    pub fn on_right(&mut self) {
        match self.window_state {
            Window::Paths => {}
            _ => {
                self.window_state = Window::Paths;

                // Select first path
                self.path_state.select(Some(0));
            }
        }
    }

    pub fn on_left(&mut self) {
        self.window_state = Window::Contexts;

        // Switch selection
        self.path_state.select(None);
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            'c' => {
                if let Window::Paths = self.window_state {
                    // Here one path should be selected!
                    let path = self.contexts[self.context_state.selected().unwrap()].paths
                        [self.path_state.selected().unwrap()]
                    .clone();
                    self.clipboard_context.set_contents(path).unwrap();
                }
            }
            _ => {}
        }
    }

    // Helper methods for item states
    fn next_item<T, S: State>(list: &[T], state: &mut S) {
        let i = match state.selected() {
            Some(i) => {
                if i >= list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        state.select(Some(i));
    }

    fn previous_item<T, S: State>(list: &[T], state: &mut S) {
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        state.select(Some(i));
    }
}
