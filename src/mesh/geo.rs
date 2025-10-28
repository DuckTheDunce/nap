use crate::render::{Pixel, Mask};

pub struct Point{
	x:i32,
	y:i32,
	z:i32
}
impl Point{
	pub fn new(x:i32, y:i32, z:i32) -> Point{
		return Point{x:x,y:y,z:z}
	}
}

pub struct Triangle{
	points:[Point; 3],
	x:i32,
	y:i32,
	z:i32
}
impl Triangle{
	pub fn new(points:[Point;3], x:i32, y:i32, z:i32) -> Triangle{
		return Triangle{points:points, x:x, y:y, z:z}
	}

	pub fn press(&self) -> Mask{
		//get the line heights and widths, get the aperent size of the lines
		let l1 = [self.points[0].x - self.points[1].x, self.points[0].y - self.points[1].y];

	}
}

pub struct Mesh{
	triangles:Vec<Triangle>,
	x:i32,
	y:i32,
	z:i32
}
