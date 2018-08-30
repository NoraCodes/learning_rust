extern crate rayon;
extern crate itertools;
use rayon::prelude::*;
use itertools::Itertools;

mod bodies;
use bodies::get_values;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

impl Body {
    fn new(x: f64, y: f64, z: f64, mass: f64) -> Body {
        Body {
            x: x, 
            y: y,
            z: z,
            mass: mass
        }
    }
}

fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn average_with_mass(a: f64, b: f64, amass: f64, bmass: f64) -> f64 {
    average(a * amass, b * bmass) / (amass + bmass)
}

fn merge_two_bodies(a: Body, b: Body) -> Body {
    Body {
        x: average_with_mass(a.x, b.x, a.mass, b.mass),
        y: average_with_mass(a.y, b.y, a.mass, b.mass),
        z: average_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass
    }
}

fn merge_all_bodies_recursive(bodies: &[Body]) -> Body {
    println!("Bodies: {}", bodies.len());

    if bodies.len() == 1 {
        return bodies[0];
    }
    
    let tuples: Vec<_> = bodies.iter().tuples().collect();
    let mut merged_bodies: Vec<_> = tuples.into_par_iter().map(|(a, b)| merge_two_bodies(*a,*b)).collect();

    if bodies.len() % 2 != 0 {
        merged_bodies.push(bodies[bodies.len() - 1]);
    }
    return merge_all_bodies_recursive(&merged_bodies); 
}

fn main() {
    let bodies = get_values();
    let barycenter = merge_all_bodies_recursive(&bodies);
    println!("Barycenter: ({}, {}, {})\nMass: {}", barycenter.x, barycenter.y, barycenter.z, barycenter.mass);
}
