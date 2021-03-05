use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::io;

fn main() {
    let mut count = HashMap::new();
    let f = compose(roll_drop_low_4d6, point_buy_v1);
    let mut temp = String::with_capacity(10);
    io::stdin().read_line(&mut temp).unwrap();
    let n: usize = temp.trim().parse().unwrap();
    for _ in 0..n {
        let temp: i32 = (0..6).map(|_| f(())).sum();
        if let Some(x) = count.get_mut(&temp) {
            *x += 1;
        } else {
            count.insert(temp, 1);
        }
    }
    let mut out = count.iter().collect::<Vec<(_, _)>>();
    out.sort();
    for (k, v) in out {
        println!("{:4}: {}", k, v);
    }
}

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn roll_drop_low_4d6(_: ()) -> i32 {
    let mut rng = rand::thread_rng();
    let roll = Uniform::new_inclusive(1, 6);
    let mut rolls = (0..4).map(|_| roll.sample(&mut rng)).collect::<Vec<i32>>();
    rolls.sort_unstable();
    rolls.iter().skip(1).sum()
}

fn roll_3d6(_: ()) -> i32 {
    let mut rng = rand::thread_rng();
    let roll = Uniform::new_inclusive(1, 6);
    let mut rolls = (0..3).map(|_| roll.sample(&mut rng)).collect::<Vec<i32>>();
    rolls.iter().sum()
}

fn point_buy_v1(score: i32) -> i32 {
    match score {
        score if score < 15 => score - 8,
        score if score >= 15 => 7 + 2 * (score - 14),
        _ => 0,
    }
}
fn point_buy_v2(score: i32) -> i32 {
    match score {
        score if score < 8 => 2 * (score - 8),
        score if 8 <= score && score < 15 => score - 8,
        score if score >= 15 => 7 + 3 * (score - 14),
        _ => 0,
    }
}
