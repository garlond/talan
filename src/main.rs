extern crate talan;

use failure::Error;
use log;
use pretty_env_logger;
use std::path::PathBuf;
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
    loop {
        match talan.get_status() {
            Ok(status) => match status.state {
                talan::craft::State::Queued => (),
                talan::craft::State::Initializing => println!("Initializing"),
                talan::craft::State::Setup => {
                    println!("Setting Up Craft {}/{}", status.craft, opt.count)
                }
                talan::craft::State::Crafting(a) => {
                    println!("  Step {}/{} {}", status.step, actions_len, a)
                }
                talan::craft::State::Done => {
                    println!("Done");
                    break;
                }
            },
            Err(_) => break,
        }
    }
    talan.join()
}
