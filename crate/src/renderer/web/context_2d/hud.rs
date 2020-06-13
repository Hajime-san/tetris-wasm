use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::store;
use crate::state;
use crate::renderer;

use store::dynamics::render::layout::LayoutContext;
use store::dynamics::field::FieldContext;
use store::dynamics::block::BlockContext;
use store::dynamics::queue::QueueContext;
use store::dynamics::level::LevelContext;
use renderer::web::context_2d::renderer as Render2d;
use state::movable as MoveFlag;
use state::complete::Complete as CheckBlockCompleteFlag;

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

    let mut layout_context: LayoutContext = Default::default();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_element = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let canvas_context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let init_layout = Render2d::init(&mut layout_context, &canvas, &canvas_context);

    let mut field_context: FieldContext = Default::default();

    let mut queue_context: QueueContext = Default::default();

    let mut block_context: BlockContext = BlockContext::new(
                                    store::statics::BLOCKS[queue_context.get_block_name() as usize].number,
                                    queue_context.get_block_name()
                                );


    let mut level_context: LevelContext = Default::default();

    let mut current_block_positions = block_context.get_current_block_positions();

    field_context.transfer_current_block(&current_block_positions);

    // field_context.list[150] = 0;
    // field_context.list[151] = 0;
    // field_context.list[152] = 0;
    // field_context.list[155] = 0;
    // field_context.list[156] = 0;
    // field_context.list[157] = 0;
    // field_context.list[158] = 0;
    // field_context.list[159] = 0;

    Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);

    field_context.create_single_rows();

    let mut complete_flag = CheckBlockCompleteFlag::Failure;

    let user_input = Rc::new(Cell::new(false));
    {
        // let context = context.clone();
        let user_input = user_input.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {

            let mut is_left = MoveFlag::left(&field_context, &current_block_positions);
            if event.key_code() == store::statics::Number::LEFT_KEY as u32 && is_left {

                Render2d::clear_playing_block(&layout_context, &field_context, &canvas_context);
                field_context.clear_current_block(&current_block_positions);
                current_block_positions = block_context.get_moved_current_block_positions(store::statics::Direction::Left);
                block_context.update_current_positions(&current_block_positions);
                field_context.transfer_current_block(&current_block_positions);
                Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);
                is_left = MoveFlag::left(&field_context, &current_block_positions);
            }

            let mut is_right = MoveFlag::right(&field_context, &current_block_positions);
            if event.key_code() == store::statics::Number::RIGHT_KEY as u32 && is_right {

                Render2d::clear_playing_block(&layout_context, &field_context, &canvas_context);
                field_context.clear_current_block(&current_block_positions);
                current_block_positions = block_context.get_moved_current_block_positions(store::statics::Direction::Right);
                block_context.update_current_positions(&current_block_positions);
                field_context.transfer_current_block(&current_block_positions);
                Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);
                is_right = MoveFlag::right(&field_context, &current_block_positions);
            }

            let mut is_down = MoveFlag::down(&field_context, &current_block_positions);
            if event.key_code() == store::statics::Number::DOWN_KEY as u32 && is_down {

                Render2d::clear_playing_block(&layout_context, &field_context, &canvas_context);
                field_context.clear_current_block(&current_block_positions);
                current_block_positions = block_context.get_moved_current_block_positions(store::statics::Direction::Down);
                block_context.update_current_positions(&current_block_positions);
                field_context.transfer_current_block(&current_block_positions);
                Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);
                is_down = MoveFlag::down(&field_context, &current_block_positions);
            }

            let is_rotate = MoveFlag::rotate(&field_context, &block_context.crate_rotate_block("simulate", true));
            if event.key_code() == store::statics::Number::UP_KEY as u32 && is_rotate {

                Render2d::clear_playing_block(&layout_context, &field_context, &canvas_context);
                field_context.clear_current_block(&current_block_positions);
                current_block_positions = block_context.crate_rotate_block("fixed", true);
                block_context.update_current_positions(&current_block_positions);
                field_context.transfer_current_block(&current_block_positions);
                Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);
            }

            field_context.create_single_rows();

            let single_rows = field_context.get_single_rows();

            complete_flag = complete_flag.check_complete(single_rows);

            if !is_down {
                // update level information
                level_context.update_count();

                match complete_flag {
                    CheckBlockCompleteFlag::Failure => {
                        block_context.update_current_positions(&current_block_positions);
                        field_context.transfer_to_fixed_number(&block_context.get_current_block_name());

                        // update game level
                        queue_context.update_queue(level_context.get_count());
                        level_context.update_score(field_context.get_complete_row_numbers());

                        // update game
                        block_context = BlockContext::new(
                            store::statics::BLOCKS[queue_context.get_block_name() as usize].number,
                            queue_context.get_block_name()
                        );
                        current_block_positions = block_context.get_current_block_positions();
                        field_context.transfer_current_block(&current_block_positions);
                        Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);
                    },
                    CheckBlockCompleteFlag::Success => {
                        field_context.delete_row();
                        Render2d::clear_completed_block(&layout_context, &field_context, &canvas_context);
                        Render2d::clear_existing_block(&layout_context, &field_context, &canvas_context);
                        field_context.drop_row();
                        field_context.transfer_to_fixed_number(&block_context.get_current_block_name());
                        Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);

                        // update game level
                        queue_context.update_queue(level_context.get_count());
                        level_context.update_completed_row(field_context.get_complete_row_numbers());
                        level_context.update_level();
                        level_context.update_speed();
                        level_context.update_score(field_context.get_complete_row_numbers());


                        // update game
                        field_context.initialize_field_values();
                        block_context = BlockContext::new(
                            store::statics::BLOCKS[queue_context.get_block_name() as usize].number,
                            queue_context.get_block_name()
                        );
                        current_block_positions = block_context.get_current_block_positions();
                        field_context.transfer_current_block(&current_block_positions);
                        Render2d::render_block(&layout_context, &field_context, &block_context, &canvas_context);

                    }
                }
            }

        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
