fn main() {
    println!("Hello, world! {}", sum(1, 2));
}

fn sum(p1: u32, p2: u32) -> u32 {
    p1 + p2 + 1
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sum_test() {
        assert_eq!(sum(10, 20), 30);
    }
}
