extern crate glutin;

extern crate glium;

extern crate nalgebra as na;

use na::*;
use sphere::*;

#[derive(Clone, Copy)]
pub struct Plane {
	point: Vec3<f32>,
	normal: Vec3<f32>,
	restitution: f32,
}

impl Plane {
	pub fn new(point: Vec3<f32>, normal: Vec3<f32>, restitution: f32) -> Plane {
		Plane {
			point: point,
			normal: normal.normalize(),
			restitution: restitution,
		}
	}

	pub fn check_collision(&self, sphere: &mut Sphere) -> bool {

		//let velocity_normal = sphere.velocity.clone().normalize();

		//println!("sphere's position{:?}\n", sphere.position);

		let velocity_direction = na::dot(&sphere.velocity, &self.normal);
		if velocity_direction > 0.0f32 {
			return false;
		}

		//dot product of previous frame - for tunneling purposes
		let prev_dot_product = na::dot(&(sphere.position - sphere.velocity - self.point), &self.normal);
		//current dot product
		let dot_product: f32 = na::dot(&(sphere.position - self.point), &self.normal);

		//vector from plane to center of sphere
		let projection = self.normal * dot_product;

		let distance: f32 = if projection.norm() < 0.0f32 {
			projection.norm() * -1.0f32
		} else {
			projection.norm()
		};

		//check if sphere is currently intersecting the line
		if velocity_direction <= 0.0f32 && distance < sphere.radius {
			
			//sphere is still on the positive side of the plane
			if dot_product > 0.0f32 {
				sphere.position = sphere.position + self.normal * (sphere.radius - distance);
				return true;
			} else{
				sphere.position = sphere.position - projection + self.normal * sphere.radius;
				return true;
			}

		}
		//check if sphere has just tunneled through plane
		else if velocity_direction < 0.0f32 && dot_product < 0.0f32 && prev_dot_product > 0.0f32 {
			sphere.position = sphere.position - projection + self.normal * sphere.radius;
			return true;
		}
		false
	}

	pub fn bounce_sphere(&self, sphere: &mut Sphere){
		sphere.velocity = self.reflect(sphere.velocity) * self.restitution; 
	}

	pub fn reflect(&self, vector: Vec3<f32>) -> Vec3<f32> {
		vector - self.normal * 2.0f32 * na::dot(&vector, &self.normal) 
	}
}