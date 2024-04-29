use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [String; N],
        B: [String; N],
    }

    for i in 0..N {
        for j in 00..N {
            if A[i].chars().nth(j) != B[i].chars().nth(j) {
                println!("{} {}", { i + 1 }, { j + 1 });
            }
        }
    }
}
