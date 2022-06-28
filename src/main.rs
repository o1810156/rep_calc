use std::cmp::{Eq, PartialEq};
use std::fmt::Display;

#[derive(Debug, Clone)]
struct Replacement {
    original: Vec<Vec<usize>>,
    table: Vec<Vec<usize>>,
    k: usize,
}

impl Replacement {
    fn e() -> Self {
        Replacement {
            original: vec![],
            table: vec![],
            k: 0,
        }
    }

    fn new(table: Vec<Vec<usize>>) -> Self {
        let original = table.clone();
        if table.len() == 0 {
            return Self::e();
        }

        for v in table.iter() {
            if v.len() == 0 {
                panic!("empty vector");
            }
        }

        let &k = table.iter().map(|v| v.iter().max().unwrap()).max().unwrap();

        let table = table
            .iter()
            .rev()
            .map(|v| {
                let mut w = vec![0; k + 1];
                for i in 0..(v.len() - 1) {
                    w[v[i]] = v[i + 1];
                }
                w[v[v.len() - 1]] = v[0];
                w
            })
            .collect::<Vec<_>>();

        Self { original, table, k }
    }

    fn get_k(&self) -> usize {
        self.k
    }

    fn get_correct_k(&self) -> Option<usize> {
        for i in (1..=self.k).rev() {
            if self.replace(i) != i {
                return Some(i);
            }
        }
        None
    }

    fn replace(&self, i: usize) -> usize {
        if i == 0 {
            panic!("index 0 is not allowed");
        }

        if i > self.k {
            return i;
        }

        let mut dist = i;
        for v in self.table.iter() {
            let next = v[dist];
            dist = if next > 0 { next } else { dist };
        }
        dist
    }

    fn concat_before(&self, other: &Self) -> Self {
        let mut new_table = self.original.clone();
        new_table.extend_from_slice(&other.original);
        Self::new(new_table)
    }

    fn rev_find(&self, val: usize) -> Option<usize> {
        if val == 0 {
            panic!("Dist 0 is not allowed");
        }

        for i in 1..=self.k {
            if self.replace(i) == val {
                return Some(i);
            }
        }
        None
    }

    fn rearrange(&self) -> Self {
        let k = match self.get_correct_k() {
            Some(k) => k,
            None => return Self::e(),
        };

        let mut new_table = vec![];
        let mut book = vec![true; k + 1];
        book[0] = false;
        for i in 1..=k {
            if book[i] {
                book[i] = false;
                let mut dist = self.replace(i);

                if dist == i {
                    continue;
                }

                let mut chain = vec![i];
                while dist != chain[0] {
                    chain.push(dist);
                    book[dist] = false;
                    dist = self.replace(dist);
                }
                new_table.push(chain);

                continue;
            }
        }

        Replacement::new(new_table)
    }
}

impl PartialEq for Replacement {
    fn eq(&self, other: &Self) -> bool {
        let k = self.get_k().max(other.get_k());
        for i in 1..=k {
            // println!("{}: {} | {}", i, self.replace(i), other.replace(i));
            if self.replace(i) != other.replace(i) {
                return false;
            }
        }
        true
    }
}

impl Eq for Replacement {}

impl Display for Replacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.k == 0 {
            return write!(f, "e");
        }

        let mut res = vec![];
        for v in self.original.iter() {
            let s = format!(
                "({})",
                v.iter()
                    .map(|&i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            res.push(s);
        }
        write!(f, "{}", res.join(""))
    }
}

#[derive(Debug, Clone, Copy)]
struct Alter123 {
    start: usize,
    e: usize,
}

impl Alter123 {
    fn new(start: usize, e: usize) -> Self {
        if e == 0 || e > 2 {
            panic!("e must be 1 or 2");
        }
        if start <= 0 {
            panic!("start must be positive");
        }
        Self { start, e }
    }

    fn rev(&self) -> Self {
        Alter123 {
            start: self.start,
            e: self.e ^ 3,
        }
    }

    fn get_rep(&self) -> Replacement {
        let mut table = vec![];
        let v = vec![self.start, self.start + 1, self.start + 2];
        for _ in 0..self.e {
            table.push(v.clone());
        }
        Replacement::new(table)
    }
}

impl Display for Alter123 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_rep())
    }
}

fn into_alter_product(r: &Replacement) -> Option<Replacement> {
    if r.get_correct_k().is_none() {
        println!("This is e: {}", r);
        return None;
    }

    let mut r = r.clone();
    let mut ck = r.get_correct_k().unwrap();
    let mut alter_book = vec![];

    while ck > 3 {
        println!("r: {}", r);
        println!("r rearrange: {}", r.rearrange());
        let i = match r.rev_find(ck) {
            Some(i) => i,
            None => return None,
        };
        println!("i: {}", i);
        let t = tau(i, ck);
        println!("tau: {:?}", t);
        for alt in t.iter().map(|alt| alt.get_rep()) {
            r = r.concat_before(&alt);
        }
        alter_book.extend_from_slice(&t);

        ck = match r.get_correct_k() {
            Some(v) => v,
            None => {
                // println!("E was generated: {}", r);
                // return None;
                break;
            }
        };
    }

    alter_book.reverse();
    let alter_book = alter_book
        .into_iter()
        .map(|alt| alt.rev().get_rep())
        .collect::<Vec<_>>();
    let mut rear_r = r.rearrange();

    for alt in alter_book.iter() {
        rear_r = rear_r.concat_before(&alt);
    }

    Some(rear_r)
}

fn tau(k: usize, n: usize) -> Vec<Alter123> {
    if k >= n {
        panic!("k must be less than n");
    }

    if n <= 3 {
        panic!("n must be greater than 3");
    }

    if k == n - 1 {
        vec![Alter123::new(n - 2, 2)]
    } else {
        let mut res = vec![Alter123::new(k, 1)];
        for i in k + 1..=n - 2 {
            res.push(Alter123::new(i, 2));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_replacement() {
        let v = vec![vec![1, 2, 3, 4, 5]];
        let r = Replacement::new(v);
        assert_eq!(r.get_k(), 5);
        assert_eq!(r.replace(1), 2);
        assert_eq!(r.replace(2), 3);
        assert_eq!(r.replace(3), 4);
        assert_eq!(r.replace(4), 5);
        assert_eq!(r.replace(5), 1);
    }

    #[test]
    fn test_eq() {
        let v1 = vec![vec![1, 2, 3, 4, 5]];
        let r1 = Replacement::new(v1);
        let v2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let r2 = Replacement::new(v2);
        assert_eq!(r1, r2);
    }
}

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
