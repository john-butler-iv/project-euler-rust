use integer_sqrt::IntegerSquareRoot;

pub fn shape<Shape>(n: Shape, sides: usize) -> Shape
where
    Shape: Triangle + Square + Pentagon + Hexagon + Heptagon + Octagon,
{
    match sides {
        3 => Triangle::triangle(n),
        4 => Square::square(n),
        5 => Pentagon::pentagon(n),
        6 => Hexagon::hexagon(n),
        7 => Heptagon::heptagon(n),
        8 => Octagon::octagon(n),
        _ => panic!("unsupported number of sides"),
    }
}

pub fn inverse_shape<Shape>(n: Shape, sides: usize) -> f64
where
    Shape: Triangle + Square + Pentagon + Hexagon + Heptagon + Octagon,
{
    match sides {
        3 => Triangle::inverse_triangle(n),
        4 => Square::inverse_square(n),
        5 => Pentagon::inverse_pentagon(n),
        6 => Hexagon::inverse_hexagon(n),
        7 => Heptagon::inverse_heptagon(n),
        8 => Octagon::inverse_octagon(n),
        _ => panic!("unsupported number of sides"),
    }
}

pub fn inverse_shape_lossy<Shape>(n: Shape, sides: usize) -> Shape
where
    Shape: Triangle + Square + Pentagon + Hexagon + Heptagon + Octagon,
{
    match sides {
        3 => Triangle::inverse_triangle_lossy(n),
        4 => Square::inverse_square_lossy(n),
        5 => Pentagon::inverse_pentagon_lossy(n),
        6 => Hexagon::inverse_hexagon_lossy(n),
        7 => Heptagon::inverse_heptagon_lossy(n),
        8 => Octagon::inverse_octagon_lossy(n),
        _ => panic!("unsupported number of sides"),
    }
}

pub trait Triangle {
    fn triangle(n: Self) -> Self;
    fn inverse_triangle(n: Self) -> f64;
    fn inverse_triangle_lossy(n: Self) -> Self;
}
macro_rules! triangle_impl {
    ( $($prim_type:ty),* ) => { $(
        impl Triangle for $prim_type {
            fn triangle(t: Self)-> Self{
                t * (t + 1) / 2
            }
            fn inverse_triangle(triangle: Self) -> f64 {
                (f64::sqrt((8 * triangle + 1) as f64) - 1.0) / 2.0
            }
            fn inverse_triangle_lossy(triangle: Self) -> Self {
                ((8 * triangle + 1).integer_sqrt() - 1) / 2
            }
        }
    )* };
}
triangle_impl!(u8, u16, u32, u64, u128, usize);
triangle_impl!(i8, i16, i32, i64, i128, isize);

pub trait Square {
    fn square(s: Self) -> Self;
    fn inverse_square(square: Self) -> f64;
    fn inverse_square_lossy(square: Self) -> Self;
}
macro_rules! square_impl {
    ( $($prim_type:ty),* ) => { $(
        impl Square for $prim_type {
            fn square(s: Self)-> Self{
                s*s
            }
            fn inverse_square(square: Self) -> f64 {
                f64::sqrt(square as f64)
            }
            fn inverse_square_lossy(square: Self) -> Self{
                square.integer_sqrt()
            }
        }
    )* };
}
square_impl!(u8, u16, u32, u64, u128, usize);
square_impl!(i8, i16, i32, i64, i128, isize);

pub trait Pentagon {
    fn pentagon(p: Self) -> Self;
    fn inverse_pentagon(pentagon: Self) -> f64;
    fn inverse_pentagon_lossy(pentagon: Self) -> Self;
    fn is_pentagonal(pentagon: Self) -> bool;
}
macro_rules! pentagon_impl {
    ( $($prim_type: ty), *) => { $(
        impl Pentagon for $prim_type {
            #[inline]
            fn pentagon(p: Self) -> Self {
                p * (3 * p - 1) / 2
            }
            #[inline]
            fn inverse_pentagon(pentagon: Self) -> f64 {
                (f64::sqrt((24 * pentagon + 1) as f64) + 1.0) / 6.0
                // NOTE: if a number is pentagonal, (sqrt(24x + 1) + 1) 6 is the index
            }
            #[inline]
            fn inverse_pentagon_lossy(pentagon: Self) -> Self {
                ((24 * pentagon + 1).integer_sqrt()  + 1) / 6
                // NOTE: if a number is pentagonal, (sqrt(24x + 1) + 1) 6 is the index
            }
            fn is_pentagonal(pentagon: Self) -> bool {
                let inverse_pentagon = Pentagon::inverse_pentagon(pentagon);
                inverse_pentagon == inverse_pentagon.trunc()
            }
        }
    )* };
}
pentagon_impl!(u8, u16, u32, u64, u128, usize);
pentagon_impl!(i8, i16, i32, i64, i128, isize);

