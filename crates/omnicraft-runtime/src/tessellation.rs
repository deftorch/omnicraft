//! Lyon Tessellation Integration
//!
//! Provides GPU-ready path tessellation using the Lyon library.
//! This enables smooth anti-aliased rendering of complex 2D shapes.

use glam::Vec2;
use lyon::path::Path;
use lyon::path::path::Builder;
use lyon::tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions,
    StrokeTessellator, StrokeVertex, VertexBuffers,
};

/// A vertex with position and color for rendering
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

/// Tessellated geometry ready for rendering
#[derive(Debug, Clone, Default)]
pub struct TessellatedMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl TessellatedMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

/// Tessellator for converting paths to triangles
pub struct PathTessellator {
    fill_tessellator: FillTessellator,
    stroke_tessellator: StrokeTessellator,
}

impl PathTessellator {
    pub fn new() -> Self {
        Self {
            fill_tessellator: FillTessellator::new(),
            stroke_tessellator: StrokeTessellator::new(),
        }
    }

    /// Tessellate a filled circle
    pub fn tessellate_circle_fill(
        &mut self,
        center: Vec2,
        radius: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_circle_path(center, radius);
        self.tessellate_fill(&path, color)
    }

    /// Tessellate a stroked circle
    pub fn tessellate_circle_stroke(
        &mut self,
        center: Vec2,
        radius: f32,
        stroke_width: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_circle_path(center, radius);
        self.tessellate_stroke(&path, stroke_width, color)
    }

    /// Tessellate a filled rectangle
    pub fn tessellate_rectangle_fill(
        &mut self,
        center: Vec2,
        width: f32,
        height: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_rectangle_path(center, width, height);
        self.tessellate_fill(&path, color)
    }

    /// Tessellate a stroked rectangle
    pub fn tessellate_rectangle_stroke(
        &mut self,
        center: Vec2,
        width: f32,
        height: f32,
        stroke_width: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_rectangle_path(center, width, height);
        self.tessellate_stroke(&path, stroke_width, color)
    }

    /// Tessellate a filled polygon
    pub fn tessellate_polygon_fill(
        &mut self,
        points: &[Vec2],
        color: [f32; 4],
    ) -> TessellatedMesh {
        if points.len() < 3 {
            return TessellatedMesh::new();
        }

        let path = self.build_polygon_path(points);
        self.tessellate_fill(&path, color)
    }

    /// Tessellate a stroked polygon
    pub fn tessellate_polygon_stroke(
        &mut self,
        points: &[Vec2],
        stroke_width: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        if points.len() < 2 {
            return TessellatedMesh::new();
        }

        let path = self.build_polygon_path(points);
        self.tessellate_stroke(&path, stroke_width, color)
    }

    /// Tessellate a line
    pub fn tessellate_line(
        &mut self,
        from: Vec2,
        to: Vec2,
        stroke_width: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_line_path(from, to);
        self.tessellate_stroke(&path, stroke_width, color)
    }

    /// Tessellate an ellipse fill
    pub fn tessellate_ellipse_fill(
        &mut self,
        center: Vec2,
        rx: f32,
        ry: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_ellipse_path(center, rx, ry);
        self.tessellate_fill(&path, color)
    }

    /// Tessellate an ellipse stroke
    pub fn tessellate_ellipse_stroke(
        &mut self,
        center: Vec2,
        rx: f32,
        ry: f32,
        stroke_width: f32,
        color: [f32; 4],
    ) -> TessellatedMesh {
        let path = self.build_ellipse_path(center, rx, ry);
        self.tessellate_stroke(&path, stroke_width, color)
    }

    // Private path builders

    fn build_circle_path(&self, center: Vec2, radius: f32) -> Path {
        let mut builder = Path::builder();
        
        // Approximate circle with cubic bezier curves
        let kappa = 0.5522847498; // Magic number for circle approximation
        let k = radius * kappa;

        builder.begin(lyon::geom::point(center.x + radius, center.y));
        builder.cubic_bezier_to(
            lyon::geom::point(center.x + radius, center.y + k),
            lyon::geom::point(center.x + k, center.y + radius),
            lyon::geom::point(center.x, center.y + radius),
        );
        builder.cubic_bezier_to(
            lyon::geom::point(center.x - k, center.y + radius),
            lyon::geom::point(center.x - radius, center.y + k),
            lyon::geom::point(center.x - radius, center.y),
        );
        builder.cubic_bezier_to(
            lyon::geom::point(center.x - radius, center.y - k),
            lyon::geom::point(center.x - k, center.y - radius),
            lyon::geom::point(center.x, center.y - radius),
        );
        builder.cubic_bezier_to(
            lyon::geom::point(center.x + k, center.y - radius),
            lyon::geom::point(center.x + radius, center.y - k),
            lyon::geom::point(center.x + radius, center.y),
        );
        builder.close();

        builder.build()
    }

