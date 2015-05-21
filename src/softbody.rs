extern crate glutin;

extern crate glium;

extern crate std;

extern crate nalgebra as na;

use vec_tools::*;

use na::*;
use sphere::*;
use std::collections::HashMap;

struct ConnectionData {
	lhs: usize,
	rhs: usize,
	starting_distance: f32,
}




pub struct SoftBody {
	points: Vec<Sphere>,
	connections: Vec<ConnectionData>,
}

impl SoftBody {
	pub fn new(position: Vec3<f32>, radius: f32) -> SoftBody {
		let mut points = Vec::new();
		let mut connections = Vec::new();

		let bounds = 21 * 21;
		let dist = radius / 10.0;

		for x in (-10 .. 11) {
			for y in (-10 .. 11) {
				let sphere_index = points.len();
				let mut new_sphere = Sphere::new(0.01, 0.01);
				new_sphere.position.x = x as f32 * radius / 10.0;
				new_sphere.position.y = y as f32 * radius / 10.0;
				points.push(new_sphere);

				let c_x = x + 1;
				let c_y = y + 1;

				let c_index_x = sphere_index + 21;
				let c_index_y = sphere_index + 1;

				if c_index_x < bounds {
					connections.push(ConnectionData {
						lhs: sphere_index,
						rhs: c_index_x,
						starting_distance: dist,
					});
				}

				if c_index_y < bounds {
					connections.push(ConnectionData {
						lhs: sphere_index,
						rhs: c_index_y,
						starting_distance: dist,
					});
				}
			}
		}
		SoftBody {
			points: points,
			connections: connections,
		}
	}

	pub fn update(& mut self) {

		for conn in self.connections.iter() {
			let (lhs, rhs) = self.points.get_pair_mut(conn.lhs, conn.rhs);
			apply_spring_force(lhs, rhs, conn.starting_distance);
		}

		for sphere in self.points.iter_mut() {
			sphere.update();	
		}
	}

	pub fn get_points(&self) -> &Vec<Sphere> {
		&self.points
	}
}
fn apply_spring_force(lhs: &mut Sphere, rhs: &mut Sphere, distance: f32){
	let mut force = rhs.position - lhs.position;
	let curr_distance = force.norm();
	force.normalize();
	let modifier = 1.0f32 * curr_distance/distance;
	lhs.position = lhs.position + force * modifier;
	rhs.position = rhs.position - force * modifier;

}
	


