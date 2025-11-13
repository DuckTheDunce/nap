mod point;
mod tri;
mod render;

use colored::Colorize;

fn main() {
    let p1 = point::Point::new(4, 0, 0);
    let p2 = point::Point::new(40, 0, 40);
    let p3 = point::Point::new(40, 40, 40);

    let t1 = tri::Triangle::new(p1, p2, p3);

	let c_pos = point::Point::new(0, 0, 0);
	let f_pos = point::Point::new(1, 0, 0);
    let camera = render::Camera::new(c_pos, f_pos, 5.0);

    let t = camera.clone().render(vec![t1.clone()], 32, 18);

	let pixel_vec = t.pixels;
	for i in 0..pixel_vec.len(){
		if i%t.width as usize == 0{
			println!();
		}
    	print!("{}", "o ".truecolor(pixel_vec[i].r, pixel_vec[i].g, pixel_vec[i].b));
		
	}
}
