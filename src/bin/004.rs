use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut mat_a: [[i64;w];h],
    }

    let mut yoko = [0; 100000];
    let mut tate = [0; 100000];

    for i in 0..h {
        for j in 0..w {
            yoko[i] += mat_a[i][j];
            tate[j] += mat_a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", yoko[i] + tate[j] - mat_a[i][j]);
        }
        println!("");
    }
}