pub trait Hexagon {
    fn hexagon(h: Self) -> Self;
    fn inverse_hexagon(hexagon: Self) -> f64;
    fn inverse_hexagon_lossy(hexagon: Self) -> Self;
}
macro_rules! hexagon_impl {
    ( $($prim_type:ty), *) => {$(
        impl Hexagon for $prim_type {
            fn hexagon(h: Self) -> Self{
                h * (2 * h - 1)
            }
            fn inverse_hexagon(hexagon: Self)-> f64 {
                return (f64::sqrt((8 * hexagon + 1) as f64) + 1.0) / 4.0;
            }
            fn inverse_hexagon_lossy(hexagon: Self)-> Self {
                return ((8 * hexagon + 1).integer_sqrt() + 1) / 4;
            }
        }
    )* };
}
hexagon_impl!(u8, u16, u32, u64, u128, usize);
hexagon_impl!(i8, i16, i32, i64, i128, isize);

pub trait Heptagon {
    fn heptagon(h: Self) -> Self;
    fn inverse_heptagon(h: Self) -> f64;
    fn inverse_heptagon_lossy(h: Self) -> Self;
}
macro_rules! heptagon_impl {
    ( $($prim_type:ty), *) => {$(
        impl Heptagon for $prim_type {
            fn heptagon(h: Self) -> Self{
                h * (5 * h - 3) / 2
            }
            fn inverse_heptagon(heptagon: Self)-> f64 {
                return (f64::sqrt((40 * heptagon + 9) as f64) +3.0) / 10.0
            }
            fn inverse_heptagon_lossy(heptagon: Self)-> Self {
                return ((40 * heptagon + 9).integer_sqrt() +3) / 10
            }
        }
    )* };
}
heptagon_impl!(u8, u16, u32, u64, u128, usize);
heptagon_impl!(i8, i16, i32, i64, i128, isize);

pub trait Octagon {
    fn octagon(o: Self) -> Self;
    fn inverse_octagon(octagon: Self) -> f64;
    fn inverse_octagon_lossy(octagon: Self) -> Self;
}
macro_rules! ocatgon_impl {
    ( $($prim_type:ty), *) => {$(
        impl Octagon for $prim_type {
            fn octagon(o: Self) -> Self{
                o * (3 * o - 2)
            }
            fn inverse_octagon(octagon: Self)-> f64 {
                (f64::sqrt(((3 * octagon + 1) as f64)) + 1.0) / 3.0
            }
            fn inverse_octagon_lossy(octagon: Self)-> Self {
                ((3 * octagon + 1).integer_sqrt() + 1) / 3
            }
        }
    )* };
}
ocatgon_impl!(u8, u16, u32, u64, u128, usize);
ocatgon_impl!(i8, i16, i32, i64, i128, isize);

mod test_figurate_numbers {

    #[allow(dead_code)]
    fn u32_test_core(known_values: Vec<u32>, compute_value: fn(u32) -> u32, number_label: &str) {
        assert_ne!(known_values.len(), 0, "There are known {}s", number_label);
        for (n, known_value) in known_values.iter().enumerate().skip(1) {
            assert_eq!(
                compute_value(n as u32),
                *known_value,
                "{} #{}",
                number_label,
                n
            );
        }
    }

    #[allow(dead_code)]
    fn u32_inverse_test_core(
        known_values: Vec<u32>,
        compute_inverse_value: fn(u32) -> f64,
        number_label: &str,
    ) {
        assert_ne!(known_values.len(), 0, "There are known {}s", number_label);
        for (n, known_value) in known_values.iter().enumerate().skip(1) {
            let n = n as u32;
            assert_eq!(
                compute_inverse_value(*known_value),
                n as f64,
                "{} {}",
                number_label,
                known_value
            );
        }
    }

    #[allow(dead_code)]
    fn u32_inverse_lossy_test_core(
        known_values: Vec<u32>,
        compute_inverse_lossy_value: fn(u32) -> u32,
        number_label: &str,
    ) {
        assert_ne!(known_values.len(), 0, "There are known {}s", number_label);
        for (n, known_value) in known_values.iter().enumerate().skip(1) {
            let n = n as u32;
            assert_eq!(
                compute_inverse_lossy_value(*known_value),
                n,
                "{} {}",
                number_label,
                known_value
            );
        }
    }

    #[allow(dead_code)]
    fn known_u32_triangles() -> Vec<u32> {
        vec![
            0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120, 136, 153, 171, 190, 210,
            231, 253, 276, 300, 325, 351, 378, 406, 435, 465, 496, 528, 561, 595, 630, 666, 703,
            741, 780, 820, 861, 903, 946, 990, 1035, 1081, 1128, 1176, 1225, 1275, 1326, 1378,
            1431,
        ]
    }

    #[test]
    fn u32_triangles() {
        u32_test_core(known_u32_triangles(), super::Triangle::triangle, "Triangle");
    }

    #[test]
    fn u32_inverse_triangles() {
        u32_inverse_test_core(
            known_u32_triangles(),
            super::Triangle::inverse_triangle,
            "Triangle",
        );
    }

    #[test]
    fn u32_inverse_triangles_lossy() {
        u32_inverse_lossy_test_core(
            known_u32_triangles(),
            super::Triangle::inverse_triangle_lossy,
            "Triangle",
        );
    }

