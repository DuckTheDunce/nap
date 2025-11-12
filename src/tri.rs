use crate::point::Point;

pub struct Triangle{
	point1:Point,
	point2:Point,
	point3:Point,
}

impl Triangle{
    pub fn new(point1:Point, point2:Point, point3:Point) -> Triangle{
		return Triangle{point1:point1, point2:point2, point3:point3}
    }
	pub fn get_points(self) -> [Point;3]{
		return [self.point1, self.point2, self.point3]
	}
}
