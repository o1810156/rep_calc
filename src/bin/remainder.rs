use itertools::Itertools;
use rep_calc::*;
use std::collections::HashSet;

fn main() {
    let h_group = vec![[1, 2, 3, 4], [3, 2, 1, 4], [1, 4, 3, 2], [3, 4, 1, 2]]
        .into_iter()
        .map(|v| Replacement::from_replaced_table(4, v.to_vec()))
        .collect::<Vec<_>>();

    println!("(2) 右完全代表系");

    let mut res_all = vec![];
    let mut res = vec![];
    let mut already_appered = HashSet::new();

    for perm in (1..=4).permutations(4) {
        let g = Replacement::from_replaced_table(4, perm);
        let g = g.rearrange();

        let dont_skip = !already_appered.contains(&g);

        let ghs = h_group
            .iter()
            .map(|h| {
                let gh = g.concat_before(h);
                let res = gh.rearrange();
                already_appered.insert(res.clone());
                res
            })
            .collect::<Vec<_>>();

        if dont_skip {
            res.push((g.clone(), ghs.clone()));
        }

        res_all.push((g, ghs));
    }

    for (g, ghs) in res_all.iter() {
        println!(
            "{} => [{}]",
            g,
            ghs.iter().map(|gh| gh.to_string()).join(", ")
        );
    }

    println!("========================================");

    for (g, ghs) in res_all.iter() {
        println!(
            "{} => [{}]",
            g,
            ghs.iter()
                .zip(h_group.iter())
                .map(|(gh, h)| format!("{} -> {}", h, gh))
                .join(", ")
        );
    }

    println!("========================================");

    for (g, ghs) in res.iter() {
        println!(
            "{} => [{}]",
            g,
            ghs.iter().map(|gh| gh.to_string()).join(", ")
        );
    }

    println!("========================================");

    for (g, ghs) in res.iter() {
        println!(
            "{} => [{}]",
            g,
            ghs.iter()
                .zip(h_group.iter())
                .map(|(gh, h)| format!("{} -> {}", h, gh))
                .join(", ")
        );
    }

    let ghs: Vec<_> = res.into_iter().map(|t| t.1).collect();

    println!("\n(3) 正規部分群判定");

    let mut h_hashset = HashSet::new();
    for h in h_group.iter() {
        h_hashset.insert(h.clone());
    }

    let mut flag = true;
    for perm in (1..=4).permutations(4) {
        let g = Replacement::from_replaced_table(4, perm);
        let g = g.rearrange();
        let g_inv = g.rev();

        for h in h_group.iter() {
            let g_h_g_inv = g.concat_before(h).concat_before(&g_inv);
            if !h_hashset.contains(&g_h_g_inv) {
                println!("{} is not in H", g_h_g_inv);
                flag = false;
                // break 'reg_loop;
            }
        }
    }

    println!("reg?: {:?}", flag);

    println!("\n(4) (1 2 3 4) in G の gH への作用を調べる");

    let g = Replacement::new(vec![vec![1, 2, 3, 4]]);

    for gh in ghs.iter() {
        let r_group = gh
            .iter()
            .map(|elm| g.concat_before(elm).rearrange())
            .collect::<Vec<_>>();

        let gh_s = gh.iter().map(|t| t.to_string()).join(", ");
        let r_group_s = r_group.iter().map(|t| t.to_string()).join(", ");
        println!("{} => {}", gh_s, r_group_s);
    }
}
