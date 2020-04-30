#![recursion_limit = "256"]
use wasm_bindgen::prelude::*; 
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use yewbulma::ui::table::Table;

pub struct Model {
    //link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model {
            // link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let headers = vec![
            "City".to_string(), 
            "Population".to_string(),
            "Rating".to_string(),
        ];
        let rows: Vec<Vec<serde_json::Value>> = vec![
            vec!["Prague".into(), 14000000.into(), 104.into()],
            vec!["London".into(), 86300000.into(), 106.into()],
            vec!["Madrid".into(), 66420000.into(), 106.into()],
        ];

        html! {
            <div>
                <Table headers=headers rows=rows />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    if console_log::init_with_level(log::Level::Trace).is_err() {
        log::error!("Not possible to init logger / message irrelevant -> log not working");
    }
    yew::start_app::<Model>();
}
