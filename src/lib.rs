pub mod craft;
pub mod garland;
pub mod macros;
pub mod role_actions;
pub mod task;
pub mod ui;

use failure::Error;
use std::ptr::null_mut;

pub struct Talan {
    window: ui::WinHandle,
}

impl Talan {
    pub fn new() -> Result<Talan, Error> {
        let mut window: ui::WinHandle = null_mut();
        // Can this became map err?
        if !ui::get_window(&mut window) {
            return Err(failure::format_err!(
                "Could not find FFXIV window. Is the client running?"
            ));
        }

        Ok(Talan{
            window: window,
        })
    }
 
    pub fn craft(&mut self, tasks: &[task::Task]) {
        craft::craft_items(self.window, tasks)
    }
}