// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate navitia_model;
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use navitia_model::model::Collections;
use navitia_model::transfers;
use navitia_model::transfers::TransfersMode;
use navitia_model::Result;
#[macro_use]
extern crate failure;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "merge-ntfs",
    about = "Merge several ntfs into one",
    rename_all = "kebab-case"
)]
struct Opt {
    /// Input directories to process
    #[structopt(name = "INPUTS", parse(from_os_str))]
    input_directories: Vec<PathBuf>,

    /// output directory
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// config csv rule files.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    rule_files: Vec<PathBuf>,

    /// output report file path
    #[structopt(short, long, parse(from_os_str))]
    report: Option<PathBuf>,

    // The max distance in meters to compute the tranfer
    #[structopt(long, short = "d", default_value = "500")]
    max_distance: f64,

    // The walking speed in meters per second.
    // You may want to divide your initial speed by sqrt(2) to simulate Manhattan distances
    #[structopt(long, short = "s", default_value = "0.785")]
    walking_speed: f64,

    // Waiting time at stop in second
    #[structopt(long, short = "t", default_value = "60")]
    waiting_time: u32,
}

fn run() -> Result<()> {
    info!("Launching merge...");

    let opt = Opt::from_args();
    if opt.input_directories.len() < 2 {
        bail!("merge-ntfs process should have at least two input directories")
    } else {
        let mut collections = Collections::default();
        for input_directory in opt.input_directories {
            let to_append_model = navitia_model::ntfs::read(input_directory)?;
            collections.merge(to_append_model.into_collections())?;
        }
        let model = navitia_model::Model::new(collections)?;
        let model = transfers::generates_transfers(
            model,
            opt.max_distance,
            opt.walking_speed,
            opt.waiting_time,
            opt.rule_files,
            &TransfersMode::InterContributor,
            opt.report,
        )?;
        navitia_model::ntfs::write(&model, opt.output)?;
        Ok(())
    }
}

fn main() {
    env_logger::init();
    if let Err(err) = run() {
        for cause in err.iter_chain() {
            eprintln!("{}", cause);
        }
        std::process::exit(1);
    }
}
