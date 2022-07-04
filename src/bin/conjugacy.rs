use itertools::Itertools;
use rep_calc::*;
use std::collections::{BTreeSet, HashMap};

fn calc(k: usize) {
    println!("\n=================== {} ==================\n", k);

    let gs = (1..=k)
        .permutations(k)
        .map(|v| Replacement::from_replaced_table(k, v.to_vec()))
        .collect::<Vec<_>>();
    let xs = gs.clone();

    let gs_gs_inv = gs
        .into_iter()
        .map(|g| {
            let g_rev = g.rev();
            (g, g_rev)
        })
        .collect::<Vec<_>>();

    let mut res = HashMap::new();
    let mut res_centers = HashMap::new();

    for x in xs.iter() {
        let mut x_orbit = BTreeSet::new();
        let mut centers = BTreeSet::new();

        for (g, g_inv) in gs_gs_inv.iter() {
            let r = g.concat_before(x).concat_before(g_inv).rearrange();
            if &r == x {
                centers.insert(g.clone());
            }

            x_orbit.insert(r);
        }

        let _ = res_centers.insert(x_orbit.clone(), centers);
        let kind = res.entry(x_orbit).or_insert(Vec::new());
        kind.push(x);
    }

    println!("\nres.len() = {}\n", res.len());

    for (kind, elms) in res.iter() {
        let centers = res_centers.get(kind).unwrap();
        /*
        println!(
            "#(elms: {}, kind: {}) [{}] <= [{}]\ncenters #({}): [{}]",
            elms.len(),
            kind.len(),
            kind.iter().collect::<Vec<_>>()
                .chunks(8).map(|c| c.iter().map(|r| r.to_string()).join(", ")).join("\n"),
            elms.chunks(8).map(|c| c.iter().map(|r| r.to_string()).join(", ")).join("\n"),
            centers.len(),
            centers.iter().collect::<Vec<_>>()
                .chunks(8).map(|c| c.iter().map(|r| r.to_string()).join(", ")).join("\n")
        );
        */
        println!(
            "#(elms: {}, kind: {}) [{}]\ncenters #({}): [{}]",
            elms.len(),
            kind.len(),
            kind.iter().collect::<Vec<_>>()
                .chunks(8).map(|c| c.iter().map(|r| r.to_string()).join(", ")).join("\n"),
            // elms.chunks(8).map(|c| c.iter().map(|r| r.to_string()).join(", ")).join("\n"),
            centers.len(),
            centers.iter().collect::<Vec<_>>()
                .chunks(8).map(|c| c.iter().map(|r| r.to_string()).join(", ")).join("\n")
        );
    }
}

fn main() {
    calc(3);
    calc(4);
    calc(5);
    calc(6);
}
