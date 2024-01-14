pub struct Php5Random {
    r: Vec<u32>,
    k: usize,
}

impl Php5Random {
    pub fn new(seed: u32) -> Php5Random {
        let mut phpr = Php5Random {
            r: vec![0; 34],
            k: 0,
        };
        phpr.srand(seed);
        phpr
    }

    pub fn srand(&mut self, seed: u32) {
        self.r = vec![0; 34];
        self.r[0] = seed;

        for i in 1..31 {
            self.r[i] = ((16807_u64 * self.r[i - 1] as u64) % 2147483647) as u32;
        }

        for i in 31..34 {
            self.r[i] = self.r[i - 31];
        }

        self.k = 0;

        for _ in 0..310 {
            _ = &self.rand();
        }
    }

    pub fn rand(&mut self) -> u32 {
        let k_as_isize = self.k as isize;
        self.r[self.k] = (self.r[(k_as_isize - 31).rem_euclid(34) as usize] as i64
            + self.r[(k_as_isize - 3).rem_euclid(34) as usize] as i64)
            as u32;
        let r = self.r[self.k] >> 1;
        self.k = (self.k + 1) % 34;
        r
    }

    pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
        let r = self.rand();
        (min as f64 + ((max as f64 - min as f64 + 1.0) * (r as f64 / 2147483647_f64))) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_seed_rand() {
        // all random numbers generated with seed 0 are 0, let's make sure
        let mut phpr = Php5Random::new(0);
        for _ in 0..100 {
            assert_eq!(phpr.rand(), 0);
        }
    }

    #[test]
    fn one_seed_rand() {
        // all random numbers generated with seed 1 are expected to be these values
        let mut phpr = Php5Random::new(1);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand());
        }

        assert_eq!(
            v,
            vec![
                1804289383, 846930886, 1681692777, 1714636915, 1957747793, 424238335, 719885386,
                1649760492, 596516649, 1189641421, 1025202362, 1350490027, 783368690, 1102520059,
                2044897763, 1967513926, 1365180540, 1540383426, 304089172, 1303455736, 35005211,
                521595368, 294702567, 1726956429, 336465782, 861021530, 278722862, 233665123,
                2145174067, 468703135, 1101513929, 1801979802, 1315634022, 635723058, 1369133069,
                1125898167, 1059961393, 2089018456, 628175011, 1656478042, 1131176229, 1653377373,
                859484421, 1914544919, 608413784, 756898537, 1734575198, 1973594324, 149798315,
                2038664370, 1129566413, 184803526, 412776091, 1424268980, 1911759956, 749241873,
                137806862, 42999170, 982906996, 135497281, 511702305, 2084420925, 1937477084,
                1827336327, 572660336, 1159126505, 805750846, 1632621729, 1100661313, 1433925857,
                1141616124, 84353895, 939819582, 2001100545, 1998898814, 1548233367, 610515434,
                1585990364, 1374344043, 760313750, 1477171087, 356426808, 945117276, 1889947178,
                1780695788, 709393584, 491705403, 1918502651, 752392754, 1474612399, 2053999932,
                1264095060, 1411549676, 1843993368, 943947739, 1984210012, 855636226, 1749698586,
                1469348094, 1956297539
            ]
        );
    }

    #[test]
    fn one_seed_rand_range() {
        let mut phpr = Php5Random::new(1);
        let mut v = Vec::new();
        for _ in 0..100 {
            v.push(phpr.rand_range(0, 100));
        }
        assert_eq!(
            v,
            [
                84, 39, 79, 80, 92, 19, 33, 77, 28, 55, 48, 63, 36, 51, 96, 92, 64, 72, 14, 61, 1,
                24, 13, 81, 15, 40, 13, 10, 100, 22, 51, 84, 61, 29, 64, 52, 49, 98, 29, 77, 53,
                77, 40, 90, 28, 35, 81, 92, 7, 95, 53, 8, 19, 66, 89, 35, 6, 2, 46, 6, 24, 98, 91,
                85, 26, 54, 37, 76, 51, 67, 53, 3, 44, 94, 94, 72, 28, 74, 64, 35, 69, 16, 44, 88,
                83, 33, 23, 90, 35, 69, 96, 59, 66, 86, 44, 93, 40, 82, 69, 92
            ]
        );
    }
}
