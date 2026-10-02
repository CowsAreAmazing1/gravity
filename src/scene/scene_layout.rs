use nannou::prelude::*;

use crate::{
    scene::shapes::{disc::Disc, quad::Quad},
    sim::system::{Body, Dust},
};

// Main trait implemented by all scene builder elements. Allows filling with Dust particles.
pub trait FillWithDust {
    fn build(&self, num: u32, target_vec: &mut Vec<Dust>);
    fn build_random(&self, num: u32, target_vec: &mut Vec<Dust>);
}

// ---------- SetupElement enum and Setup builder (static dispatch) ----------
/// Closed set of setup elements — static, enumerable, and fast to match on.
#[derive(Debug)]
pub enum SetupElement {
    Disc(Disc),
    Quad(Quad),
}

impl From<Disc> for SetupElement {
    fn from(d: Disc) -> Self {
        SetupElement::Disc(d)
    }
}

impl From<Quad> for SetupElement {
    fn from(q: Quad) -> Self {
        SetupElement::Quad(q)
    }
}

impl FillWithDust for SetupElement {
    fn build(&self, num: u32, target_vec: &mut Vec<Dust>) {
        // println!("Building {:?} with {} particles", self, num);
        match self {
            SetupElement::Disc(d) => d.build(num, target_vec),
            SetupElement::Quad(q) => q.build(num, target_vec),
        }
    }

    fn build_random(&self, num: u32, target_vec: &mut Vec<Dust>) {
        match self {
            SetupElement::Disc(d) => d.build_random(num, target_vec),
            SetupElement::Quad(q) => q.build_random(num, target_vec),
        }
    }
}

/// Scene setup: holds a list of `SetupElement`s and can populate dust particles.
pub struct Setup {
    elements: Vec<SetupElement>,
}

impl Setup {
    pub fn new() -> Self {
        Setup {
            elements: Vec::new(),
        }
    }

    /// Accept anything that converts into a `SetupElement` (Disc, Quad, ...)
    pub fn add<E: Into<SetupElement>>(&mut self, element: E) -> &mut Self {
        self.elements.push(element.into());
        self
    }

    pub fn build(&self, total_num_particles: u32, target: &mut Vec<Dust>) {
        if self.elements.is_empty() {
            return;
        }
        let num = total_num_particles / self.elements.len() as u32;
        println!(
            "Building setup with {} elements, {} particles each for {} in total",
            self.elements.len(),
            num,
            self.elements.len() * num as usize,
        );
        for element in &self.elements {
            element.build(num, target);
        }
    }

    pub fn build_random(&self, total_num_particles: u32, target: &mut Vec<Dust>) {
        if self.elements.is_empty() {
            return;
        }
        let num = total_num_particles / self.elements.len() as u32;
        for element in &self.elements {
            element.build_random(num, target);
        }
    }
}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SetupObject {
    /// Return an updated builder with the new operation.
    fn add_operation(self, op: CreationOperation) -> Self
    where
        Self: Sized;

    fn center_position(self, center: Vec2) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::CenterOffset(center))
    }
    fn center_position_xy(self, x: f32, y: f32) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::CenterOffset(vec2(x, y)))
    }
    fn center_velocity(self, velocity: Vec2) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::VelocityOffset(velocity))
    }
    fn center_velocity_xy(self, x: f32, y: f32) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::VelocityOffset(vec2(x, y)))
    }
    fn speed_scale(self, scale: f32) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::VelocityScale(scale))
    }

    fn orbit(self, center: Vec2, mass: f32, clockwise: bool) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::Orbit(center, mass, clockwise))
    }
    fn orbit_attractor<T: Body>(self, body: &T, clockwise: bool) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::Orbit(
            body.position(),
            body.mass(),
            clockwise,
        ))
    }
    fn rotate(self, angle: f32) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::RotateAround(Vec2::ZERO, angle))
    }
    fn rotate_around(self, center: Vec2, angle: f32) -> Self
    where
        Self: Sized,
    {
        self.add_operation(CreationOperation::RotateAround(center, angle))
    }
}

#[derive(Debug)]
pub enum CreationOperation {
    CenterOffset(Vec2),
    VelocityOffset(Vec2),
    // RadialVelocity(f32),
    VelocityScale(f32),
    Orbit(Vec2, f32, bool),  // (orbit center, mass, clockwise?)
    RotateAround(Vec2, f32), // (center, angle)
}

// pub fn compute_l1_point(_g: f64, M: f64, m: f64, R: f64) -> f64 {
//     // let omega2 = g * (M + m) / (R * R * R);

//     let f = |x: f64| -> f64 {
//         if x <= 0.0 || x >= R {
//             return f64::INFINITY;
//         }
//         -M / (x * x) + m / ((R - x).powi(2)) + M * m / (x * x)
//     };

//     // Try to find a valid bracket where f(x) crosses zero
//     let mut a = R * 0.001;
//     let mut b = R * 0.999;

//     let mut fa = f(a);
//     let mut fb = f(b);

//     // If initial guess doesn't cross zero, scan for a bracket
//     if fa * fb > 0.0 {
//         for i in 1..1000 {
//             let t = i as f64 / 1000.0;
//             let x1 = t * R;
//             let x2 = (t + 0.001) * R;
//             let f1 = f(x1);
//             let f2 = f(x2);
//             if f1.is_finite() && f2.is_finite() && f1 * f2 < 0.0 {
//                 a = x1;
//                 b = x2;
//                 fa = f1;
//                 fb = f2;
//                 break;
//             }
//         }
//         if fa * fb > 0.0 {
//             panic!("Could not find a valid bracket for root finding.");
//         }
//     }

//     // Brent-style bisection
//     for _ in 0..100 {
//         let m = 0.5 * (a + b);
//         let fm = f(m);

//         if fm.abs() < 1e-10 || (b - a).abs() < 1e-8 {
//             return m;
//         }

//         if fa * fm < 0.0 {
//             b = m;
//             fb = fm;
//         } else {
//             a = m;
//             fa = fm;
//         }
//     }

//     panic!("Root finding did not converge.");
// }
