use std::iter::Iterator;
use std::ops::Not;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

const ANIM_INTERVAL_MS: i32 = 2;
const MAX_AMPLITUDE: i32 = 48;

static mut SPANS: Vec<HtmlElement> = vec![];
static mut ANGD: usize = 0;
const INIT_FRAME: Vec<FrameAttr> = Vec::new();
static mut FRAMES: [Vec<FrameAttr>; 360] = [INIT_FRAME; 360];
static mut IS_PLAYING: bool = false;
static mut DONT_PANIC: bool = false;

#[wasm_bindgen(start)]
pub fn main() {
    let document = window().unwrap().document().unwrap();
    let banner = document
        .get_element_by_id("banner")
        .expect("banner should exist")
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .to_owned();
    let onclick = Closure::wrap(Box::new(|| unsafe {
        if DONT_PANIC {
            reset_banner("ekezet.com")
        } else {
            reset_banner("don't panic!")
        }
        DONT_PANIC = DONT_PANIC.not();
    }) as Box<dyn FnMut()>);

    banner.set_onclick(Some(onclick.as_ref().unchecked_ref()));

    onclick.forget();

    unsafe { reset_banner("ekezet.com") }
}

unsafe fn reset_banner<T: ToString>(text: T) {
    IS_PLAYING = false;
    __reset_banner(text);
    IS_PLAYING = true;
}

#[inline]
unsafe fn __reset_banner<T: ToString>(text: T) {
    let text = text.to_string();
    let num_chars = text.chars().count();
    let step = 360.0 / num_chars as f32;
    for deg in 0..360 {
        FRAMES[deg].clear();
        let rad = (deg as f32).to_radians();
        for index in 0..num_chars {
            let phase: f32 = (index as f32) * step;
            let offset: f32 = MAX_AMPLITUDE as f32 * (rad + phase.to_radians()).sin();
            let r = 245.0 * (rad + phase).cos() + 140.0;
            let g = 245.0 * (rad + phase * 1.5).cos() + 140.0;
            let b = 245.0 * (rad + phase * 2.0).cos() + 140.0;
            let offset = format!("{}px", offset);
            let color = format!(
                "rgb({}, {}, {})",
                r.round() as usize,
                g.round() as usize,
                b.round() as usize,
            );
            FRAMES[deg].push(FrameAttr { offset, color })
        }
    }

    let document = window().unwrap().document().unwrap();
    let banner = document
        .get_element_by_id("banner")
        .expect("banner should exist");
    SPANS = replace_with_spans(text, document, banner);
}

fn replace_with_spans(orig_text: String, document: Document, banner: Element) -> Vec<HtmlElement> {
    let spans: Vec<HtmlElement> = orig_text
        .chars()
        .map(|char| {
            let el = document.create_element("span").unwrap();
            if char.is_whitespace() {
                el.set_inner_html("&nbsp;");
            } else {
                el.set_inner_html(&char.to_string());
            }
            el.dyn_ref::<HtmlElement>().unwrap().to_owned()
        })
        .collect();

    banner.set_inner_html("");
    for span in spans.iter() {
        banner.append_child(span).unwrap();
    }

    spans
}

struct FrameAttr {
    offset: String,
    color: String,
}

#[wasm_bindgen]
pub struct AnimHandle {
    interval_id: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for AnimHandle {
    fn drop(&mut self) {
        let window = window().unwrap();
        window.clear_interval_with_handle(self.interval_id);
    }
}

#[wasm_bindgen]
pub fn run() -> Result<AnimHandle, JsValue> {
    let cb = Closure::wrap(Box::new(|| unsafe { update_frame() }) as Box<dyn FnMut()>);
    let interval_id = window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            ANIM_INTERVAL_MS,
        )?;

    Ok(AnimHandle {
        interval_id,
        _closure: cb,
    })
}

#[inline]
unsafe fn update_frame() {
    if !IS_PLAYING {
        return;
    }

    let spans = &SPANS;
    let frames = &FRAMES[ANGD];
    for (index, span) in spans.iter().enumerate() {
        if !IS_PLAYING {
            return;
        }
        let frame = frames.get(index).unwrap();
        span.style()
            .set_property("top", &frame.offset)
            .expect("should set the y position");
        span.style()
            .set_property("color", &frame.color)
            .expect("should set the color");
    }

    ANGD += 1;
    if 360 == ANGD {
        ANGD = 0;
    }
}
