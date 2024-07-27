#![feature(decl_macro, test)]
extern crate test;

macro benchmark($year:tt, $day:tt) {
    mod $day {
        use advent_of_code::$year::$day::*;
        use test::Bencher;

        #[bench]
        fn part1_bench(b: &mut Bencher) {
            b.iter(|| part1(&INPUT));
        }

        #[bench]
        fn part2_bench(b: &mut Bencher) {
            b.iter(|| part2(&INPUT));
        }
    }
}

mod year_2015 {
    use crate::benchmark;

    benchmark!(year_2015, day_01);
    benchmark!(year_2015, day_02);
    benchmark!(year_2015, day_03);
    benchmark!(year_2015, day_04);
    benchmark!(year_2015, day_05);
    benchmark!(year_2015, day_06);
    benchmark!(year_2015, day_07);
    benchmark!(year_2015, day_08);
    benchmark!(year_2015, day_09);
    benchmark!(year_2015, day_10);
    benchmark!(year_2015, day_11);
    benchmark!(year_2015, day_12);
    benchmark!(year_2015, day_13);
    benchmark!(year_2015, day_14);
    benchmark!(year_2015, day_15);
    benchmark!(year_2015, day_16);
    benchmark!(year_2015, day_17);
    benchmark!(year_2015, day_18);
    benchmark!(year_2015, day_19);
    benchmark!(year_2015, day_20);
    benchmark!(year_2015, day_21);
    benchmark!(year_2015, day_22);
    benchmark!(year_2015, day_23);
    benchmark!(year_2015, day_24);
    benchmark!(year_2015, day_25);
}

mod year_2016 {
    use crate::benchmark;

    benchmark!(year_2016, day_01);
    benchmark!(year_2016, day_02);
    benchmark!(year_2016, day_03);
    benchmark!(year_2016, day_04);
    benchmark!(year_2016, day_05);
    benchmark!(year_2016, day_06);
    benchmark!(year_2016, day_07);
    benchmark!(year_2016, day_08);
    benchmark!(year_2016, day_09);
    benchmark!(year_2016, day_10);
    benchmark!(year_2016, day_11);
    benchmark!(year_2016, day_12);
    benchmark!(year_2016, day_13);
    benchmark!(year_2016, day_14);
    benchmark!(year_2016, day_15);
    benchmark!(year_2016, day_16);
    benchmark!(year_2016, day_17);
    benchmark!(year_2016, day_18);
    benchmark!(year_2016, day_19);
    benchmark!(year_2016, day_20);
    benchmark!(year_2016, day_21);
    benchmark!(year_2016, day_22);
    benchmark!(year_2016, day_23);
    benchmark!(year_2016, day_24);
    benchmark!(year_2016, day_25);
}
