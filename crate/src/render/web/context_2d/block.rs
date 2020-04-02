use wasm_bindgen::prelude::*;

use std::iter::Iterator;

use crate::store;

use store::dynamics::render::field::Field as GameField;
use store::dynamics::render::field::Update as UpdateGameFieldData;
use store::dynamics::field::Field as FieldCollection;

pub fn render_block(field: &GameField, field_collection: &FieldCollection, block_type: &store::statics::BlockName, context: &web_sys::CanvasRenderingContext2d) {
    for (i, v) in field_collection.numbers.iter().enumerate() {

        // draw controllable block
        if v == &store::statics::Number::CURRENT {
            context.set_fill_style(&JsValue::from(store::statics::BLOCKS[block_type.unwrap_invalid() as usize].color));
            let rect = field.set_fill_rect(&i);
            context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            context.fill();
        }

        // draw fixed block
        if v != &store::statics::Number::CURRENT && v != &store::statics::Number::EMPTY {
            context.set_fill_style(&JsValue::from(store::statics::BLOCKS[*v as usize].color));
            let rect = field.set_fill_rect(&i);
            context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            context.fill();
        }
    }
}
