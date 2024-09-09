use bevy::{prelude::*, render::{mesh::{self, PrimitiveTopology}, render_asset::RenderAssetUsages}};

#[derive(Clone, Copy)]
pub enum CornerStyle {
    Sharp,
    Rounded {
        /// Number of vertices in a circle. A number proportional to the arc is used
        vertices: usize,
    },
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Center,
    LeftSide,

    /// The points are simply reversed to make the right side the left side
    RightSide,
}

#[derive(Clone)]
pub enum PathWidth {
    Constant(f32),
    Variable(Vec<f32>),
}

#[derive(Clone, Component)]
pub struct FlexPath {
    /// If first and last points are the same, the path is closed
    pub vertices: Vec<[f32; 3]>,
    pub width: PathWidth,
    pub corner_style: CornerStyle,
    pub alignment: Alignment,
}

impl Default for FlexPath {
    fn default() -> Self {
        FlexPath {
            vertices: Vec::new(),
            width: PathWidth::Constant(1.0),
            corner_style: CornerStyle::Sharp,
            alignment: Alignment::Center,
        }
    }
}

impl FlexPath {
    pub fn new(
        vertices: Vec<[f32; 3]>, 
        width: PathWidth, 
        alignment: Alignment, 
        corner_style: CornerStyle
    ) -> Self {
        // Should be removed when builder implemented
        let connected = vertices.first() == vertices.last();
        if let PathWidth::Variable(widths) = &width {
            if connected {
                assert_eq!(widths.len(), vertices.len());
            } else {
                assert_eq!(widths.len(), vertices.len() - 1);
            }
        }
        assert!(vertices.len() >= 2);
        /////////////////////////////////////////////

        FlexPath {
            vertices,
            width,
            corner_style,
            alignment,
        }
    }

    pub(crate) fn make_mesh(&self) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for i in 0..self.vertices.len() {
            self.calc_segment(i, &mut vertices, &mut indices);
        }
        
        if !self.is_connected() {
            //self.calc_caps(&mut vertices, &mut indices);
        }
        
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vertices,
            ).with_inserted_indices(
                mesh::Indices::U32(indices)
            )
    }

    fn calc_segment(&self, 
        segment_idx: usize, 
        vertices: &mut Vec<[f32; 3]>, indices: &mut Vec<u32>)
    {
        // Seg start
        if let Some(prev) = self.get_prev_idx(segment_idx) {

        } else {}

        // Seg end
        let next = self.get_next_idx(segment_idx);
    }

    /// Calculate cap for unconnected path.
    /// 
    fn _calc_cap(&self, _is_first: bool, _vertices: &mut Vec<[f32; 3]>, _indices: &mut Vec<u32>) {
        
    }

    pub fn is_connected(&self) -> bool {
        self.vertices.first() == self.vertices.last()
    }

    fn get_next_idx(&self, idx: usize) -> Option<usize> {
        if self.is_connected() || idx < self.vertices.len() - 1 {
            Some((idx + 1) % self.vertices.len())
        } else {
            None
        }
    }

    fn get_prev_idx(&self, idx: usize) -> Option<usize> {
        if self.is_connected() || idx > 0 {
            Some((idx + self.vertices.len() - 1) % self.vertices.len())
        } else {
            None
        }
    }
}