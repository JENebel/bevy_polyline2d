use std::f32::consts::PI;

use bevy::{prelude::*, render::{mesh::{self, PrimitiveTopology}, render_asset::RenderAssetUsages}};

use crate::vector_utils::*;

#[derive(Clone, Component)]
pub struct FlexLine {
    pub locations: Vec<Vec2>,
    pub width: f32,
    pub corner_style: CornerStyle,
    pub alignment: Alignment,
    pub connection_style: ConnectionStyle,
    pub color: LineColor,
}

#[derive(Clone, Copy)]
pub enum CornerStyle {
    Sharp,
    Rounded {
        radius: f32,
        /// Number of vertices in a circle. A number proportional to the arc is used
        resolution: usize,
    },
}

#[derive(Clone, Copy, PartialEq)]
pub enum Alignment {
    Center,
    LeftSide,
    RightSide,
    /// Offset from center to the right
    Offset(f32),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ConnectionStyle {
    Connected,
    Unconnected,
}

#[derive(Clone)]
pub enum LineColor {
    Fill(Color),
    GradientAcross {
        left: Color,
        right: Color,
    },
    PerVertex(Vec<Color>),
}

impl LineColor {
    /// gradient: 1 for right color, -1 for left color
    fn get(&self, index: usize, gradient: f32) -> [f32; 4] {
        match self {
            LineColor::Fill(color) => {
                let color = color.to_srgba();
                [color.red, color.green, color.blue, color.alpha]
            },
            LineColor::GradientAcross { left, right } => {
                let gradient = (gradient + 1.) / 2.;
                let color = left.to_srgba().mix(&right.to_srgba(), gradient);
                [color.red, color.green, color.blue, color.alpha]
            },
            LineColor::PerVertex(vertex_colors) => {
                let color = vertex_colors[index].to_srgba();
                [color.red, color.green, color.blue, color.alpha]
            },
        }
    }
}

impl Alignment {
    fn left_width(&self, width: f32) -> f32 {
        match self {
            Alignment::Center => width / 2.,
            Alignment::LeftSide => width,
            Alignment::RightSide => 0.,
            Alignment::Offset(offset) => width / 2. - offset,
        }
    }

    fn right_width(&self, width: f32) -> f32 {
        match self {
            Alignment::Center => width / 2.,
            Alignment::LeftSide => 0.,
            Alignment::RightSide => width,
            Alignment::Offset(offset) => width / 2. + offset,
        }
    }
}

impl Default for FlexLine {
    fn default() -> Self {
        FlexLine {
            locations: Vec::new(),
            width: 1.,
            corner_style: CornerStyle::Sharp,
            alignment: Alignment::Center,
            connection_style: ConnectionStyle::Connected,
            color: LineColor::Fill(Color::WHITE),
        }
    }
}

impl FlexLine {
    pub fn new(
        locations: Vec<Vec2>, 
        width: f32, 
        alignment: Alignment, 
        corner_style: CornerStyle,
        connection_style: ConnectionStyle,
        color: LineColor
    ) -> Self {
        assert!(locations.len() >= 2, "FlexPath must have at least 2 locations");
        FlexLine {
            locations,
            width,
            corner_style,
            alignment,
            connection_style: connection_style,
            color,
        }
    }

    fn left_width(&self) -> f32 {
        self.alignment.left_width(self.width)
    }

    fn right_width(&self) -> f32 {
        self.alignment.right_width(self.width)
    }

    fn is_connected(&self) -> bool {
        match self.connection_style {
            ConnectionStyle::Connected => true,
            ConnectionStyle::Unconnected => false,
        }
    }

    fn get_next_idx(&self, idx: usize) -> Option<usize> {
        if self.is_connected() || idx < self.locations.len() - 1 {
            Some((idx + 1) % self.locations.len())
        } else {
            None
        }
    }

    fn get_prev_idx(&self, idx: usize) -> Option<usize> {
        if self.is_connected() || idx > 0 {
            Some((idx as isize + self.locations.len() as isize - 1) as usize % self.locations.len())
        } else {
            None
        }
    }

