use proconio::input;

fn main() {
    input! {
        A: [i32; 9],
        B: [i32; 8]
    }

    let mut tt = 0;
    for i in 0..9 {
        tt += A[i];
    }

    let mut ta = 0;
    for j in 0..8 {
        ta += B[j];
    }

    println!("{}", { tt - ta + 1 });
}
