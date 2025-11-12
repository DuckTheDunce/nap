use crate::point::Point;
use crate::point;
use crate::tri::Triangle;

struct Pixel{
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

pub struct PixelLayer{
	pixels:Vec<Pixel>,
	width:u16
}

pub struct Camera{
	pos:Point,
	focal_point:Point
}

impl Camera{
    pub fn new(pos:Point, focal_point:Point) -> Camera{
		return Camera{pos:pos, focal_point:focal_point}
    }
	pub fn render(self, triangles:Vec<Triangle>, width:u32, height:u32) -> PixelLayer{
    	//find rotation of camera via focal point

		let mut flat_points:Vec<[f32; 3]> = Vec::new();
    	
    	for triangle in triangles{
			for point in triangle.get_points(){
				let mut spher:[f32; 3] = point.spher();
				spher[0] = 1.0;
				flat_points.push(spher);
				//flat_points.push(point::sher_to_point(spher));
			}
    	}
    	//get theta and azim range of camera
    	let focal_dist =  point::distance(self.pos, self.focal_point);
    	let offsets = self.focal_point.spher();
    	let theta_range = (0.5/focal_dist as f32).atan();
    	let azim_range = ((width as f32/(height as f32*2.0))/focal_dist as f32).atan();
    	

    	let mut pix:Vec<Pixel> = Vec::new();
    	for i in 0..(width*height){
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
			println!("x{} y{}", x_pos, y_pos);
			pix[(x_pos + y_pos*width) as usize] = Pixel::new(255, 0, 0, 255);
    	}

    	
		return PixelLayer{pixels:pix, width:width as u16}
	}
}
