#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::isize;

pub struct Point { x: isize, y: isize}
struct Line  { p1: Point, p2: Point }

impl Line {
  pub fn length(&self) -> f64 {
      let xdiff = self.p1.x - self.p2.x;
      let ydiff = self.p1.y - self.p2.y;
      ((isize::pow(xdiff, 2) + isize::pow(ydiff, 2)) as f64).sqrt()
    }
}

#[no_mangle]
pub extern "C" fn make_point(x: isize, y: isize) -> Box<Point> {
    Box::new(Point { x: x, y: y })
}

// *p2 and *p1 raises the error
// cannot move out of borrowed content
// #[no_mangle]
// pub extern "C" fn get_distance(p1: &Point, p2: &Point) -> f64 {
//     Line { p1: *p1, p2: *p2 }.length()
// }
