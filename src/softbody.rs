extern crate glutin;

extern crate glium;

extern crate std;

extern crate nalgebra as na;

use vec_tools::*;

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

		let half_dim = 2i32;
		let dim = 2 * half_dim + 1;

		let bounds = dim * dim * dim;
		let dist = radius / half_dim as f32;

		for z in (-half_dim .. half_dim + 1) {
			for x in (-half_dim .. half_dim + 1) {
				for y in (-half_dim .. half_dim + 1) {
					let sphere_index = points.len();
					let mut new_sphere = Sphere::new(0.3, 1.0);
					new_sphere.position = position;
					new_sphere.position.x += x as f32 * radius / half_dim as f32;
					new_sphere.position.y += y as f32 * radius / half_dim as f32;
					new_sphere.position.z += z as f32 * radius / half_dim as f32;
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
		//points[0].velocity.x = -0.2f32;
		//points[0].velocity.y = -0.2f32;
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
			sphere.velocity = sphere.velocity * 0.99;
		}
	}

	pub fn get_points(&self) -> &Vec<Sphere> {
		&self.points
	}

	pub fn get_points_mut(& mut self) -> & mut Vec<Sphere> {
		& mut self.points
	}
}
fn apply_spring_force(lhs: &mut Sphere, rhs: &mut Sphere, distance: f32){
	
	let mut force = rhs.position - lhs.position;
	let curr_distance = force.norm();
	force = force.normalize();
	let mut modifier = curr_distance - distance;
	//println!("{:?}", modifier);
	{
		modifier = modifier * 0.1f32;
		lhs.force = lhs.force + force * modifier;
		rhs.force = rhs.force - force * modifier;
	}

}
	


