use rep_calc::*;

// #[macro_use]
// extern crate anyhow;

use anyhow::{Ok, Result};
use regex::Regex;
use std::io::{stdin, stdout, Write};

fn main() -> Result<()> {
    print!("mode > ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    match input.as_str() {
        "eq" => eq_mode()?,
        "alter" => alter_mode()?,
        "re" => rearrange_mode()?,
        _ => (),
    }

    Ok(())
}

fn eq_mode() -> Result<()> {
    print!("r1 > ");
    stdout().flush()?;
    let r1 = input_replaces()?;
    print!("r2 > ");
    stdout().flush()?;
    let r2 = input_replaces()?;

    if r1 == r2 {
        println!("Same");
    } else {
        println!("Different");
        let k = r1.get_k().max(r2.get_k());
        for i in 1..=k {
            println!("{}: {} | {}", i, r1.replace(i), r2.replace(i));
        }
    }

    Ok(())
}

use once_cell::sync::Lazy;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\([1-9\s]+\)").unwrap());
static BRS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\(\)]").unwrap());

fn input_replaces() -> Result<Replacement> {
    let mut table = vec![];
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let matches = RE.find_iter(&input);
    for m in matches {
        let v = BRS
            .replace_all(m.as_str(), "")
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        table.push(v);
    }

    Ok(Replacement::new(table))
}

fn alter_mode() -> Result<()> {
    print!("r > ");
    stdout().flush()?;
    let r = input_replaces()?;
    let rearrenged = into_alter_product(&r).unwrap();

    println!("alter res: {}", rearrenged);

    if r == rearrenged {
        println!("Same");
    } else {
        println!("Different");
        let k = r.get_k().max(rearrenged.get_k());
        for i in 1..=k {
            println!("{}: {} | {}", i, r.replace(i), rearrenged.replace(i));
        }
    }

    Ok(())
}

fn rearrange_mode() -> Result<()> {
    print!("r > ");
    stdout().flush()?;
    let r = input_replaces()?;
    let rearrenged = r.rearrange();

    println!("rearrange res: {}", rearrenged);

    Ok(())
}