    pub(crate) fn make_mesh(&self) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut colors: Vec<[f32; 4]> = Vec::new();


        if self.is_connected() {
            // Add dummy vertices to the beginnig. These will be replaced by the 2 last vertices at the end
            vertices.push([0., 0., 0.]);
            vertices.push([0., 0., 0.]);
            colors.push(self.color.get(0, -1.));
            colors.push(self.color.get(0,  1.));
        }

        for i in 0..self.locations.len() {
            self.add_corner(i, &mut vertices, &mut indices, &mut colors);
        }
        
        if !self.is_connected() {
            self.calc_caps(&mut vertices, &mut indices, &mut colors);
        } else {
            // Replace the dummy vertices with the last 2 vertices
            vertices[1] = vertices.pop().unwrap();
            vertices[0] = vertices.pop().unwrap();
            colors[1] = colors.pop().unwrap();
            colors[0] = colors.pop().unwrap();
            
            // Replace instances of the last 2 vertices in the indices
            let index_count = indices.len();
            if let CornerStyle::Rounded { .. } = self.corner_style {
                indices[index_count - 4] = 0;
                indices[index_count - 2] = 1;
                indices[index_count - 1] = 0;
            }
        }

        // The other color styles built along with the mesh
        if let LineColor::Fill(color) = self.color {
            colors = vec![[
                    color.to_srgba().red, 
                    color.to_srgba().green, 
                    color.to_srgba().blue, 
                    color.to_srgba().alpha]; 
                vertices.len()];
        }

        return Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_indices(mesh::Indices::U32(indices))
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }

    fn add_corner(&self, 
        index: usize, 
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        colors: &mut Vec<[f32; 4]>
    ) {
        let location = self.locations[index];
        let Some(prev_idx) = self.get_prev_idx(index) else {
            let next = self.locations[index + 1];
            let left_vert = calc_left_side_segment(location, next, self.left_width()).0;
            let right_vert = calc_right_side_segment(location, next, self.right_width()).0;
            vertices.push([left_vert.x, left_vert.y, 0.]);
            vertices.push([right_vert.x, right_vert.y, 0.]);
            colors.push(self.color.get(index, -1.));
            colors.push(self.color.get(index,  1.));
            return;
        };

        let Some(next_idx) = self.get_next_idx(index) else {
            // Last segment
            let prev = self.locations[index - 1];
            let left_vert = calc_left_side_segment(prev, location, self.left_width()).1;
            let right_vert = calc_right_side_segment(prev, location, self.right_width()).1;
            vertices.push([left_vert.x, left_vert.y, 0.]);
            vertices.push([right_vert.x, right_vert.y, 0.]);
            colors.push(self.color.get(index, -1.));
            colors.push(self.color.get(index,  1.));

            let a = (vertices.len() - 4) as u32;
            let b = a + 1;
            let c = a + 2;
            let d = a + 3;
            self.add_quad_indices(indices, a, b, c, d);
            return;
        };

        let location = self.locations[index];
        let prev = self.locations[prev_idx];
        let next = self.locations[next_idx];

        match self.corner_style {
            CornerStyle::Sharp => self.add_sharp_corner(vertices, indices, colors, index, prev_idx, next_idx),
            CornerStyle::Rounded { radius, resolution } => self.add_rounded_corner(vertices, indices, colors, index, location, prev, next, radius, resolution),
        }
    }

