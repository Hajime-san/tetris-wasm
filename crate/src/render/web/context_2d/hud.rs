use math::round;
use std::f64;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::func;
use crate::store;
use store::dynamics::render::element::*;
use store::dynamics::render::field as Field;

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
    let mut canvas_size: Canvas = Default::default();
    canvas_size.set_width(400);
    canvas_size.set_height(600);

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(canvas_size.get_width());
    canvas.set_height(canvas_size.get_height());

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_line_width(1.0);
    context.set_stroke_style(&JsValue::from("rgba(255,255,255,0.6)"));
    context.set_fill_style(&JsValue::from("rgba(255,255,255,0.7)"));

    let field: Field::Field = Default::default();

    // outline

    let outline = web_sys::Path2d::new().unwrap();

    context.begin_path();
    outline.rect(
        field.standard,
        field.standard,
        field.horizon,
        field.vertical,
    );
    context.stroke_with_path(&outline);

    // grid

    let grid = web_sys::Path2d::new().unwrap();

    let start = (field.step + field.standard) as usize;

    // horizon line
    let v_end = field.vertical as usize;
    let mut v_i = start as f64;
    for _ in (start..v_end).step_by(field.step as usize) {
        context.begin_path();
        grid.move_to(field.standard, v_i);
        grid.line_to(field.horizon + field.standard, v_i);
        v_i += field.step;
    }

    // vertical line
    let h_end = field.horizon as usize;
    let mut h_i = start as f64;
    for _ in (start..h_end).step_by(field.step as usize) {
        context.begin_path();
        grid.move_to(h_i, field.standard);
        grid.line_to(h_i, field.vertical + field.standard);
        h_i += field.step;
    }
    context.stroke_with_path(&grid);

    let mut field_array: store::dynamics::field::Field = Default::default();

    field_array.numbers[4] = 99;
    field_array.numbers[5] = 99;
    field_array.numbers[13] = 99;
    field_array.numbers[14] = 99;

    for (i, v) in field_array.numbers.iter().enumerate() {
        if v == &store::statics::Number::CURRENT {
            context.set_fill_style(&JsValue::from("rgba(255,0,255,1)"));
            context.fill_rect(
                (((func::fix_digit(i as i32) * field.step as i32) as f64) + (field.standard + 1.0)),
                (round::floor((i as i32 / store::statics::Number::ROW).into(), 0) as f64)
                    * field.step
                    + (field.standard + 1.0),
                field.step - 2.0,
                field.step - 2.0,
            );
            context.fill();
        }
    }
}
