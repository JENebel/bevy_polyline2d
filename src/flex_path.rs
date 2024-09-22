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
            let left_vert = calc_left_side_segment(location, next, self.left_width).0;
            let right_vert = calc_right_side_segment(location, next, self.right_width).0;
            vertices.push([left_vert.x, left_vert.y, 0.]);
            vertices.push([right_vert.x, right_vert.y, 0.]);
            if self.is_connected {
                self.add_sharp_segment_indices(indices, idx);
            }
            return;
        };

        let Some(next_idx) = self.get_next_idx(idx) else {
            // Last segment
            let prev = self.locations[idx - 1];
            let left_vert = calc_left_side_segment(prev, location, self.left_width).1;
            let right_vert = calc_right_side_segment(prev, location, self.right_width).1;
            vertices.push([left_vert.x, left_vert.y, 0.]);
            vertices.push([right_vert.x, right_vert.y, 0.]);
            let a = (vertices.len() - 4) as u32;
            let b = a + 1;
            let c = a + 2;
            let d = a + 3;
            self.add_quad_indices(indices, a, b, c, d);
            return;
        };

        let location = self.locations[idx];
        let prev = self.locations[prev_idx];
        let next = self.locations[next_idx];

        match self.corner_style {
            CornerStyle::Sharp => self.add_sharp_corner(vertices, indices, idx, prev_idx, next_idx),
            CornerStyle::Rounded { .. } => match orientation_test(prev, location, next) {
                Orientation::Left => todo!(),
                Orientation::Right => self.add_rounded_right_corner(vertices, indices, location, prev, next),
                Orientation::Straight => (), // do nothing
            },
        }
    }

    fn add_rounded_right_corner(&self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        location: Vec2,
        prev: Vec2,
        next: Vec2
    ) {
        let CornerStyle::Rounded { radius, resolution } = self.corner_style else {
            return;
        };

        let right_side_a = calc_right_side_segment(prev, location, self.right_width);
        let right_side_b = calc_right_side_segment(location, next, self.right_width);
        
        let inner_angle =  -(next - location).angle_between(prev - location);
        let corner_angle = 2. * PI - inner_angle;

        let corner_origo = {
            let intersection = intersection_point(
                right_side_a.0, right_side_a.1 - right_side_a.0,
                right_side_b.1, right_side_b.0 - right_side_b.1
            ).unwrap();

            let rotation_vec = Vec2::from_angle(-inner_angle / 2.);
            let towards_origo = rotation_vec.rotate(next - location).normalize();
            intersection + towards_origo * (radius / (corner_angle / 2.).sin())
        };
        let fan_count: i32 = 2.max((resolution as f32 / (2. * PI) * (corner_angle - PI)) as i32);

        let out_vec = {
            let projected = Self::project_point_onto_line(right_side_a.0, right_side_a.1, corner_origo);
            projected - corner_origo
        };

        let angle_step_size = (corner_angle - PI) / fan_count as f32;

        for i in 0..fan_count + 1 {
            let angle = -i as f32 * angle_step_size;
            let rotation_vec = Vec2::from_angle(angle);
            let dir_vec = rotation_vec.rotate(out_vec).normalize();
            let outer_vert = corner_origo + dir_vec * (radius + self.total_width()); // left
            let inner_vert = corner_origo + dir_vec * radius; // right
            vertices.push([outer_vert.x, outer_vert.y, 0.]);
            vertices.push([inner_vert.x, inner_vert.y, 0.]);

            let start_index = vertices.len() as u32 - 4;
            let a = start_index;
            let b = a + 1;
            let c = a + 2;
            let d = a + 3;
            self.add_quad_indices(indices, a, b, c, d);
        }
    }

    fn project_point_onto_line(p1: Vec2, p2: Vec2, p: Vec2) -> Vec2 {
        let v = p2 - p1;
        let w = p - p1;
        let proj_factor = w.dot(v) / v.dot(v);
        let p_proj = p1 + v * proj_factor;
        p_proj
    }

    fn add_quad_indices(&self, 
        indices: &mut Vec<u32>,
        a: u32,
        b: u32,
        c: u32,
        d: u32
    ) {
        indices.push(a);
        indices.push(b);
        indices.push(c);

        indices.push(b);
        indices.push(d);
        indices.push(c);
    }

    fn add_sharp_corner(&self, 
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        idx: usize,
        prev_idx: usize,
        next_idx: usize
    ) {
        let location = self.locations[idx];
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

            self.add_sharp_segment_indices(indices, idx);
        }
    }

    fn add_sharp_segment_indices(&self,
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

    fn total_width(&self) -> f32 {
        self.left_width + self.right_width
    }

    /// Calculate cap for unconnected path.
    fn calc_caps(&self, vertices: &mut Vec<[f32; 3]>, indices: &mut Vec<u32>) {
        // End cap
        let last = self.locations[self.locations.len() - 1];
        let second_last = self.locations[self.locations.len() - 2];
        let end_origo_left = calc_left_side_segment(second_last, last, self.left_width).1;
        let end_origo_right = calc_right_side_segment(second_last, last, self.right_width).1;
        let end_origo = end_origo_left.midpoint(end_origo_right);
        let end_segment_vec = self.locations[self.locations.len() - 2] - self.locations[self.locations.len() - 1];
        self.calc_cap(vertices, indices, end_origo, end_segment_vec, (vertices.len() - 1) as u32, (vertices.len() - 2) as u32);

        // Start cap
        let first = self.locations[0];
        let second = self.locations[1];
        let start_origo_left = calc_left_side_segment(first, second, self.left_width).0;
        let start_origo_right = calc_right_side_segment(first, second, self.right_width).0;
        let start_origo = start_origo_left.midpoint(start_origo_right);
        let start_segment_vec = self.locations[1] - self.locations[0];
        self.calc_cap(vertices, indices, start_origo, start_segment_vec, 0, 1);
    }

    /// Calculate cap for unconnected path.
    fn calc_cap(&self, 
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        origo: Vec2,
        segment_vec: Vec2,
        index_a: u32,
        index_b: u32
    ) {
        let CornerStyle::Rounded { resolution, .. } = self.corner_style else {
            return;
        };
        
        // Add origo as separate vertex
        let first_vertex_idx = vertices.len() as u32;
        //vertices.push([origo.x, origo.y, 0.]);

        // Add fan vertices
        let fan_vec = segment_vec.perp().normalize();
        let angle_increment = PI / (resolution / 2 - 1) as f32;
        let new_vertices: u32 = resolution as u32 / 2 - 2;
        for i in 1..new_vertices + 1 {
            let angle = i as f32 * angle_increment;
            
            let rotation_vec = Vec2::from_angle(angle);
            let vert = origo + (rotation_vec.rotate(fan_vec) * (self.total_width() / 2.));
            vertices.push([vert.x, vert.y, 0.]);
        }

        // First and last triangle, reuses existing vertices
        indices.push(index_a);
        indices.push(first_vertex_idx + 1);
        indices.push(first_vertex_idx);

        indices.push(index_a);
        indices.push(first_vertex_idx + new_vertices - 1);
        indices.push(index_b);

        // Middle triangles, only using new vertices
        for i in 1..new_vertices - 1 {
            indices.push(index_a);
            indices.push(first_vertex_idx + i + 1);
            indices.push(first_vertex_idx + i);
        }
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
        1.,
        Alignment::Center,
        CornerStyle::Rounded { radius: 0.25, resolution: 20 },
        false,
    );

    let mesh = path.make_mesh();
}