extern crate indicatif;
extern crate talan;

use failure::Error;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log;
use pretty_env_logger;
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;
use talan::task::Task;

#[derive(StructOpt, Debug)]
#[structopt(name = "Talan")]
struct Opt {
    /// For recipes which have multiple search results this offset is used to
    /// determine the specific recipe to use. Offsets start at 0 for the first
    /// recipe in search results and increment by one for each recipe down.
    #[structopt(short = "i", default_value = "0")]
    recipe_index: u64,

    /// Path to the file containing the XIV macros to use
    #[structopt(name = "macro file", parse(from_os_str))]
    macro_file: PathBuf,

    /// Name of the item to craft
    #[structopt(name = "item name")]
    item_name: String,

    /// Number of items to craft
    #[structopt(short = "c", default_value = "1")]
    count: u64,

    /// Gearset to use for this crafting task.
    #[structopt(short = "g", default_value = "0")]
    gearset: u64,

    /// Item(s) will be crafted as collectable
    #[structopt(long = "collectable")]
    collectable: bool,

    /// Do not craft, but attempt to set everything up to do so
    #[structopt(short = "n")]
    dryrun: bool,
}

struct ProgressDisplay {
    thread_handle: thread::JoinHandle<()>,
    craft_bar: ProgressBar,
    step_bar: ProgressBar,
}

impl ProgressDisplay {
    fn new(num_crafts: u64, num_steps: u64, item_name: &str) -> ProgressDisplay {
        let bar_style = ProgressStyle::default_bar()
            .template("{pos:>2}/{len:2} {spinner} {bar:40.cyan/blue} {msg}");
        let m = MultiProgress::new();
        let craft_bar = m.add(ProgressBar::new(num_crafts));
        craft_bar.set_style(bar_style.clone());
        let step_bar = m.add(ProgressBar::new(num_steps as u64));
        step_bar.set_style(bar_style.clone());

        let t = thread::spawn(move || {
            m.join_and_clear().unwrap();
        });

        craft_bar.set_position(0);
        craft_bar.set_message(item_name);
        step_bar.set_position(0);
        step_bar.enable_steady_tick(100);

        ProgressDisplay {
            thread_handle: t,
            craft_bar: craft_bar,
            step_bar: step_bar,
        }
    }

    /// Handles a status update and updates the progress bars
    ///
    /// Returns true if the task is done.
    fn update(&self, status: &talan::craft::Status) -> bool {
        match &status.state {
            talan::craft::State::Queued => false,
            talan::craft::State::Initializing => {
                self.step_bar.set_message("Initializing");
                false
            }
            talan::craft::State::Setup => {
                self.craft_bar.set_position(status.craft);
                self.step_bar.set_message("Setting up");
                false
            }
            talan::craft::State::Crafting(a) => {
                self.step_bar.set_position(status.step);
                if status.step == 0 {
                    self.step_bar.set_draw_delta(status.step);
                }
                self.step_bar.set_message(&a);
                false
            }
            talan::craft::State::Done => true,
        }
    }

    /// Clean up the progress bar display.
    ///
    /// Consumes self.  Blocks until all helper threads are terminated.
    fn join(self) {
        self.step_bar.finish_and_clear();
        self.craft_bar.finish_and_clear();
        self.thread_handle.join().unwrap();
    }
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init_timed();

    let opt = Opt::from_args();

    let mut talan = talan::Talan::new()?;

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let macro_contents = talan::macros::parse_file(opt.macro_file)
        .map_err(|e| format!("error parsing macro: `{}`", e));

    let item = talan::garland::fetch_item_info(&opt.item_name)?;
    log::info!("item information: {}", item);
    let actions = macro_contents.unwrap();
    let actions_len = actions.len();
    let tasks = vec![Task {
        item: item,
        index: opt.recipe_index,
        count: opt.count,
        actions: actions,
        gearset: opt.gearset,
        collectable: opt.collectable,
    }];

    talan.craft(tasks);
    let prog = ProgressDisplay::new(opt.count, actions_len as u64, &opt.item_name);
    loop {
        let status = talan.get_status()?;
        if prog.update(&status) {
            break;
        }
    }

    talan.join()?;
    prog.join();

    println!("Done.");

    Ok(())
}
