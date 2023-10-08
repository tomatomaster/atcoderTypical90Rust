use proconio::input;

fn main() {
    input! { n : usize}

    if n % 2 != 0 {
        return;
    }

    let sum = n / 2;
    let mut results: Vec<String> = Vec::new();
    make_kakko("", 0, 0, sum, &mut results);
    for r in results {
        println!("{}", r);
    }
}

fn make_kakko(moji: &str, kakko: usize, toji: usize, sum: usize, results: &mut Vec<String>) {
    if kakko == toji && kakko == sum {
        results.push(moji.to_string());
    }

    if kakko > sum || toji > sum {
        return;
    }

    if kakko >= toji {
        let mut r = String::from(moji);
        r.push_str("(");
        make_kakko(&r, kakko + 1, toji, sum, results);
        let mut r = String::from(moji);
        r.push_str(")");
        make_kakko(&r, kakko, toji + 1, sum, results);
    } else {
        return;
    }
}
