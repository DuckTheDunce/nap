pub struct Pixel{
	r:u8,
	g:u8,
	b:u8,
	a:u8
}

impl Pixel{
	pub fn new(r:u8, g:u8, b:u8, a:u8) -> Pixel{
		return Pixel{r:r, g:g, b:b, a:a}
	}
}

pub struct Mask{
	pixels:Vec<Pixel>
}

impl Mask{
	pub fn new() -> Mask{
		return Mask{pixels:Vec::new()}
	}
	pub fn add(&mut self, pixel:Pixel){
		self.pixels.push(pixel);
	}
}

struct Camera{
	x:i32,
	y:i32,
	z:i32,
	fov:f32,

}