    #[allow(dead_code)]
    fn known_u32_squares() -> Vec<u32> {
        vec![
            0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361,
            400, 441, 484, 529, 576, 625, 676, 729, 784, 841, 900, 961, 1024, 1089, 1156, 1225,
            1296, 1369, 1444, 1521, 1600, 1681, 1764, 1849, 1936, 2025, 2116, 2209, 2304, 2401,
            2500,
        ]
    }

    #[test]
    fn u32_squares() {
        u32_test_core(known_u32_squares(), super::Square::square, "Square");
    }

    #[test]
    fn u32_inverse_square() {
        u32_inverse_test_core(known_u32_squares(), super::Square::inverse_square, "Square");
    }

    #[test]
    fn u32_inverse_square_lossy() {
        u32_inverse_lossy_test_core(
            known_u32_squares(),
            super::Square::inverse_square_lossy,
            "Square",
        );
    }

    #[allow(dead_code)]

    fn known_u32_pentagons() -> Vec<u32> {
        vec![
            0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287, 330, 376, 425, 477, 532,
            590, 651, 715, 782, 852, 925, 1001, 1080, 1162, 1247, 1335, 1426, 1520, 1617, 1717,
            1820, 1926, 2035, 2147, 2262, 2380, 2501, 2625, 2752, 2882, 3015, 3151,
        ]
    }

    #[test]
    fn u32_pentagons() {
        u32_test_core(known_u32_pentagons(), super::Pentagon::pentagon, "Pentagon");
    }

    #[test]
    fn u32_inverse_pentagons() {
        u32_inverse_test_core(
            known_u32_pentagons(),
            super::Pentagon::inverse_pentagon,
            "Pentagon",
        );
    }

    #[test]
    fn u32_inverse_pentagons_lossy() {
        u32_inverse_lossy_test_core(
            known_u32_pentagons(),
            super::Pentagon::inverse_pentagon_lossy,
            "Pentagon",
        );
    }

    #[allow(dead_code)]
    fn known_u32_hexagons() -> Vec<u32> {
        vec![
            0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630,
            703, 780, 861, 946, 1035, 1128, 1225, 1326, 1431, 1540, 1653, 1770, 1891, 2016, 2145,
            2278, 2415, 2556, 2701, 2850, 3003, 3160, 3321, 3486, 3655, 3828, 4005, 4186, 4371,
            4560,
        ]
    }

    #[test]
    fn u32_hexagons() {
        u32_test_core(known_u32_hexagons(), super::Hexagon::hexagon, "Hexagon");
    }

    #[test]
    fn u32_inverse_hexagons() {
        u32_inverse_test_core(
            known_u32_hexagons(),
            super::Hexagon::inverse_hexagon,
            "Hexagon",
        );
    }

    #[test]
    fn u32_inverse_hexagons_lossy() {
        u32_inverse_lossy_test_core(
            known_u32_hexagons(),
            super::Hexagon::inverse_hexagon_lossy,
            "Hexagon",
        );
    }

    #[allow(dead_code)]
    fn known_u32_heptagons() -> Vec<u32> {
        vec![
            0, 1, 7, 18, 34, 55, 81, 112, 148, 189, 235, 286, 342, 403, 469, 540, 616, 697, 783,
            874, 970, 1071, 1177, 1288, 1404, 1525, 1651, 1782, 1918, 2059, 2205, 2356, 2512, 2673,
            2839, 3010, 3186, 3367, 3553, 3744, 3940, 4141, 4347, 4558, 4774, 4995, 5221, 5452,
            5688,
        ]
    }

    #[test]
    fn u32_heptagons() {
        u32_test_core(known_u32_heptagons(), super::Heptagon::heptagon, "Heptagon");
    }

    #[test]
    fn u32_inverse_heptagons() {
        u32_inverse_test_core(
            known_u32_heptagons(),
            super::Heptagon::inverse_heptagon,
            "Heptagon",
        );
    }

    #[test]
    fn u32_inverse_heptagons_lossy() {
        u32_inverse_lossy_test_core(
            known_u32_heptagons(),
            super::Heptagon::inverse_heptagon_lossy,
            "Heptagon",
        );
    }

    #[allow(dead_code)]
    fn known_u32_octagons() -> Vec<u32> {
        vec![
            0, 1, 8, 21, 40, 65, 96, 133, 176, 225, 280, 341, 408, 481, 560, 645, 736, 833, 936,
            1045, 1160, 1281, 1408, 1541, 1680, 1825, 1976, 2133, 2296, 2465, 2640, 2821, 3008,
            3201, 3400, 3605, 3816, 4033, 4256, 4485, 4720, 4961, 5208, 5461,
        ]
    }

    #[test]
    fn u32_octagons() {
        u32_test_core(known_u32_octagons(), super::Octagon::octagon, "Octagon");
    }

    #[test]
    fn u32_inverse_octagons() {
        u32_inverse_test_core(
            known_u32_octagons(),
            super::Octagon::inverse_octagon,
            "Octagon",
        );
    }

    #[test]
    fn u32_inverse_octagons_lossy() {
        u32_inverse_lossy_test_core(
            known_u32_octagons(),
            super::Octagon::inverse_octagon_lossy,
            "Octagon",
        );
    }
}
