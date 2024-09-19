use std::f32::consts::PI;

use bevy::{prelude::*, render::{mesh::{self, PrimitiveTopology}, render_asset::RenderAssetUsages}};

use crate::vector_extensions::*;

#[derive(Clone, Copy)]
pub enum CornerStyle {
    Sharp,
    Rounded {
        radius: f32,
        /// Number of vertices in a circle. A number proportional to the arc is used
        resolution: usize,
    },
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Center,
    LeftSide,
    RightSide,
}

impl Alignment {
    fn left_width(&self, width: f32) -> f32 {
        match self {
            Alignment::Center => width / 2.,
            Alignment::LeftSide => width,
            Alignment::RightSide => 0.,
        }
    }

    fn right_width(&self, width: f32) -> f32 {
        match self {
            Alignment::Center => width / 2.,
            Alignment::LeftSide => 0.,
            Alignment::RightSide => width,
        }
    }
}

#[derive(Clone, Component)]
pub struct FlexPath {
    /// If first and last points are the same, the path is closed
    locations: Vec<Vec2>,
    left_width: f32,
    right_width: f32,
    corner_style: CornerStyle,
    alignment: Alignment,
    is_connected: bool,
}

impl Default for FlexPath {
    fn default() -> Self {
        FlexPath {
            locations: Vec::new(),
            left_width: 0.5,
            right_width: 0.5,
            corner_style: CornerStyle::Sharp,
            alignment: Alignment::Center,
            is_connected: false,
        }
    }
}

impl FlexPath {
    pub fn new(
        locations: Vec<Vec2>, 
        width: f32, 
        alignment: Alignment, 
        corner_style: CornerStyle,
        connected: bool
    ) -> Self {
        assert!(locations.len() >= 2, "FlexPath must have at least 2 locations");
        FlexPath {
            locations,
            left_width: alignment.left_width(width),
            right_width: alignment.right_width(width),
            corner_style,
            alignment,
            is_connected: connected,
        }
    }

    fn get_next_idx(&self, idx: usize) -> Option<usize> {
        if self.is_connected || idx < self.locations.len() - 1 {
            Some((idx + 1) % self.locations.len())
        } else {
            None
        }
    }

    fn get_prev_idx(&self, idx: usize) -> Option<usize> {
        if self.is_connected || idx > 0 {
            Some((idx as isize + self.locations.len() as isize - 1) as usize % self.locations.len())
        } else {
            None
        }
    }

    pub(crate) fn make_mesh(&self) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for i in 0..self.locations.len() {
            self.calc_segment(i, &mut vertices, &mut indices);
        }
        
        if !self.is_connected {
            self.calc_caps(&mut vertices, &mut indices);
        }
        
