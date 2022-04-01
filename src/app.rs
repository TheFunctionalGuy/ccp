use crate::CrashContext;

pub struct App<'a> {
    pub title: &'a str,
    pub contexts: Vec<CrashContext>,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, crash_contexts: Vec<CrashContext>) -> App<'a> {
        App {
            title,
            contexts: crash_contexts,
            should_quit: false,
        }
    }

    pub fn on_up(&mut self) {
        // self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        // self.tasks.next();
    }

    pub fn on_right(&mut self) {
        // self.tabs.next();
    }

    pub fn on_left(&mut self) {
        // self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
