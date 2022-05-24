use super::{format::FString, pos_iter::CharPositionIter};
use crate::{
    batch::{quad::QuadMesh, Mesh},
    color::Color,
    packer::glyph::Glyphs,
    prelude::vertex::DefaultVertex,
    target::Target,
};
use glam::Vec2;
use image::RgbaImage;

//

/// #### `px` is the font size
///
/// #### `origin` is the offset
///
/// #### anchor
/// examples:
///  - default: `Some(Vec2::new(0.0, 0.0))` or `None`
///  - center text: `Some(Vec2::new(0.5, 0.5))`
pub fn text(
    target: &Target,
    string: &FString,
    glyphs: &mut Glyphs,
    px: f32,
    mut origin: Vec2,
    anchor: Option<Vec2>,
) -> Result<(Vec<DefaultVertex>, Vec<u32>), &'static str> {
    let sdf = glyphs.is_sdf();

    // setup glyphs
    for (c, format) in string.chars() {
        glyphs.queue(c, px as u16, format.font);
    }
    glyphs.flush(target)?;

    if let Some(anchor) = anchor {
        let mut iter = CharPositionIter::new(string, glyphs, px, sdf);
        if let Some(first) = iter.next() {
            let mut min_x = first.x;
            let mut min_y = first.y;
            let mut max_x = first.x + first.width as i32;
            let mut max_y = first.y + first.height as i32;

            for c in iter {
                min_x = min_x.min(c.x);
                min_y = min_y.min(c.y);
                max_x = max_x.max(c.x + c.width as i32);
                max_y = max_y.max(c.y + c.height as i32);
            }

            let w = max_x - min_x;
            let h = max_y - min_y;
            origin -= anchor * Vec2::new(w as f32, h as f32);
        }
    }

    // gen vbo
    let vertices: Vec<_> = CharPositionIter::new(string, glyphs, px, sdf)
        .map(|c| {
            let tex = glyphs
                .get_indexed(c.index, px as u16, c.format.font)
                .unwrap();
            let col = Color::new(c.format.color.x, c.format.color.y, c.format.color.z, 1.0);

            // TODO: mesh anchoring
            let size = Vec2::new(c.width as f32, c.height as f32);
            QuadMesh {
                pos: Vec2::new(c.x as f32, c.y as f32) + origin + 0.5 * size,
                size,
                col,
                tex,
            }
        })
        .collect();

    let indices = vertices.clone();

    let vertices = vertices
        .into_iter()
        .flat_map(|mesh| mesh.vertices())
        .collect();

    let indices = indices
        .into_iter()
        .enumerate()
        .flat_map(|(i, mesh)| mesh.indices(i as u32))
        .collect();

    Ok((vertices, indices))
}

pub fn baked_text<'s, F>(string: F, glyphs: &Glyphs, px: f32) -> Option<RgbaImage>
where
    F: Into<&'s FString> + Clone,
{
    let sdf = glyphs.is_sdf();
    let string: &FString = string.into();

    // text bounding box
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;

    for c in CharPositionIter::new(string, glyphs, px, sdf) {
        x_min = x_min.min(c.x);
        y_min = y_min.min(c.y);

        x_max = x_max.max(c.x + c.width as i32);
        y_max = y_max.max(c.y + c.height as i32);
    }
    let width = (x_max - x_min).max(0) as usize;
    let height = (y_max - y_min).max(0) as usize;

    let mut text = vec![0; width * height * 4];
    for c in CharPositionIter::new(string, glyphs, px, sdf) {
        let (metrics, bitmap) = glyphs
            .get_font(c.format.font)
            .unwrap()
            .rasterize_indexed(c.index, px, sdf);

        for (index, pixel) in bitmap.iter().enumerate() {
            let x = (c.x - x_min) as usize + index % metrics.width;
            let y = (c.y - y_min) as usize + index / metrics.width;
            let index = (x + y * width) * 4;
            let a = *pixel as f32 / 255.0;

            let output_r = &mut text[index];
            *output_r =
                (*output_r as f32 * (1.0 - a) + (255.0 * c.format.color.x) as f32 * a) as u8;
            let output_g = &mut text[index + 1];
            *output_g =
                (*output_g as f32 * (1.0 - a) + (255.0 * c.format.color.y) as f32 * a) as u8;
            let output_b = &mut text[index + 2];
            *output_b =
                (*output_b as f32 * (1.0 - a) + (255.0 * c.format.color.z) as f32 * a) as u8;
            let output_a = &mut text[index + 3];
            *output_a = (*output_a).max(*pixel);
        }
    }

    RgbaImage::from_raw(width as u32, height as u32, text)
}
