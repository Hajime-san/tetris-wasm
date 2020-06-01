use wasm_bindgen::prelude::*;

use std::iter::Iterator;

use crate::store;
use store::dynamics::render::field::Field as RenderFieldContext;
use store::dynamics::field::Field as GameFieldContext;
use store::dynamics::block::Block as BlockContext;

pub fn render_block(field: &RenderFieldContext, field_collection: &GameFieldContext, block_type: &BlockContext, context: &web_sys::CanvasRenderingContext2d) {
    for (i, v) in field_collection.get_list().iter().enumerate() {

        // draw controllable block
        if v == &store::statics::Number::CURRENT {
            context.set_fill_style(&JsValue::from(store::statics::BLOCKS[block_type.get_current_block_name() as usize].color));
            context.fill_rect(field.get_rect_x(&i), field.get_rect_y(&i), field.get_rect_w(), field.get_rect_h());
            context.fill();
        }

        // draw fixed block
        if v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY {
            context.set_fill_style(&JsValue::from(store::statics::BLOCKS[*v as usize].color));
            context.fill_rect(field.get_rect_x(&i), field.get_rect_y(&i), field.get_rect_w(), field.get_rect_h());
            context.fill();
        }
    }
}

pub fn clear_playing_block(field: &RenderFieldContext, field_collection: &GameFieldContext, context: &web_sys::CanvasRenderingContext2d) {

    for (i, v) in field_collection.get_list().iter().enumerate() {

        // clear controllable block
        if v == &store::statics::Number::CURRENT {
            context.clear_rect(field.get_rect_x(&i), field.get_rect_y(&i), field.get_rect_w(), field.get_rect_h());
        }
    }
}

pub fn clear_existing_block(field: &RenderFieldContext, field_collection: &GameFieldContext, context: &web_sys::CanvasRenderingContext2d) {

    for (i, v) in field_collection.get_list().iter().enumerate() {

        // draw controllable block
        if v != &store::statics::Number::EMPTY {
            context.clear_rect(field.get_rect_x(&i), field.get_rect_y(&i), field.get_rect_w(), field.get_rect_h());
        }
    }
}

pub fn clear_completed_block(field: &RenderFieldContext, field_collection: &GameFieldContext, context: &web_sys::CanvasRenderingContext2d) {
    for v in field_collection.get_complete_row_numbers().iter() {

        for iter in 0..store::statics::Number::ROW {
            let i = (v * store::statics::Number::ROW + iter) as usize;

            context.clear_rect(field.get_rect_x(&i), field.get_rect_y(&i), field.get_rect_w(), field.get_rect_h());
        }

    }
}