        println!("{:?}", vertices);
        println!("{:?}", indices);
        
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vertices,
            ).with_inserted_indices(
                mesh::Indices::U32(indices)
            )
    }

    fn calc_segment(&self, 
        idx: usize, 
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>
    ) {
        let location = self.locations[idx];
        let Some(prev_idx) = self.get_prev_idx(idx) else {
            // First segment
            let next = self.locations[idx + 1];
            let vert1 = calc_left_side_segment(location, next, self.left_width).0;
            let vert2 = calc_right_side_segment(location, next, self.right_width).0;
            vertices.push([vert1.x, vert1.y, 0.]);
            vertices.push([vert2.x, vert2.y, 0.]);
            if self.is_connected {
                self.add_segment_indices(indices, idx);
            }
            return;
        };

        let Some(next_idx) = self.get_next_idx(idx) else {
            // Last segment
            let prev = self.locations[idx - 1];
            let vert1 = calc_left_side_segment(prev, location, self.left_width).1;
            let vert2 = calc_right_side_segment(prev, location, self.right_width).1;
            vertices.push([vert1.x, vert1.y, 0.]);
            vertices.push([vert2.x, vert2.y, 0.]);
            self.add_segment_indices(indices, idx);
            return;
        };

        let prev = self.locations[prev_idx];
        let next = self.locations[next_idx];

        let left_side_a = calc_left_side_segment(prev, location, self.left_width);
        let left_side_b = calc_left_side_segment(location, next, self.left_width);
        
        let right_side_a = calc_right_side_segment(prev, location, self.right_width);
        let right_side_b = calc_right_side_segment(location, next, self.right_width);
        
        let (left_vert, right_vert) = (intersection_point(
            left_side_a.0, left_side_a.1 - left_side_a.0,
            left_side_b.1, left_side_b.0 - left_side_b.1
        ), intersection_point(
            right_side_a.0, right_side_a.1 - right_side_a.0,
            right_side_b.1, right_side_b.0 - right_side_b.1
        ));

        if let (Some(left_vert), Some(right_vert)) = (left_vert, right_vert) {
            vertices.push([left_vert.x, left_vert.y, 0.]);
            vertices.push([right_vert.x, right_vert.y, 0.]);

            self.add_segment_indices(indices, idx);
        }
    }

    fn add_segment_indices(&self,
        indices: &mut Vec<u32>,
        index: usize
    ) {
        let start = index * 2;
        let prev = self.get_prev_idx(index).unwrap() as u32 * 2;

        let a = prev;
        let b = a + 1;

        let c = start as u32;
        let d = c + 1;

        indices.push(a); // a
        indices.push(b); // b
        indices.push(c); // c

        indices.push(b); // b
        indices.push(d); // d
        indices.push(c); // c
    }

    /// Calculate cap for unconnected path.
    fn calc_caps(&self, vertices: &mut Vec<[f32; 3]>, indices: &mut Vec<u32>) {
        // Start cap
        let CornerStyle::Rounded { resolution, .. } = self.corner_style else {
            return;
        };

        let start = self.locations[0];
        let next = self.locations[1];
        let vec = next - start;
        
        let fan_vec = vec.perp().normalize();

        let rotation_angle = vec.y.atan2(vec.x);

        let angle_increment = PI / (resolution / 2 - 1) as f32;
        
        for i in 1..resolution / 2 {
            let angle = i as f32 * angle_increment + rotation_angle;
            println!("{}", angle.to_degrees());
            let rotation_vec = Vec2::new(angle.cos(), angle.sin()).normalize();
            let fan_vec = rotation_vec.rotate(fan_vec);

            let vert = start + fan_vec * self.left_width;

            println!("{:?}", vert);
        }

        /*let num_points = 10; // Number of points
        let half_circle = std::f32::consts::PI; // Half-circle in radians (π)
        let angle_increment = half_circle / (num_points - 1) as f32;

        // Vector at the end of which the half-circle points will be placed
        let end_vector = Vec2::new(2.0, 1.0); // Example vector
        let rotation_angle = end_vector.y.atan2(end_vector.x); // Angle of the vector

        // Rotation vector for the computed angle
        let rotation_vec = Vec2::new(rotation_angle.cos(), rotation_angle.sin());

        // Calculate and rotate each point
        for i in 0..num_points {
            let angle = i as f32 * angle_increment;
            let point_on_half_circle = Vec2::new(angle.cos(), angle.sin());

            // Rotate the half-circle point by the rotation vector's angle
            let rotated_point = rotation_vec.rotate(point_on_half_circle);

            println!("Rotated Point {}: {:?}", i + 1, rotated_point);
        }*/
    }
}

#[test]
fn rungoddammit() {
    let path = FlexPath::new(
        vec![
            Vec2::new(0., 0.),
            Vec2::new(0., 1.),
            Vec2::new(1., 1.),
        ],
        0.5,
        Alignment::Center,
        CornerStyle::Rounded { radius: 0., resolution: 20 },
        false,
    );

    let mesh = path.make_mesh();
}