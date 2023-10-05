use proconio::input;

fn main() {
    input! {
        n: i32,
        l: i32,
        k: i32,
        mut a: [i32; n]
    }

    let mut left = -1;
    let mut right = l + 1;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if check(mid, n as usize, &mut a, l, k) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}

fn check(x: i32, n: usize, a: &mut [i32], l: i32, k: i32) -> bool {
    let mut num = 0;
    let mut pre = 0;
    for i in 0..n {
        if a[i] - pre >= x {
            num += 1;
            pre = a[i];
        }
    }

    if l - pre >= x {
        num += 1;
    }

    num >= k + 1
}
