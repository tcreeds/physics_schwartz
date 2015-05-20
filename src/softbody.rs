extern crate glutin;

extern crate glium;

extern crate nalgebra as na;

use na::*;
use sphere::*;

struct ConnectionData {
	connections: [i32, ..14],
	distance: [f32, ..14],
}

#[derive(Clone)]
pub struct SoftBody {
	pub points: Vec<Sphere>,
	connections: HashMap<&Sphere, Vec<usize>>,
	distances: HashMap<&Sphere, [f32, ..14]>,
}

impl SoftBody {
	pub fn new(position: Vec3<f32>, radius: f32) -> SoftBody {
		let mut points = Vec::new();
		points.push(Sphere::new(0.1f32, 0.1f32));
		let displacement = 2.0f32.sqrt() * radius;
		
		let mut sphere1 = Sphere::new(0.1f32, 0.1f32);
		sphere1.position = Vec3::new(position.x + displacement, position.y + displacement, position.z + displacement);
		points.push(sphere1);
		let mut sphere2 = Sphere::new(0.1f32, 0.1f32);
		sphere2.position = Vec3::new(position.x - displacement, position.y + displacement, position.z + displacement);
		points.push(sphere2);
		let mut sphere1 = Sphere::new(0.1f32, 0.1f32);
		sphere1.position = Vec3::new(position.x + displacement, position.y - displacement, position.z + displacement);
		points.push(sphere1);
		let mut sphere4 = Sphere::new(0.1f32, 0.1f32);
		sphere4.position = Vec3::new(position.x - displacement, position.y - displacement, position.z + displacement);
		points.push(sphere4);

		let mut sphere5 = Sphere::new(0.1f32, 0.1f32);
		sphere5.position = Vec3::new(position.x + displacement, position.y + displacement, position.z - displacement);
		points.push(sphere5);
		let mut sphere6 = Sphere::new(0.1f32, 0.1f32);
		sphere6.position = Vec3::new(position.x - displacement, position.y + displacement, position.z - displacement);
		points.push(sphere6);
		let mut sphere7 = Sphere::new(0.1f32, 0.1f32);
		sphere7.position = Vec3::new(position.x + displacement, position.y - displacement, position.z - displacement);
		points.push(sphere7);
		let mut sphere8 = Sphere::new(0.1f32, 0.1f32);
		sphere8.position = Vec3::new(position.x - displacement, position.y - displacement, position.z - displacement);
		points.push(sphere8);

		let mut sphere9 = Sphere::new(0.1f32, 0.1f32);
		sphere9.position = Vec3::new(position.x + radius, position.y, position.z);
		points.push(sphere9);
		let mut sphere10 = Sphere::new(0.1f32, 0.1f32);
		sphere10.position = Vec3::new(position.x - radius, position.y, position.z);
		points.push(sphere10);
		let mut sphere11 = Sphere::new(0.1f32, 0.1f32);
		sphere11.position = Vec3::new(position.x, position.y + radius, position.z);
		points.push(sphere11);
		let mut sphere12 = Sphere::new(0.1f32, 0.1f32);
		sphere12.position = Vec3::new(position.x, position.y - radius, position.z);
		points.push(sphere12);
		let mut sphere13 = Sphere::new(0.1f32, 0.1f32);
		sphere13.position = Vec3::new(position.x, position.y, position.z + radius);
		points.push(sphere13);
		let mut sphere14 = Sphere::new(0.1f32, 0.1f32);
		sphere14.position = Vec3::new(position.x, position.y, position.z - radius);
		points.push(sphere14);

		let mut connections = HashMap::new();
		for i in 0..14 {
			let mut conn = Vec::new();
			let mut data = [f32, ..14];
			for j in 0..14 {
				let dist = (points[i].position - points[j].position).norm();
				if dist <= radius * 2.0f32 {
					conn.push(j);
					data[j] = dist;
				} else {
					data[j] = -1.0f32;
				}
			}
			connections.insert(points[i], conn);
			distances.insert(points[i], data);
		}

		SoftBody {
			points: points,
			connections: connections,
			distances: distances,
		}

	}

	pub fn update(&self){
		let mut closed = vec![];
		let mut open = vec![self.points[0]];

		while open.len() > 0i32{
			let mut curr = open.pop();
			for point in self.connections[curr].iter() {
				if !closed.contains(&self.points[point]) {
					self.apply_spring_force(&curr, &self.points[point], self.distances(&curr)[point]);
					open.push(self.points[point]);
				}
			}
			closed.push(curr);
		}
	}

	fn apply_spring_force(&self, lhs: &mut Sphere, rhs: &mut Sphere, distance: f32){
		let mut force = rhs.position - lhs.position;
		let curr_distance = force.norm();
		force.normalize();
		let modifier = 1.0f32 * curr_distance/distance;
		lhs.position += modifier * force;
		rhs.position -= modifier * force;

	}
	


}