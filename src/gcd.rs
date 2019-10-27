
/// Returns a 64-bit unsigned integer
///
/// # Arguments
///
/// * `n` - 64-bit unsigned integer
/// * `m` - 64 bit unsigned integer
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if n > m {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(5, 10), 5);
    assert_eq!(gcd(99, 66), 33);
}
