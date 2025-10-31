struct Transformation{
	values:[[i32;4];4]
}

impl Transformation{
    pub fn new() -> Transformation{
		return Transformation{values:[[0, 0, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0],[0, 0, 0, 0]]}
    }
	pub fn displace(&mut self, x:i32, y:i32, z:i32){
		self.values[0][3] += x;
		self.values[1][3] += y;
		self.values[2][3] += z;
	}
	pub fn scale(&mut self, x:i32, y:i32, z:i32){
		self.values[0][0] += x;
		self.values[1][1] += y;
		self.values[2][2] += z;
	}
}

enum Direction{
	X,
	Y,
	Z
}

struct Rotation{
    values:[[f32;3];3]
}

impl Rotation{
    pub fn new() -> Rotation{
		return Rotation{values:[[0, 0, 0],[0, 0, 0],[0, 0, 0]]}
    }
	pub fn rotate(&mut self, direction:Direction, theta:f32){
		match direction{
			Direction::X => {
				println!("ok");
			}
			Direction::Y => {
				println!("ok");
			}
			Direction::Z => {
				println!("ok");
			}
		}
	}
}
