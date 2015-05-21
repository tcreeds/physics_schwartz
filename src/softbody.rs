extern crate glutin;

extern crate glium;

extern crate std;

extern crate nalgebra as na;

use vec_tools::*;
use vm::*;

use na::*;
use sphere::*;
#[derive(Debug)]
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

		let half_dim = 3i32;
		let dim = 2 * half_dim + 1;

		let bounds = dim * dim * dim;
		let dist = radius / half_dim as f32;

		for z in (-half_dim .. half_dim + 1) {
			for x in (-half_dim .. half_dim + 1) {
				for y in (-half_dim .. half_dim + 1) {
					let sphere_index = points.len();
					let mut new_sphere = Sphere::new(radius / 2.0 / half_dim as f32, 0.2);
					new_sphere.position = position;
					new_sphere.position.x += x as f32 * radius / half_dim as f32;
					new_sphere.position.y += y as f32 * radius / half_dim as f32;
					new_sphere.position.z += z as f32 * radius / half_dim as f32;
					new_sphere.fixed = y == half_dim;
					points.push(new_sphere);

					let c_index_y = sphere_index as i32 + 1;
					let c_index_x = dim * dim * (z + half_dim) + dim * (x + half_dim + 1) + (y + half_dim);
					let c_index_z = dim * dim * (z + 1 + half_dim) + dim * (x + half_dim) + (y + half_dim);

					if c_index_x < bounds && (x + half_dim + 1) % dim != 0 {
						connections.push(ConnectionData {
							lhs: sphere_index as usize,
							rhs: c_index_x as usize,
							starting_distance: dist,
						});
					}

					if c_index_y < bounds && (y + half_dim + 1) % dim != 0 {
						connections.push(ConnectionData {
							lhs: sphere_index as usize,
							rhs: c_index_y as usize,
							starting_distance: dist,
						});
					}

					if c_index_z < bounds {
						connections.push(ConnectionData {
							lhs: sphere_index as usize,
							rhs: c_index_z as usize,
							starting_distance: dist,
						});
					}

				}
			}
		}
		SoftBody {
			points: points,
			connections: connections,
		}
	}

	pub fn update(& mut self, g: f32, k: f32, damp: f32, mac: & VM) {
		for conn in self.connections.iter() {
			let (lhs, rhs) = self.points.get_pair_mut(conn.lhs, conn.rhs);
			apply_spring_force(lhs, rhs, conn.starting_distance, k, damp, mac);
			
		}
		for sphere in self.points.iter_mut() {
			sphere.update();	
			sphere.velocity.y += g;
		}
	}

	pub fn get_points(&self) -> &Vec<Sphere> {
		&self.points
	}

	pub fn get_points_mut(& mut self) -> & mut Vec<Sphere> {
		& mut self.points
	}
}
fn apply_spring_force(lhs: &mut Sphere, rhs: &mut Sphere, distance: f32, k: f32, damp: f32, mac: & VM){
	
	let mut force = rhs.position - lhs.position;
	let curr_distance = force.norm();
	let rel_velocity = rhs.velocity - lhs.velocity;
	force = force.normalize();
	let mut modifier = curr_distance - distance;
	{
		// x v d k
		let data = vec![force.x as f64 * modifier as f64, rel_velocity.x as f64, damp as f64, k as f64];
		force.x = mac.run(&data) as f32;
		let data = vec![force.y as f64 * modifier as f64, rel_velocity.y as f64, damp as f64, k as f64];
		force.y = mac.run(&data) as f32;
		let data = vec![force.z as f64 * modifier as f64, rel_velocity.z as f64, damp as f64, k as f64];
		force.z = mac.run(&data) as f32;


		lhs.force = lhs.force - force;
		rhs.force = rhs.force + force;
	}

}