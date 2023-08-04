extern crate nalgebra as na;

fn main() {
    let v = na::Vector3::new(1, 2, 3);

    let m = na::Matrix3x4::new( 11, 12, 13, 14,
                            21, 22, 23, 24,
                            31, 32, 33, 34);

    println!("{v}");

    println!("{m}");

}
