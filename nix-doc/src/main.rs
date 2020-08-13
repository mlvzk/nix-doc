//! A nix documentation search program

use nix_doc::{is_searchable, search, Result};

use regex::Regex;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_path() -> PathBuf {
    let channel_path = Command::new("nix-instantiate")
        .arg("--eval")
        .arg("--strict")
        .arg("-E")
        .arg("<nixpkgs>")
        .output()
        .map(|o| String::from_utf8(o.stdout));

    if let Ok(Ok(path)) = channel_path {
        PathBuf::from(path.trim_end())
    } else {
        PathBuf::from(".")
    }
}

fn main() -> Result<()> {
    let args = env::args().skip(1);
    let re_match = args.collect::<String>();
    let file = get_path();
    if re_match == "" {
        eprintln!("Usage: nix-doc SearchRegex");
        return Ok(());
    }

    let re_match = Regex::new(&re_match)?;
    search(&file, re_match, is_searchable);
    Ok(())
}
