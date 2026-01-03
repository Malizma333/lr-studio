#[cfg(test)]
mod tests {
    use lcg_random::Random;

    #[test]
    fn rand() {
        let mut rng = Random::from_seed(1);

        let x1 = dbg!(rng.rand());
        let x2 = dbg!(rng.rand());
        assert_ne!(x1, x2);

        let mut rng = Random::from_seed(2);
        let x3 = dbg!(rng.rand());
        assert_ne!(x1, x3);

        for _ in 0..1000 {
            let next = rng.rand();
            assert!(0.0 <= next);
            assert!(next < 1.0);
        }
    }

    #[test]
    fn rand_from_time() {
        let mut rng = Random::new();
        let mut rng2 = Random::new();
        assert_ne!(rng.rand(), rng2.rand());
    }

    #[test]
    fn rand_choice() {
        let mut rng = Random::from_seed(1);

        let items = vec![1, 3, 5, 7];
        let x = dbg!(rng.rand_choice(items.as_slice()).unwrap());
        assert!(items.contains(x));

        let y = dbg!(rng.rand_choice(items.as_slice()).unwrap());
        assert!(items.contains(y));

        let empty_items = Vec::<u8>::new();
        let x = dbg!(rng.rand_choice(empty_items.as_slice()));
        assert!(x.is_err());
    }

    #[test]
    fn rand_range_f64() {
        let mut rng = Random::from_seed(1);

        let x = dbg!(rng.rand_range_f64(0.0, 100.0).unwrap());
        assert!(x >= 0.0);
        assert!(x < 100.0);

        let y = dbg!(rng.rand_range_f64(0.0, 100.0).unwrap());
        assert!(y >= 0.0);
        assert!(y < 100.0);
    }

    #[test]
    fn rand_range_u32() {
        let mut rng = Random::from_seed(1);

        let x = dbg!(rng.rand_range_u32(10, 100).unwrap());
        assert!(x >= 10);
        assert!(x < 100);

        let y = dbg!(rng.rand_range_u32(10, 100).unwrap());
        assert!(y >= 10);
        assert!(y < 100);
    }

    #[test]
    fn rand_range_i32() {
        let mut rng = Random::from_seed(2);

        let x = dbg!(rng.rand_range_i32(-100, 100).unwrap());
        assert!(x >= -100);
        assert!(x < 100);

        let y = dbg!(rng.rand_range_i32(-100, 100).unwrap());
        assert!(y >= -100);
        assert!(y < 100);
    }

    #[test]
    fn rand_str() {
        let mut rng = Random::from_seed(1);

        let rand_string = dbg!(rng.rand_str(10));
        assert!(rand_string.len() == 10);
    }
}
