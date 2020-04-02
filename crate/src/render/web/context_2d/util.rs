use crate::store::dynamics::render::field::Field;
use crate::store::dynamics::render::field::Get;

pub fn clear_canvas_field(field: &Field, context: &web_sys::CanvasRenderingContext2d) {
    context.clear_rect(
        0.0,
        0.0,
        field.get_width() as f64,
        field.get_height() as f64,
    );
}
