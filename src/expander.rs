use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

lazy_static! {
    static ref INCLUDE_REGEX: Regex =
        Regex::new("#include\\s*[\"<](santolib/[a-z_]+.hpp)[\">]\\s*").unwrap();
}

pub fn expand_file(file: PathBuf, lib: PathBuf) -> io::Result<String> {
    let mut result: Vec<String> = vec![];
    let mut included: HashSet<String> = HashSet::new();

    let file = File::open(file)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if INCLUDE_REGEX.is_match(&line) {
            let name = INCLUDE_REGEX.captures(&line).unwrap()[1].to_string();
            expand_lib(name, &lib, &mut included, &mut result)?;
        } else {
            result.push(line);
        }
    }

    Ok(result.join("\n"))
}

fn expand_lib(
    name: String,
    lib: &PathBuf,
    included: &mut HashSet<String>,
    result: &mut Vec<String>,
) -> io::Result<()> {
    if included.contains(&name) {
        return Ok(());
    }

    let path = {
        let mut path = lib.clone();
        path.push(&name);
        path
    };
    included.insert(name);

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        if INCLUDE_REGEX.is_match(&line) {
            let name = INCLUDE_REGEX.captures(&line).unwrap()[1].to_string();
            expand_lib(name, &lib, included, result)?;
        } else if !should_ignore_line(&line) {
            result.push(line);
        }
    }

    Ok(())
}

fn should_ignore_line(line: &String) -> bool {
    let line = line.trim();

    if line == "#pragma once" || line.starts_with("//") {
        true
    } else {
        false
    }
}
