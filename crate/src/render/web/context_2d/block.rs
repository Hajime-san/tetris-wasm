use wasm_bindgen::prelude::*;

use std::iter::Iterator;

use crate::store;

use store::dynamics::render::field::{ Field as RenderField, Get as _, Update as _ };

use store::dynamics::field::{ Field as GameField, Get as _ };

use store::dynamics::delete::Delete as DeleteField;

pub fn render_block(field: &RenderField, field_collection: &GameField, block_type: &store::statics::BlockName, context: &web_sys::CanvasRenderingContext2d) {
    for (i, v) in field_collection.get_numbers().iter().enumerate() {

        // draw controllable block
        if v == &store::statics::Number::CURRENT {
            context.set_fill_style(&JsValue::from(store::statics::BLOCKS[block_type.unwrap_invalid() as usize].color));
            let rect = field.get_fill_rect(&i);
            context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            context.fill();
        }

        // draw fixed block
        if v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY {
            context.set_fill_style(&JsValue::from(store::statics::BLOCKS[*v as usize].color));
            let rect = field.get_fill_rect(&i);
            context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            context.fill();
        }
    }
}

pub fn clear_playing_block(field: &RenderField, field_collection: &GameField, context: &web_sys::CanvasRenderingContext2d) {

    for (i, v) in field_collection.get_numbers().iter().enumerate() {

        // draw controllable block
        if v == &store::statics::Number::CURRENT {
            let rect = field.get_fill_rect(&i);
            context.clear_rect(rect.x, rect.y, rect.w, rect.h);
        }
    }
}

pub fn delete_completed_block(field: &RenderField, rows: &DeleteField, context: &web_sys::CanvasRenderingContext2d) {
    for v in rows.complete_row_numbers.iter() {

        for iter in 0..store::statics::Number::ROW {
            let i = (v * store::statics::Number::ROW + iter) as usize;

            let rect = field.get_fill_rect(&i);
            context.clear_rect(rect.x, rect.y, rect.w, rect.h);
        }

    }
}
