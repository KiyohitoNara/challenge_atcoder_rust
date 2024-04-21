use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut count = 0;
    for i in 1..=n {
        if i % 2 == 1 {
            let mut divisor = 0;
            for j in 1..=i {
                if i % j == 0 {
                    divisor += 1;
                }
            }

            if divisor == 8 {
                count += 1;
            }
        }
    }

    println!("{}", {count});
}
