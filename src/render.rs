use crate::point::Point;
use crate::point;
use crate::tri::Triangle;

#[derive(Debug)]
pub struct Pixel{
	pub r:u8,
	pub g:u8,
	pub b:u8,
	pub a:u8
}

impl Pixel{
	pub fn new(r:u8, g:u8, b:u8, a:u8) -> Pixel{
		return Pixel{r:r, g:g, b:b, a:a}
	}
	
}

pub struct PixelLayer{
	pub pixels:Vec<Pixel>,
	pub width:u16
}

impl PixelLayer{
	/*pub fn to_arr_rgba(self) -> Vec::<u8>{
		let mut buf:Vec<u8> = Vec::new();
		for pixel in self.pixels{
			buf.push(pixel.r);
			buf.push(pixel.g);
			buf.push(pixel.b);
			buf.push(pixel.a);
		}
		return buf;
	}*/
}

#[derive(Clone)]
pub struct Camera{
	pos:Point,
	focal_point:Point,
	focal_scale:f32
}

impl Camera{
    pub fn new(pos:Point, focal_point:Point, focal_scale:f32) -> Camera{
		return Camera{pos:pos, focal_point:focal_point, focal_scale:focal_scale}
    }
	pub fn render(self, triangles:Vec<Triangle>, width:u32, height:u32) -> PixelLayer{
    	//find rotation of camera via focal point

		let mut flat_points:Vec<[f32; 4]> = Vec::new();
    	
    	for triangle in triangles{
			for point in triangle.get_points(){
				let mut spher:[f32; 3] = point.spher();
				let original_dist = spher[0];
				spher[0] = 1.0;
				flat_points.push([spher[0],spher[1],spher[2],original_dist]);
				//flat_points.push(point::sher_to_point(spher));
			}
    	}
    	//get theta and azim range of camera
    	let focal_dist =  point::distance(self.pos, self.focal_point);
    	let offsets = self.focal_point.spher();
    	let theta_range = ((self.focal_scale)/focal_dist as f32).atan();
    	let azim_range = ((width as f32/(height as f32*2.0))*self.focal_scale/focal_dist as f32).atan();
    	

    	let mut pix:Vec<Pixel> = Vec::new();
    	for _ in 0..(width*height){
			pix.push(Pixel::new(0, 0, 0, 255));
    	}
		//let mut 2d_points:Vec<[f32; 2]> = Vec::new();
    	for point in flat_points{
			//get the distance the point is off in the viewport
			let theta = offsets[1] + theta_range;
			let x_pos = ((theta - point[1])/(2.0*theta_range) * width as f32) as u32;

			let azim = offsets[2] + azim_range;
			let y_pos = ((azim - point[2])/(2.0*azim_range) * height as f32) as u32;
			//2d_points.push([x_pos, y_pos]);
			println!("{}", point[3]);
			pix[(x_pos + y_pos*width) as usize] = Pixel::new(255 - point[3] as u8, 255 - point[3] as u8, 255 - point[3] as u8, 255);
    	}

    	
		return PixelLayer{pixels:pix, width:width as u16}
	}
}
