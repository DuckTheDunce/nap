mod point;
mod tri;
mod render;

fn main() {
    let p1 = point::Point::new(9, 2, 0);
    let p2 = point::Point::new(9, 0, 2);
    let p3 = point::Point::new(2, 0, 0);

    let t1 = tri::Triangle::new(p1, p2, p3);

	let c_pos = point::Point::new(0, 0, 0);
	let f_pos = point::Point::new(1, 0, 0);
    let mut camera = render::Camera::new(c_pos, f_pos);

    camera.render(vec![t1], 16, 16);
}
