use wasm_bindgen::prelude::*;

use crate::game::{EVENTS, Gloam};

#[wasm_bindgen(typescript_custom_section)]
const SCRIPT: &'static str = r#"
    export interface EventSub {
        trigger(): void;
    }
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "EventSub")]
    pub type EventSub;

    #[wasm_bindgen(structural, method)]
    pub fn trigger(this: &EventSub);
}

#[wasm_bindgen]
impl Gloam {
    pub fn register(event: &str, sub: EventSub) {
        EVENTS.with(|events| {
            events.borrow_mut().entry(event.to_string()).or_insert_with(|| vec![]).push(sub);
        })
    }

    pub fn trigger(event: &str) {
        EVENTS.with(|events| {
            if let Some(event) = events.borrow().get(event) {
                for sub in event {
                    sub.trigger();
                }
            }
        });
    }
}