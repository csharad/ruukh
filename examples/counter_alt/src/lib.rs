#![feature(proc_macro_gen, proc_macro_non_items, decl_macro)]

use ruukh::{prelude::*, web_api::Event};
use wasm_bindgen::prelude::*;

#[component]
#[derive(Lifecycle)]
struct MainApp {
    #[state]
    count: i32,
}

impl Render for MainApp {
    fn render(&self) -> Markup<Self> {
        html! {
            The count is: { self.count }.
            <button @click={Self::on_click}>Increment</button>
        }
    }
}

impl MainApp {
    fn on_click(&self, _: Event) {
        self.set_state(|state| {
            state.count += 1;
        });
    }
}

#[wasm_bindgen]
pub fn run() -> ReactiveApp {
    App::<MainApp>::new().mount("app")
}