    fn add_rounded_corner(&self,
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        colors: &mut Vec<[f32; 4]>,
        index: usize,
        location: Vec2,
        prev: Vec2,
        next: Vec2,
        radius: f32,
        resolution: usize
    ) {
        let orientation = orientation_test(prev, location, next);
        if orientation == Orientation::Straight {
            return;
        }

        let (side_a, side_b) = if orientation == Orientation::Right {(
            calc_right_side_segment(prev, location, self.right_width()),
            calc_right_side_segment(location, next, self.right_width()))
        } else {(
            calc_left_side_segment(prev, location, self.left_width()),
            calc_left_side_segment(location, next, self.left_width()))
        };
        
        let inner_angle = if orientation == Orientation::Right {
            -(next - location).angle_between(prev - location)
        } else {
            (location - next).angle_between(location - prev)
        };
        
        let corner_angle = 2. * PI - inner_angle;

        let corner_origo = {
            let intersection = intersection_point(
                side_a.0, side_a.1 - side_a.0,
                side_b.1, side_b.0 - side_b.1
            ).unwrap();

            let angle = if orientation == Orientation::Right {
                -inner_angle / 2.
            } else {
                inner_angle / 2.
            };
            let rotation_vec = Vec2::from_angle(angle);
            let towards_origo = rotation_vec.rotate(next - location).normalize();
            intersection + towards_origo * (radius / (corner_angle / 2.).sin())
        };

        let out_vec = {
            // Can't use same side, as radius=0 won't work then
            let other_side = if orientation == Orientation::Right {
                calc_left_side_segment(prev, location, self.right_width())
            } else {
                calc_right_side_segment(prev, location, self.left_width())
            };
            let projected = project_point_onto_line(corner_origo, other_side.0, other_side.1);
            projected - corner_origo
        };
        
        let fan_count: i32 = 2.max((resolution as f32 / (2. * PI) * (corner_angle - PI)) as i32);
        let mut angle_step_size = (corner_angle - PI) / fan_count as f32;

        if orientation == Orientation::Right {
            angle_step_size = -angle_step_size;
        }

        for i in 0..fan_count + 1 {
            let angle = i as f32 * angle_step_size;
            let rotation_vec = Vec2::from_angle(angle);
            let dir_vec = rotation_vec.rotate(out_vec).normalize();
            let outer_vert = corner_origo + dir_vec * (radius + self.width); // left
            let inner_vert = corner_origo + dir_vec * radius; // right

            if orientation == Orientation::Right {
                vertices.push([outer_vert.x, outer_vert.y, 0.]);
                vertices.push([inner_vert.x, inner_vert.y, 0.]);
            } else {
                vertices.push([inner_vert.x, inner_vert.y, 0.]);
                vertices.push([outer_vert.x, outer_vert.y, 0.]);
            }
            colors.push(self.color.get(index, -1.));
            colors.push(self.color.get(index,  1.));
            
            let start_index = vertices.len() as u32 - 4;
            let a = start_index;
            let b = a + 1;
            let c = a + 2;
            let d = a + 3;
            self.add_quad_indices(indices, a, b, c, d);
        }
    }

    fn add_quad_indices(&self, 
        indices: &mut Vec<u32>,
        a: u32, b: u32,
        c: u32, d: u32
    ) {
        indices.push(a);
        indices.push(b);
        indices.push(c);

        indices.push(b);
        indices.push(d);
        indices.push(c);
    }

    /// Add a sharp corner to the mesh, by intersecting the 2 sides, and adding a vertex at each intersection.
    fn add_sharp_corner(&self, 
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        colors: &mut Vec<[f32; 4]>,
        index: usize,
        prev_idx: usize,
        next_idx: usize
    ) {
        let location = self.locations[index];
        let prev = self.locations[prev_idx];
        let next = self.locations[next_idx];

        let left_side_a = calc_left_side_segment(prev, location, self.left_width());
        let left_side_b = calc_left_side_segment(location, next, self.left_width());
        
        let right_side_a = calc_right_side_segment(prev, location, self.right_width());
        let right_side_b = calc_right_side_segment(location, next, self.right_width());
        
        let left_intersection = intersection_point(
                left_side_a.0, left_side_a.1 - left_side_a.0,
                left_side_b.1, left_side_b.0 - left_side_b.1);
                
        let right_intersection = intersection_point(
                right_side_a.0, right_side_a.1 - right_side_a.0,
                right_side_b.1, right_side_b.0 - right_side_b.1);

        // If the intersection is None, the corner is straight, and we don't do anything
        if let (Some(left_vert), Some(right_vert)) = (left_intersection, right_intersection) {
            vertices.push([left_vert.x, left_vert.y, 0.]);
            vertices.push([right_vert.x, right_vert.y, 0.]);
            colors.push(self.color.get(index, -1.));
            colors.push(self.color.get(index, 1.));

            let start = index * 2;
            let prev = prev_idx as u32 * 2;

            let a = prev;
            let b = a + 1;

            let c = start as u32;
            let d = c + 1;
            self.add_quad_indices(indices, a, b, c, d);
        }
    }

