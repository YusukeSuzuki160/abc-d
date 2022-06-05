use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut sq = Vec::new();
    for _i in 0..=n {
        sq.push(false);
    }
    for i in 1..=((n as f64).sqrt() as usize) {
        sq[i * i] = true;
    }
    let mut d = Vec::new();
    for _i in 0..=n {
        let m = Vec::new();
        d.push(m);
    }
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            d[j].push(i);
        }
    }

    let mut cnt: Vec<usize> = Vec::new();
    for _i in 0..=n {
        cnt.push(0);
    }
    for i in 1..=n {
        let mut f = 0;
        for j in 0..d[i].len() {
            if sq[d[i][j]] {
                f = d[i][j];
            }
        }
        cnt[i / f] += 1;
    }
    let mut ans = 0;
    for i in 1..=n {
        ans += cnt[i] * cnt[i];
    }
    println!("{}", ans);
}
