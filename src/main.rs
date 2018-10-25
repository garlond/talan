mod craft;
mod cross;
mod garland;
mod macros;
mod task;
mod ui;

use crate::craft::craft_items;
use crate::task::{Jobs, Task};
use std::path::PathBuf;
use std::ptr::null_mut;
use structopt::StructOpt;

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

    /// Increase delay between actions and UI navigation. Recommended with higher
    /// latency or input lag. [UNIMPLEMENTED]
    #[structopt(short = "d")]
    use_delay: bool,

    /// Gearset to use for this crafting task.
    #[structopt(short = "g")]
    gearset: u64,

    /// Item(s) will be crafted as collectable
    #[structopt(long = "collectable")]
    collectable: bool,

    /// Print verbose information during execution. [UNIMPLEMNETED]
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();
    let mut window: ui::WinHandle = null_mut();
    if !ui::get_window(&mut window) {
        return Err("Could not find FFXIV window. Is the client running?".to_string());
    }

    // Grab and parse the config file. Errors are all especially fatal so
    // let them bubble up if they occur.
    let macro_contents =
        macros::parse_file(opt.macro_file).map_err(|e| format!("error parsing macro: `{}`", e));

    let tasks = vec![Task {
        item_name: opt.item_name,
        index: opt.recipe_index,
        count: opt.count,
        collectable: opt.collectable,
        actions: macro_contents.unwrap(),
        gearset: opt.gearset,
        job: Jobs::CUL,
    }];
    craft_items(window, &tasks);
    Ok(())
}
