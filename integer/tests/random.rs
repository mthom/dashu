use dashu_int::{ops::DivRem, UBig};
use rand::{distributions::uniform::Uniform, prelude::*};

mod helper_macros;

#[test]
fn test_uniform_ubig() {
    let mut rng = StdRng::seed_from_u64(1);

    let distr = Uniform::from(ubig!(3)..ubig!(7));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ubig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ubig!(6));

    let distr = Uniform::from(ubig!(3)..=ubig!(7));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ubig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ubig!(7));

    let distr = Uniform::from(ubig!(0b100) << 128..ubig!(0b1000) << 128);
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert!(x >= ubig!(0b100) << 128 && x < ubig!(0b101) << 128);
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert!(x >= ubig!(0b111) << 128 && x < ubig!(0b1000) << 128);
}

#[test]
fn test_uniform_ibig() {
    let mut rng = StdRng::seed_from_u64(1);

    let distr = Uniform::from(ibig!(-7)..ibig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ibig!(-7));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ibig!(2));

    let distr = Uniform::from(ibig!(-7)..=ibig!(3));
    let x = (&mut rng).sample_iter(&distr).take(1000).min().unwrap();
    assert_eq!(x, ibig!(-7));
    let x = (&mut rng).sample_iter(&distr).take(1000).max().unwrap();
    assert_eq!(x, ibig!(3));
}

#[test]
fn test_random_arithmetic() {
    let mut rng = StdRng::seed_from_u64(3);
    let p = ubig!(1000000007);

    // 10^2 bits: 10^5 cases
    // ..to..
    // 10^6 bits: 10 cases
    for log_num_bits in 2..=6 {
        let num_bits = match 10usize.checked_pow(log_num_bits) {
            None => continue,
            Some(x) => x,
        };
        let num_cases = 10u32.pow(7 - log_num_bits);
        for i in 0..num_cases {
            let len_a = (&mut rng).gen_range(10..num_bits);
            let len_b = (&mut rng).gen_range(10..num_bits);
            let a = (&mut rng).gen_range(ubig!(100)..ubig!(1) << len_a);
            let b = (&mut rng).gen_range(ubig!(100)..ubig!(1) << len_b);
            let c = (&mut rng).sample(Uniform::new(ubig!(0), &a));
            let radix = (&mut rng).gen_range(2..=36);

            assert_eq!((&a + &b) % &p, ((&a % &p) + (&b % &p)) % &p);
            assert_eq!(&a + &b - &a, b);
            assert_eq!((&a * &b) % &p, ((&a % &p) * (&b % &p)) % &p);
            let (quot, rem) = (&a * &b + &c).div_rem(&a);
            assert_eq!(quot, b);
            assert_eq!(rem, c);
            assert_eq!(
                UBig::from_str_radix(&a.in_radix(radix).to_string(), radix).unwrap(),
                a
            );
            assert_eq!((&a + UBig::ONE) * (&a - UBig::ONE), a.square() - UBig::ONE);

            // gcd is much slower than primitive operations, test with lower frequency
            if i % 32 == 0 {
                let (g, ca, cb) = a.gcd_ext(&b);
                assert_eq!(g, a.gcd(&b));
                assert_eq!(&a % &g, 0);
                assert_eq!(&b % &g, 0);
                assert_eq!(g, a * ca + b * cb);
            }
        }
    }
}