    fn build_rectangle_path(&self, center: Vec2, width: f32, height: f32) -> Path {
        let mut builder = Path::builder();
        
        let half_w = width / 2.0;
        let half_h = height / 2.0;

        builder.begin(lyon::geom::point(center.x - half_w, center.y - half_h));
        builder.line_to(lyon::geom::point(center.x + half_w, center.y - half_h));
        builder.line_to(lyon::geom::point(center.x + half_w, center.y + half_h));
        builder.line_to(lyon::geom::point(center.x - half_w, center.y + half_h));
        builder.close();

        builder.build()
    }

    fn build_polygon_path(&self, points: &[Vec2]) -> Path {
        let mut builder = Path::builder();
        
        if let Some(first) = points.first() {
            builder.begin(lyon::geom::point(first.x, first.y));
            
            for point in points.iter().skip(1) {
                builder.line_to(lyon::geom::point(point.x, point.y));
            }
            
            builder.close();
        }

        builder.build()
    }

    fn build_line_path(&self, from: Vec2, to: Vec2) -> Path {
        let mut builder = Path::builder();
        
        builder.begin(lyon::geom::point(from.x, from.y));
        builder.line_to(lyon::geom::point(to.x, to.y));
        builder.end(false);

        builder.build()
    }

    fn build_ellipse_path(&self, center: Vec2, rx: f32, ry: f32) -> Path {
        let mut builder = Path::builder();
        
        // Approximate ellipse with cubic bezier curves
        let kappa = 0.5522847498;
        let kx = rx * kappa;
        let ky = ry * kappa;

        builder.begin(lyon::geom::point(center.x + rx, center.y));
        builder.cubic_bezier_to(
            lyon::geom::point(center.x + rx, center.y + ky),
            lyon::geom::point(center.x + kx, center.y + ry),
            lyon::geom::point(center.x, center.y + ry),
        );
        builder.cubic_bezier_to(
            lyon::geom::point(center.x - kx, center.y + ry),
            lyon::geom::point(center.x - rx, center.y + ky),
            lyon::geom::point(center.x - rx, center.y),
        );
        builder.cubic_bezier_to(
            lyon::geom::point(center.x - rx, center.y - ky),
            lyon::geom::point(center.x - kx, center.y - ry),
            lyon::geom::point(center.x, center.y - ry),
        );
        builder.cubic_bezier_to(
            lyon::geom::point(center.x + kx, center.y - ry),
            lyon::geom::point(center.x + rx, center.y - ky),
            lyon::geom::point(center.x + rx, center.y),
        );
        builder.close();

        builder.build()
    }

    // Tessellation methods

    fn tessellate_fill(&mut self, path: &Path, color: [f32; 4]) -> TessellatedMesh {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        {
            let mut builder = BuffersBuilder::new(&mut geometry, |vertex: FillVertex| Vertex {
                position: vertex.position().to_array(),
                color,
            });

            let options = FillOptions::default();
            
            if self.fill_tessellator.tessellate_path(path, &options, &mut builder).is_err() {
                return TessellatedMesh::new();
            }
        }

        TessellatedMesh {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }

    fn tessellate_stroke(&mut self, path: &Path, width: f32, color: [f32; 4]) -> TessellatedMesh {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        {
            let mut builder = BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| Vertex {
                position: vertex.position().to_array(),
                color,
            });

            let options = StrokeOptions::default().with_line_width(width);
            
            if self.stroke_tessellator.tessellate_path(path, &options, &mut builder).is_err() {
                return TessellatedMesh::new();
            }
        }

        TessellatedMesh {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }
}

impl Default for PathTessellator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tessellate_circle() {
        let mut tessellator = PathTessellator::new();
        let mesh = tessellator.tessellate_circle_fill(
            Vec2::new(0.0, 0.0),
            50.0,
            [1.0, 0.0, 0.0, 1.0],
        );
        
        assert!(!mesh.is_empty());
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.indices.is_empty());
    }

    #[test]
    fn test_tessellate_rectangle() {
        let mut tessellator = PathTessellator::new();
        let mesh = tessellator.tessellate_rectangle_fill(
            Vec2::new(0.0, 0.0),
            100.0,
            50.0,
            [0.0, 1.0, 0.0, 1.0],
        );
        
        assert!(!mesh.is_empty());
        // Rectangle should have 4 vertices and 6 indices (2 triangles)
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
    }

    #[test]
    fn test_tessellate_polygon() {
        let mut tessellator = PathTessellator::new();
        let points = vec![
            Vec2::new(0.0, -50.0),
            Vec2::new(50.0, 50.0),
            Vec2::new(-50.0, 50.0),
        ];
        let mesh = tessellator.tessellate_polygon_fill(&points, [0.0, 0.0, 1.0, 1.0]);
        
        assert!(!mesh.is_empty());
        // Triangle should have 3 vertices and 3 indices
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.indices.len(), 3);
    }

    #[test]
    fn test_tessellate_line() {
        let mut tessellator = PathTessellator::new();
        let mesh = tessellator.tessellate_line(
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 100.0),
            2.0,
            [1.0, 1.0, 1.0, 1.0],
        );
        
        assert!(!mesh.is_empty());
    }
}
