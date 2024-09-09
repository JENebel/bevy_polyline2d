use bevy::{prelude::*, render::{mesh, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}};

use crate::vector_extensions::{intersection_point_legacy, VectorExtensions};

#[derive(Clone, Component)]
pub struct Polyline2d {
    pub path: Vec<[f32; 3]>,
    /// Width of the line
    pub width: f32,
    /// Whether the line is closed or not, meaning connecting the last point back to the first
    pub closed: bool,
    /// What type of line placement to use
    pub line_placement: Align,
}

impl Default for Polyline2d {
    fn default() -> Self {
        Polyline2d {
            path: Vec::new(),
            width: 1.0,
            closed: false,
            line_placement: Align::default(),
        }
    }
}

#[derive(Debug)]
enum Orientation {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    /// Line will have half the width on each side of the path
    Center,
    /// Line will be entirely on the left side of the path
    Left,
}

impl Default for Align {
    fn default() -> Self {
        Align::Center
    }
}

fn orientation_test(p1: Vec3, p2: Vec3, p3: Vec3) -> Orientation {
    let v1 = p2 - p1;
    let v2 = p3 - p2;
    let cross = v1.cross(v2);
    if cross.z > f32::EPSILON {
        Orientation::Left
    } else if cross.z < -f32::EPSILON {
        Orientation::Right
    } else {
        Orientation::Straight
    }
}

impl Polyline2d {
    pub(crate) fn make_mesh(&self) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut points: Vec<[f32; 3]> = self.path.clone();

        //remove duplicate points
        let mut i = 0;
        while i < points.len() - 1 {
            if points[i] == points[i + 1] {
                points.remove(i);
            } else {
                i += 1;
            }
        }

        if self.closed {
            points.push(points[0]);
            points.push(points[1]);
        }

        let only_inner = self.line_placement == Align::Left;

        let width = if only_inner { self.width } else { self.width/2. };

        // First vertex
        let p0 = Vec3::from(points[0]);
        let p1 = Vec3::from(points[1]);
        let v0 = p1 - p0;
        let ortho = v0.ortho_normal();
        let vert1 = p0 + ortho * width;
        let vert2 = if only_inner { p0 } else { p0 - ortho * width };
        vertices.push([vert1.x, vert1.y, vert1.z]);
        vertices.push([vert2.x, vert2.y, vert2.z]);
        //
        
        for points in points.windows(3) {
            let p1 = Vec3::from(points[0]);
            let p2 = Vec3::from(points[1]);
            let p3 = Vec3::from(points[2]);
            let v1 = p2 - p1;
            let v2 = p3 - p2;

            let ortho1 = v1.ortho_normal();
            let ortho2 = v2.ortho_normal();

            match orientation_test(p1, p2, p3) {
                Orientation::Left => {
                    if only_inner {
                        let inner = intersection_point_legacy(
                            p1 + ortho1 * width, 
                            p2 + ortho1 * width, 

                            p2 + ortho2 * width, 
                            p3 + ortho2 * width
                        );

                        vertices.push([inner.x, inner.y, inner.z]); // 2
                        vertices.push([p2.x, p2.y, p2.z]); // 3

                        let i = vertices.len() - 4;

                        indices.push(i as u32);
                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 2);

                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 3);
                        indices.push(i as u32 + 2);
                    } else {
                        let outer2 = p2 - ortho2 * (width);
                        let outer1 = p2 - ortho1 * (width);
                        let inner = intersection_point_legacy(
                            p1 + ortho1 * (width), 
                            p2 + ortho1 * (width), 

                            p2 + ortho2 * (width), 
                            p3 + ortho2 * (width)
                        );

                        vertices.push([outer1.x, outer1.y, outer1.z]); // 2
                        vertices.push([inner.x, inner.y, inner.z]); // 3
                        vertices.push([outer2.x, outer2.y, outer2.z]); // 4

                        let i = vertices.len() - 5;

                        indices.push(i as u32);
                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 3);

                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 2);
                        indices.push(i as u32 + 3);

                        indices.push(i as u32 + 2);
                        indices.push(i as u32 + 4);
                        indices.push(i as u32 + 3);
                    }
                },
                Orientation::Right => {
                    if only_inner {
                        let inner = intersection_point_legacy(
                            p1 + ortho1 * width, 
                            p2 + ortho1 * width, 

                            p2 + ortho2 * width, 
                            p3 + ortho2 * width
                        );

                        vertices.push([inner.x, inner.y, inner.z]); // 3
                        vertices.push([p2.x, p2.y, p2.z]); // 2

                        let i = vertices.len() - 4;

                        indices.push(i as u32);
                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 2);

                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 3);
                        indices.push(i as u32 + 2);
                    } else {
                        let outer2 = p2 + ortho2 * (width);
                        let outer1 = p2 + ortho1 * (width);
                        let inner = intersection_point_legacy(
                            p1 - ortho1 * (width), 
                            p2 - ortho1 * (width), 

                            p2 - ortho2 * (width), 
                            p3 - ortho2 * (width)
                        );

                        vertices.push([outer1.x, outer1.y, outer1.z]); // 2
                        vertices.push([outer2.x, outer2.y, outer2.z]); // 3
                        vertices.push([inner.x, inner.y, inner.z]); // 4

                        let i = vertices.len() - 5;

                        indices.push(i as u32);
                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 2);

                        indices.push(i as u32 + 1);
                        indices.push(i as u32 + 4);
                        indices.push(i as u32 + 2);

                        indices.push(i as u32 + 2);
                        indices.push(i as u32 + 4);
                        indices.push(i as u32 + 3);
                    }
                },
                Orientation::Straight => {
                    /*let vert1 = p2 + ortho1 * width;
                    let vert2 = if only_inner { p2 } else { p2 - ortho1 * width };
                    vertices.push([vert1.x, vert1.y, vert1.z]);
                    vertices.push([vert2.x, vert2.y, vert2.z]);

                    let i = vertices.len() - 4;

                    indices.push(i as u32);
                    indices.push(i as u32 + 1);
                    indices.push(i as u32 + 2);

                    indices.push(i as u32 + 1);
                    indices.push(i as u32 + 3);
                    indices.push(i as u32 + 2);*/
                },
            }
        }

        // Last vertex
        if !self.closed {
            let p0 = Vec3::from(points[points.len() - 2]);
            let p1 = Vec3::from(points[points.len() - 1]);
            let v0 = p1 - p0;
            let ortho = v0.ortho_normal();
            let vert1 = p1 + ortho * width;
            let vert2 = if only_inner { p1 } else { p1 - ortho * width };
            vertices.push([vert1.x, vert1.y, vert1.z]);
            vertices.push([vert2.x, vert2.y, vert2.z]);
            let i = vertices.len() - 4;

            indices.push(i as u32);
            indices.push(i as u32 + 1);
            indices.push(i as u32 + 2);

            indices.push(i as u32 + 1);
            indices.push(i as u32 + 3);
            indices.push(i as u32 + 2);
        } else {
            // Replace two last indices with two first
            for i in indices.len() - 9..indices.len() {
                if indices[i] == vertices.len() as u32 - 1 {
                    indices[i] = 1;
                } else if indices[i] == vertices.len() as u32 - 2 {
                    indices[i] = 0;
                }
            }

            vertices[0] = vertices[vertices.len() - 2];
            vertices[1] = vertices[vertices.len() - 1];

            vertices.pop();
            vertices.pop();
        }
        
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vertices,
            ).with_inserted_indices(
                mesh::Indices::U32(indices)
            )
    }
}