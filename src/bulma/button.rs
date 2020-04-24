use yew::prelude::*;

pub struct PushButton {
    link: ComponentLink<Self>,
    title: String,
    disabled: bool,
    onclick: Callback<()>,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<()>,
    pub disabled: bool,
}

impl Component for PushButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PushButton {
            link,
            title: String::new(),
            disabled: props.disabled,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.onclick.emit(());
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onclick = props.onclick;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button class="button is-small" disabled=self.disabled onclick=self.link.callback(|_| Msg::Clicked)>
                { &self.title }
            </button>
        }
    }
}
