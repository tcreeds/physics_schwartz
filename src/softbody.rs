extern crate glutin;

extern crate glium;

extern crate nalgebra as na;

use na::*;
use sphere::*;

#[derive(Clone, Copy)]
pub struct SoftBody {
	points: Vec<Sphere>,
	connections: Vec<Vec<i32>>,
}

impl SoftBody {
	pub fn new(position: Vec3<f32>, radius: f32) -> SoftBody {
		let points = Vec::new();
		points.push(Sphere::new(0.1f32, 0.1f32));
		let displacement = Math.sqrt(2.0f32) * radius;
		
		let sphere1 = Sphere::new(0.1f32, 0.1f32);
		sphere1.position = Vec3::new(position.x + displacement, position.y + displacement, position.z + displacement);
		points.push(sphere1);
		let sphere2 = Sphere::new(0.1f32, 0.1f32);
		sphere2.position = Vec3::new(position.x - displacement, position.y + displacement, position.z + displacement);
		points.push(sphere2);
		let sphere1 = Sphere::new(0.1f32, 0.1f32);
		sphere1.position = Vec3::new(position.x + displacement, position.y - displacement, position.z + displacement);
		points.push(sphere1);
		let sphere4 = Sphere::new(0.1f32, 0.1f32);
		sphere4.position = Vec3::new(position.x - displacement, position.y - displacement, position.z + displacement);
		points.push(sphere4);

		let sphere5 = Sphere::new(0.1f32, 0.1f32);
		sphere5.position = Vec3::new(position.x + displacement, position.y + displacement, position.z - displacement);
		points.push(sphere5);
		let sphere6 = Sphere::new(0.1f32, 0.1f32);
		sphere6.position = Vec3::new(position.x - displacement, position.y + displacement, position.z - displacement);
		points.push(sphere6);
		let sphere7 = Sphere::new(0.1f32, 0.1f32);
		sphere7.position = Vec3::new(position.x + displacement, position.y - displacement, position.z - displacement);
		points.push(sphere7);
		let sphere8 = Sphere::new(0.1f32, 0.1f32);
		sphere8.position = Vec3::new(position.x - displacement, position.y - displacement, position.z - displacement);
		points.push(sphere8);

		let sphere9 = Sphere::new(0.1f32, 0.1f32);
		sphere9.position = Vec3::new(position.x + radius, position.y, position.z);
		points.push(sphere9);
		let sphere10 = Sphere::new(0.1f32, 0.1f32);
		sphere10.position = Vec3::new(position.x - radius, position.y, position.z);
		points.push(sphere10);
		let sphere11 = Sphere::new(0.1f32, 0.1f32);
		sphere11.position = Vec3::new(position.x, position.y + radius, position.z);
		points.push(sphere11);
		let sphere12 = Sphere::new(0.1f32, 0.1f32);
		sphere12.position = Vec3::new(position.x, position.y - radius, position.z);
		points.push(sphere12);
		let sphere13 = Sphere::new(0.1f32, 0.1f32);
		sphere13.position = Vec3::new(position.x, position.y, position.z + radius);
		points.push(sphere13);
		let sphere14 = Sphere::new(0.1f32, 0.1f32);
		sphere14.position = Vec3::new(position.x, position.y, position.z - radius);
		points.push(sphere14);

		let connections = Vec::new();
		for i in 0..14 {
			connections.push(Vec::new());
			for j in 0..14 {
				if (points[i].position - points[j].position).norm() <= radius * 2 {
					connections[i].push(j);
				}
			}
		}

		SoftBody {
			points: points,
			connections: connections,
		}

	}
	


}