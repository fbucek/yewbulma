#![recursion_limit = "256"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use yewbulma::ui::table::Table;

pub struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
        let rows: Vec<Vec<_>> = vec![
            vec!["Prague".to_string(), "14000000".to_string(), "104".to_string()],
            vec!["London".to_string(), "86300000".to_string(), "106".to_string()],
            vec!["Madrid".to_string(), "66420000".to_string(), "106".to_string()],
        ];

        html! {
            <div>
                <Table headers=headers rows=rows />
            </div>
        }
    }
}
