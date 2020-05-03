use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::func;
use crate::store;
use crate::state;
use crate::render;

use store::dynamics::render::field::Field as RenderFieldContext;

use store::dynamics::field::Field as GameFieldContext;

use store::dynamics::block::Block as BlockContext;

use render::web::context_2d::block as RenderBlock;

use state::movable as MoveFlag;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn start() {
    // initialize Game field area size
    let mut field: RenderFieldContext = Default::default();
    field.set_width(400);
    field.set_height(600);

    // get CanvasElement and initialize

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(field.get_width());
    canvas.set_height(field.get_height());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_line_width(1.0);
    context.set_stroke_style(&JsValue::from("rgba(255,255,255,0.6)"));
    context.set_fill_style(&JsValue::from("rgba(255,255,255,0.7)"));

    // outline

    let outline = web_sys::Path2d::new().unwrap();

    context.begin_path();
    outline.rect(
        field.get_render_boundary(),
        field.get_render_boundary(),
        field.get_horizon_boundary(),
        field.get_vertical_boundary(),
    );
    context.stroke_with_path(&outline);

    // grid

    let grid = web_sys::Path2d::new().unwrap();

    let start = (field.get_step() + field.get_render_boundary()) as usize;

    // horizon line
    let v_end = field.get_vertical_boundary() as usize;
    let mut v_i = start as f64;
    for _ in (start..v_end).step_by(field.get_step() as usize) {
        context.begin_path();
        grid.move_to(field.get_render_boundary(), v_i);
        grid.line_to(
            field.get_horizon_boundary() + field.get_render_boundary(),
            v_i,
        );
        v_i += field.get_step();
    }

    // vertical line
    let h_end = field.get_horizon_boundary() as usize;
    let mut h_i = start as f64;
    for _ in (start..h_end).step_by(field.get_step() as usize) {
        context.begin_path();
        grid.move_to(h_i, field.get_render_boundary());
        grid.line_to(
            h_i,
            field.get_vertical_boundary() + field.get_render_boundary(),
        );
        h_i += field.get_step();
    }
    context.stroke_with_path(&grid);




    let mut field_collection: GameFieldContext = Default::default();

    let mut block: BlockContext = Default::default();

    let mut current_block_positions = block.get_current_block_positions();

    field_collection.transfer_current_block(&current_block_positions);

    //field_collection.list[10] = 2;

    RenderBlock::render_block(&field, &field_collection, &block, &context);

    let user_input = Rc::new(Cell::new(false));
    {
        // let context = context.clone();
        let user_input = user_input.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut left = MoveFlag::left(&field_collection, &current_block_positions);
            if event.key_code() == store::statics::Number::LEFT_KEY as u32 && left {

                RenderBlock::clear_playing_block(&field, &field_collection, &context);
                field_collection.clear_current_block(&current_block_positions);
                current_block_positions = block.get_moved_current_block_positions("left").unwrap();
                block.update_current_positions(&current_block_positions);
                field_collection.transfer_current_block(&current_block_positions);
                RenderBlock::render_block(&field, &field_collection, &block, &context);
                left = MoveFlag::left(&field_collection, &current_block_positions);
                console_log!("console.log from Rust with WebAssembly {:?}", &current_block_positions);
            }

            let mut right = MoveFlag::right(&field_collection, &current_block_positions);
            if event.key_code() == store::statics::Number::RIGHT_KEY as u32 && right {

                RenderBlock::clear_playing_block(&field, &field_collection, &context);
                field_collection.clear_current_block(&current_block_positions);
                current_block_positions = block.get_moved_current_block_positions("right").unwrap();
                block.update_current_positions(&current_block_positions);
                field_collection.transfer_current_block(&current_block_positions);
                RenderBlock::render_block(&field, &field_collection, &block, &context);
                right = MoveFlag::right(&field_collection, &current_block_positions);
                console_log!("console.log from Rust with WebAssembly {:?}", &current_block_positions);
            }


        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
