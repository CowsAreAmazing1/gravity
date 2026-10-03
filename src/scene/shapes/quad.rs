use nannou::{
    geom::{Range, Rect},
    glam::{DVec2, Vec2, dvec2, vec2},
    rand::random_range,
};
use serde::{Deserialize, Serialize};

use crate::{
    scene::scene_layout::{CreationOperation, FillWithDust, SetupObject},
    sim::system::Dust,
};

#[derive(Debug)]
pub struct Quad {
    rect: Rect,
    ops: Vec<CreationOperation>,
}

impl SetupObject for Quad {
    fn add_operation(mut self, op: CreationOperation) -> Self {
        self.ops.push(op);
        self
    }
}

impl Quad {
    pub fn new() -> Self {
        Quad {
            rect: Rect::from_wh(Vec2::ONE),
            ops: vec![],
        }
    }

    pub fn square(mut self, size: f32) -> Self {
        self.rect = Rect::from_w_h(size, size);
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.rect.x = Range::new(-0.5 * width, 0.5 * width);
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.rect.y = Range::new(-0.5 * height, 0.5 * height);
        self
    }
}

impl Default for Quad {
    fn default() -> Self {
        Self::new()
    }
}

impl FillWithDust for Quad {
    // fn build(&self, num: u32, target: &mut Vec<Dust>) {
    //     let Self { rect, ops } = self;
    //     let (w, h) = rect.w_h();

    //     // Initial random distribution
    //     let dx = 1.0 / num as f64;
    //     let inv_phi = 2.0 / (1.0 + 5_f64.sqrt());
    //     let points = (0..num / 2)
    //         .map(|n| {
    //             // voronator::delaunator::Point {
    //             //     x: random_range(-0.5*w, 0.5*w) as f64,
    //             //     y: random_range(-0.5*h, 0.5*h) as f64,
    //             // }
    //             voronator::delaunator::Point {
    //                 x: w as f64 * (n as f64 * dx - 0.5),
    //                 y: h as f64 * ((n as f64 * inv_phi).fract() - 0.5),
    //             }
    //         })
    //         .collect::<Vec<voronator::delaunator::Point>>();

    //     // Single pass of Lloyd's algorithm to improve distribution
    //     let points = voronator::CentroidDiagram::<voronator::delaunator::Point>::new(&points)
    //         .unwrap()
    //         .centers;

    //     for point in points {
    //         let mut pos = vec2(point.x as f32, point.y as f32);
    //         let mut vel = Vec2::ZERO;

    //         for op in ops {
    //             match op {
    //                 CreationOperation::CenterOffset(v) => {
    //                     pos += *v;
    //                 }
    //                 CreationOperation::VelocityOffset(v) => {
    //                     vel += *v;
    //                 }
    //                 CreationOperation::VelocityScale(s) => {
    //                     vel *= *s;
    //                 }
    //                 CreationOperation::Orbit(center, mass, clockwise) => {
    //                     let rel_pos = *center - pos;
    //                     let tangent = rel_pos.perp().normalize();
    //                     let dist = rel_pos.length();
    //                     let speed = (mass / dist).sqrt();
    //                     let sign = if *clockwise { 1.0 } else { -1.0 };
    //                     vel += sign * speed * tangent;
    //                 }
    //             }
    //         }
    //         target.push(Dust::new(pos, vel));
    //     }
    // }