    /// Calculate cap for unconnected path.
    fn calc_caps(&self, vertices: &mut Vec<[f32; 3]>, indices: &mut Vec<u32>, colors: &mut Vec<[f32; 4]>) {
        // End cap
        let last = self.locations[self.locations.len() - 1];
        let second_last = self.locations[self.locations.len() - 2];
        let end_origo_left = calc_left_side_segment(second_last, last, self.left_width()).1;
        let end_origo_right = calc_right_side_segment(second_last, last, self.right_width()).1;
        let end_origo = end_origo_left.midpoint(end_origo_right);
        let end_segment_vec = self.locations[self.locations.len() - 2] - self.locations[self.locations.len() - 1];
        self.add_cap(vertices, indices, end_origo, end_segment_vec, (vertices.len() - 1) as u32, (vertices.len() - 2) as u32,
                     self.locations.len() - 1, colors, -1.);
        

        // Start cap
        let first = self.locations[0];
        let second = self.locations[1];
        let start_origo_left = calc_left_side_segment(first, second, self.left_width()).0;
        let start_origo_right = calc_right_side_segment(first, second, self.right_width()).0;
        let start_origo = start_origo_left.midpoint(start_origo_right);
        let start_segment_vec = self.locations[1] - self.locations[0];
        self.add_cap(vertices, indices, start_origo, start_segment_vec, 0, 1,
                     0, colors, 1.);
    }

    /// Calculate cap for unconnected path.
    fn add_cap(&self, 
        vertices: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u32>,
        origo: Vec2,
        segment_vec: Vec2,
        index_a: u32,
        index_b: u32,
        
        index: usize,
        colors: &mut Vec<[f32; 4]>,
        side_factor: f32
    ) {
        let CornerStyle::Rounded { resolution, .. } = self.corner_style else {
            return;
        };
        
        // Add origo as separate vertex
        let origo_idx = vertices.len() as u32;
        vertices.push([origo.x, origo.y, 0.]);
        colors.push(self.color.get(index, 0.));
        
        // Add fan vertices
        let first_vertex_idx = vertices.len() as u32;

        let fan_vec = segment_vec.perp().normalize();
        let triangles: u32 = 1.max(resolution as i32 / 2 - 2) as u32;
        let angle_increment = PI / (triangles + 1) as f32;
        
        for i in 1..triangles + 1 {
            let angle = i as f32 * angle_increment;
            let rotation_vec = Vec2::from_angle(angle);
            let vert = origo + (rotation_vec.rotate(fan_vec) * (self.width / 2.));
            vertices.push([vert.x, vert.y, 0.]);
            let gradient: f32 = ((angle - PI / 2.).sin()) * side_factor;
            colors.push(self.color.get(index, gradient));
        }

        // First and last triangle, reuses existing vertices
        indices.push(origo_idx);
        indices.push(first_vertex_idx);
        indices.push(index_a);

        indices.push(origo_idx);
        indices.push(first_vertex_idx + triangles - 1);
        indices.push(index_b);

        // Middle triangles, only using new vertices
        if triangles > 2 {
            for i in 0..triangles - 1 {
                indices.push(origo_idx);
                indices.push(first_vertex_idx + i + 1);
                indices.push(first_vertex_idx + i);
            }
        }
    }
}