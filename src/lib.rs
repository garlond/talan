#[macro_use]
extern crate failure;

pub mod craft;
pub mod garland;
pub mod macros;
pub mod role_actions;
pub mod task;
pub mod ui;

use failure::Error;
use std::ptr::null_mut;
use std::sync::mpsc;
use std::thread;

enum Action {
    Tasks(Vec<task::Task>),
    Exit,
}

pub struct Talan {
    t: thread::JoinHandle<Result<(), Error>>,
    action_tx: mpsc::Sender<Action>,
    status_rx: mpsc::Receiver<craft::Status>,
}

impl Talan {
    pub fn new() -> Result<Talan, Error> {
        let (action_tx, action_rx): (mpsc::Sender<Action>, _) = mpsc::channel();
        let (status_tx, status_rx) = mpsc::channel();

        let t = thread::spawn(move || -> Result<(), Error> {
            let mut window: ui::WinHandle = null_mut();
            // Can this became map err?
            if !ui::get_window(&mut window) {
                return Err(failure::format_err!(
                    "Could not find FFXIV window. Is the client running?"
                ));
            }

            loop {
                match action_rx.recv() {
                    Ok(action) => {
                        match action {
                            Action::Tasks(tasks) => craft::craft_items(window, &tasks, status_tx.clone())?,
                            Action::Exit => break
                        }
                    },
                    Err(_) => break,
                };
            }

            Ok(())
        });

        Ok(Talan {
            t: t,
            action_tx: action_tx,
            status_rx: status_rx,
        })
    }

    pub fn craft(&mut self, tasks: Vec<task::Task>) {
        self.action_tx.send(Action::Tasks(tasks)).unwrap();
    }

    pub fn get_status(&self) -> Result<craft::Status, Error> {
        self.status_rx.recv().map_err(|e| format_err!("error receiving status: {}", e))
    }

    pub fn join(self) -> Result<(), Error> {
        self.action_tx.send(Action::Exit).unwrap();
        let err = self.t.join().unwrap();
        err
    }

}

