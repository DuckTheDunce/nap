#[derive(Clone, Copy)]
pub struct Point{
	x:i32,
	y:i32,
	z:i32
}

impl Point{
    pub fn new(x:i32, y:i32, z:i32) -> Point{
		return Point{x:x, y:y, z:z}
    }
	pub fn rotate(&mut self, rotmx:Rotation){
		self.x = (self.x as f32 * rotmx.matrix[0][0] * rotmx.matrix[1][0] * rotmx.matrix[2][0]) as i32
	}
	pub fn spher(self) -> [f32; 3]{
		let r = ((self.x as f64).powf(2.0) + (self.y as f64).powf(2.0) + (self.z as f64).powf(2.0)).sqrt();
		let theta = (self.z as f64/r).acos();
		let azim = (self.y as f64/self.x as f64).atan();

		return [r as f32, theta as f32, azim as f32]
	}
	pub fn sher_to_point(spher:[f32; 3]) -> Point{
		let r = spher[0];
		let theta = spher[1];
		let azim = spher[2];
		let x = r * theta.sin()*azim.cos();
		let y = r * theta.sin()*azim.sin();
		let z = r * theta.cos();

		return Point::new(x as i32, y as i32, z as i32);
	}
	pub fn get_vals(self) -> [i32; 3]{
		return [self.x, self.y, self.z]
	}
}



pub fn distance(point1:Point, point2:Point) -> f64{
	let x = (point1.x as f64 - point2.x as f64).powf(2.0);
	let y = (point1.y as f64 - point2.y as f64).powf(2.0);
	let z = (point1.z as f64 - point2.z as f64).powf(2.0);
	return (x+y+z).sqrt();
}

struct Transformation{
	matrix:[[i32;4];4]
}

struct Rotation{
	matrix:[[f32;3];3]
}

impl Rotation{
	pub fn new(xr:f32, yr:f32, zr:f32) -> Rotation{
		let row1 = [yr.cos()*zr.cos(), yr.cos()*zr.sin(), 0.0-yr.sin()];
		let row2 = [0.0-zr.sin(), xr.sin()*zr.sin(), xr.sin()];
		let row3 = [yr.sin(), 0.0-xr.sin(), xr.cos()*yr.cos()];
		return Rotation{matrix:[row1, row2, row3]};
	}
}
