use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::iter::Iterator;

use crate::store;
use store::dynamics::render::layout::LayoutContext;
use store::dynamics::field::FieldContext;
use store::dynamics::block::BlockContext;

pub fn init(layout_context: &mut LayoutContext, canvas_element: &web_sys::HtmlCanvasElement, canvas_context: &web_sys::CanvasRenderingContext2d) {

    // initialize Game field area size
    layout_context.set_width(400);
    layout_context.set_height(600);

    // get CanvasElement and initialize

    canvas_element.set_width(layout_context.get_width());
    canvas_element.set_height(layout_context.get_height());

    canvas_context.set_line_width(1.0);
    canvas_context.set_stroke_style(&JsValue::from("rgba(255,255,255,0.6)"));
    canvas_context.set_fill_style(&JsValue::from("rgba(255,255,255,0.7)"));

    // outline

    let outline = web_sys::Path2d::new().unwrap();

    canvas_context.begin_path();
    outline.rect(
        layout_context.get_render_boundary(),
        layout_context.get_render_boundary(),
        layout_context.get_horizon_boundary(),
        layout_context.get_vertical_boundary(),
    );
    canvas_context.stroke_with_path(&outline);

    // grid

    let grid = web_sys::Path2d::new().unwrap();

    let start = (layout_context.get_step() + layout_context.get_render_boundary()) as usize;

    // horizon line
    let v_end = layout_context.get_vertical_boundary() as usize;
    let mut v_i = start as f64;
    for _ in (start..v_end).step_by(layout_context.get_step() as usize) {
        canvas_context.begin_path();
        grid.move_to(layout_context.get_render_boundary(), v_i);
        grid.line_to(
            layout_context.get_horizon_boundary() + layout_context.get_render_boundary(),
            v_i,
        );
        v_i += layout_context.get_step();
    }

    // vertical line
    let h_end = layout_context.get_horizon_boundary() as usize;
    let mut h_i = start as f64;
    for _ in (start..h_end).step_by(layout_context.get_step() as usize) {
        canvas_context.begin_path();
        grid.move_to(h_i, layout_context.get_render_boundary());
        grid.line_to(
            h_i,
            layout_context.get_vertical_boundary() + layout_context.get_render_boundary(),
        );
        h_i += layout_context.get_step();
    }
    canvas_context.stroke_with_path(&grid);
}

/// render block according to field context
pub fn render_block(layout_context: &LayoutContext, field_context: &FieldContext, block_context: &BlockContext, canvas_context: &web_sys::CanvasRenderingContext2d) {
    for (i, v) in field_context.get_list().iter().enumerate() {

        // draw controllable block
        if v == &store::statics::Number::CURRENT {
            canvas_context.set_fill_style(&JsValue::from(store::statics::BLOCKS[block_context.get_current_block_name() as usize].color));
            canvas_context.fill_rect(layout_context.get_rect_x(&i), layout_context.get_rect_y(&i), layout_context.get_rect_w(), layout_context.get_rect_h());
            canvas_context.fill();
        }

        // draw fixed block
        if v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY {
            canvas_context.set_fill_style(&JsValue::from(store::statics::BLOCKS[*v as usize].color));
            canvas_context.fill_rect(layout_context.get_rect_x(&i), layout_context.get_rect_y(&i), layout_context.get_rect_w(), layout_context.get_rect_h());
            canvas_context.fill();
        }
    }
}

pub fn clear_playing_block(layout_context: &LayoutContext, field_context: &FieldContext, canvas_context: &web_sys::CanvasRenderingContext2d) {

    for (i, v) in field_context.get_list().iter().enumerate() {

        // clear controllable block
        if v == &store::statics::Number::CURRENT {
            canvas_context.clear_rect(layout_context.get_rect_x(&i), layout_context.get_rect_y(&i), layout_context.get_rect_w(), layout_context.get_rect_h());
        }
    }
}

pub fn clear_existing_block(layout_context: &LayoutContext, field_context: &FieldContext, canvas_context: &web_sys::CanvasRenderingContext2d) {

    for (i, v) in field_context.get_list().iter().enumerate() {

        // draw controllable block
        if v != &store::statics::Number::EMPTY {
            canvas_context.clear_rect(layout_context.get_rect_x(&i), layout_context.get_rect_y(&i), layout_context.get_rect_w(), layout_context.get_rect_h());
        }
    }
}

pub fn clear_completed_block(layout_context: &LayoutContext, field_context: &FieldContext, canvas_context: &web_sys::CanvasRenderingContext2d) {
    for v in field_context.get_complete_row_numbers().iter() {

        for iter in 0..store::statics::Number::ROW {
            let i = (v * store::statics::Number::ROW + iter) as usize;

            canvas_context.clear_rect(layout_context.get_rect_x(&i), layout_context.get_rect_y(&i), layout_context.get_rect_w(), layout_context.get_rect_h());
        }

    }
}
