use regex::Regex;
use std::path::Path;
use structopt::StructOpt;

use bitvec_helpers::{bitvec_reader, bitvec_writer};

mod commands;
use commands::Command;

mod dovi;
use dovi::{demuxer::Demuxer, editor::Editor, rpu_extractor::RpuExtractor, Format, RpuOptions};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(
        name = "mode",
        short = "m",
        long,
        help = "Sets the mode for RPU processing. --help for more info",
        long_help = "Sets the mode for RPU processing.\nMode 1: Converts the RPU to be MEL compatible\nMode 2: Converts the RPU to be profile 8.1 compatible"
    )]
    mode: Option<u8>,

    #[structopt(
        long,
        short = "c",
        help = "Set active area offsets to 0, cannot be used with mode 0"
    )]
    crop: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

fn main() {
    let opt = Opt::from_args();

    let rpu_options = RpuOptions {
        mode: opt.mode,
        crop: opt.crop,
    };

    match opt.cmd {
        Command::Demux {
            input,
            stdin,
            bl_out,
            el_out,
        } => Demuxer::demux(input, stdin, bl_out, el_out, rpu_options),
        Command::ExtractRpu {
            input,
            stdin,
            rpu_out,
        } => RpuExtractor::extract_rpu(input, stdin, rpu_out, rpu_options),
        Command::Editor {
            input,
            json_file,
            rpu_out,
        } => Editor::edit(input, json_file, rpu_out),
    }
}

pub fn input_format(input: &Path) -> Result<Format, &str> {
    let regex = Regex::new(r"\.(hevc|.?265|mkv)").unwrap();
    let file_name = match input.file_name() {
        Some(file_name) => file_name.to_str().unwrap(),
        None => "",
    };

    if file_name == "-" {
        Ok(Format::RawStdin)
    } else if regex.is_match(file_name) && input.is_file() {
        if file_name.contains("mkv") {
            Ok(Format::Matroska)
        } else {
            Ok(Format::Raw)
        }
    } else if file_name.is_empty() {
        Err("Missing input.")
    } else if !input.is_file() {
        Err("Input file doesn't exist.")
    } else {
        Err("Invalid input file type.")
    }
}
