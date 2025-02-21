extern crate nalgebra as na;

use nalgebra::Vector2;
pub fn reduce(mut v1: Vector2<i128>, mut v2: Vector2<i128>) -> (Vector2<i128>, Vector2<i128>) {
    loop {
        if v1.dot(&v1) > v2.dot(&v2) {
            std::mem::swap(&mut v1, &mut v2);
        }

        let m = v1.dot(&v2) / v1.dot(&v1);
        if m == 0 {
            return (v1, v2);
        }
        let scaled_v1 = v1.map(|x| x*m);
        v2 = v2 - scaled_v1;
    }
}