    fn build(&self, num: u32, target: &mut Vec<Dust>) {
        let Self { rect, ops } = self;
        let (w, h) = rect.w_h();

        // Initial random distribution
        let dx = 1.0 / num as f64;
        let inv_phi = 2.0 / (1.0 + 5_f64.sqrt());
        target.append(
            &mut (0..num)
                .map(|n| {
                    let mut pos = dvec2(
                        w as f64 * (n as f64 * dx - 0.5),
                        h as f64 * ((n as f64 * inv_phi).fract() - 0.5),
                    );
                    let mut vel = DVec2::ZERO;

                    for op in ops {
                        match op {
                            CreationOperation::CenterOffset(v) => {
                                let d_v = dvec2(v.x as f64, v.y as f64);
                                pos += d_v;
                            }
                            CreationOperation::VelocityOffset(v) => {
                                let d_v = dvec2(v.x as f64, v.y as f64);
                                vel += d_v;
                            }
                            CreationOperation::VelocityScale(s) => {
                                vel *= *s as f64;
                            }
                            CreationOperation::Orbit(center, mass, clockwise) => {
                                let d_center = dvec2(center.x as f64, center.y as f64);

                                let rel_pos = d_center - pos;
                                let tangent = rel_pos.perp().normalize();
                                let dist = rel_pos.length();
                                let speed = (*mass as f64 / dist).sqrt();
                                let sign = if *clockwise { 1.0 } else { -1.0 };
                                vel += sign * speed * tangent;
                            }
                            CreationOperation::RotateAround(center, angle) => {
                                let d_center = dvec2(center.x as f64, center.y as f64);

                                let rel_pos = pos - d_center;
                                let cos_angle = angle.cos() as f64;
                                let sin_angle = angle.sin() as f64;
                                pos = d_center
                                    + dvec2(
                                        rel_pos.x * cos_angle - rel_pos.y * sin_angle,
                                        rel_pos.x * sin_angle + rel_pos.y * cos_angle,
                                    );
                            }
                        }
                    }

                    let pos = vec2(pos.x as f32, pos.y as f32);
                    let vel = vec2(vel.x as f32, vel.y as f32);

                    Dust::new(pos, vel)
                })
                .collect::<Vec<Dust>>(),
        );
    }

    fn build_random(&self, num: u32, target_vec: &mut Vec<Dust>) {
        let Self { rect, ops } = self;
        let (w, h) = rect.w_h();

        target_vec.append(
            &mut (0..num)
                .map(|_| {
                    let mut pos = vec2(
                        random_range(-0.5 * w, 0.5 * w),
                        random_range(-0.5 * h, 0.5 * h),
                    );
                    let mut vel = Vec2::ZERO;

                    for op in ops {
                        match op {
                            CreationOperation::CenterOffset(v) => {
                                pos += *v;
                            }
                            CreationOperation::VelocityOffset(v) => {
                                vel += *v;
                            }
                            CreationOperation::VelocityScale(s) => {
                                vel *= *s;
                            }
                            CreationOperation::Orbit(center, mass, clockwise) => {
                                let rel_pos = *center - pos;
                                let tangent = rel_pos.perp().normalize();
                                let dist = rel_pos.length();
                                let speed = (mass / dist).sqrt();
                                let sign = if *clockwise { 1.0 } else { -1.0 };
                                vel += sign * speed * tangent;
                            }
                            CreationOperation::RotateAround(center, angle) => {
                                let rel_pos = pos - *center;
                                let cos_angle = angle.cos();
                                let sin_angle = angle.sin();
                                pos = *center
                                    + vec2(
                                        rel_pos.x * cos_angle - rel_pos.y * sin_angle,
                                        rel_pos.x * sin_angle + rel_pos.y * cos_angle,
                                    );
                            }
                        }
                    }

                    Dust::new(pos, vel)
                })
                .collect::<Vec<Dust>>(),
        );
    }
}

#[derive(Serialize, Deserialize)]
pub enum SelfOperation {
    Square(f32),
    Width(f32),
    Height(f32),
}

#[derive(Serialize, Deserialize)]
pub struct ParseQuad {
    self_ops: Vec<SelfOperation>,
    dust_ops: Vec<CreationOperation>,
}

impl ParseQuad {
    pub fn new(self_ops: Vec<SelfOperation>, dust_ops: Vec<CreationOperation>) -> Self {
        ParseQuad { self_ops, dust_ops }
    }
}

impl From<ParseQuad> for Quad {
    fn from(value: ParseQuad) -> Self {
        let mut quad = Quad::new();

        for op in value.self_ops {
            match op {
                SelfOperation::Square(size) => {
                    quad = quad.square(size);
                }
                SelfOperation::Width(width) => {
                    quad = quad.width(width);
                }
                SelfOperation::Height(height) => {
                    quad = quad.height(height);
                }
            }
        }

        for op in value.dust_ops {
            quad = quad.add_operation(op);
        }

        quad
    }
}
