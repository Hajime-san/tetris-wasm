use math::round;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::func;
use crate::store;

use store::dynamics::render::field::Field as GameField;
use store::dynamics::render::field::Get as GetGameFieldData;
use store::dynamics::render::field::Update as UpdateGameFieldData;

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
    let mut field: GameField = Default::default();
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

    let mut field_array: store::dynamics::field::Field = Default::default();

    field_array.numbers[4] = 99;
    field_array.numbers[5] = 99;
    field_array.numbers[13] = 99;
    field_array.numbers[14] = 99;

    for (i, v) in field_array.numbers.iter().enumerate() {
        if v == &store::statics::Number::CURRENT {
            context.set_fill_style(&JsValue::from("rgba(255,0,255,0.6)"));
            let rect = field.set_fill_rect(&i);
            context.fill_rect(rect.x, rect.y, rect.w, rect.h);
            context.fill();
        }
    }
}
