extern crate glutin;

extern crate glium;

extern crate std;

extern crate nalgebra as na;

use na::*;
use sphere::*;
use std::collections::HashMap;

struct ConnectionData {
	distance: Vec<f32>,
}

#[derive(Clone)]
pub struct SoftBody {
	pub points: Vec<Sphere>,
	connections: HashMap<Sphere, Vec<i32>>,
	distances: HashMap<Sphere, Vec<f32>>,
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
		let mut distances = HashMap::new();
		for i in 0..14 {
			let mut conn = Vec::new();
			let mut data = Vec::new();
			for j in 0..14 {
				let dist = (points[i].position - points[j].position).norm();
				if dist <= radius * 2.0f32 {
					conn.push(j as i32);
					data.push(dist);
				} else {
					data.push(-1.0f32);
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

		while !open.is_empty() {
			let mut curr = open.pop().unwrap();
			for point in self.connections.get(&curr).unwrap().iter() {
				if !closed.contains(self.points[point]) {
					self.apply_spring_force(&mut curr, &mut self.points[*point], self.distances.get(&curr).unwrap()[*point]);
					open.push(self.points[*point]);
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
		lhs.position = lhs.position + force * modifier;
		rhs.position = rhs.position - force * modifier;

	}
	


}