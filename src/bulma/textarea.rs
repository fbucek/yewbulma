use yew::prelude::*;

pub struct TextArea {
    link: ComponentLink<Self>,
    value: String,
    placeholder: String,
    disabled: bool,
    oninput: Callback<InputData>,
}

pub enum Msg {
    InputChange(InputData),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub disabled: bool,
    pub oninput: Callback<InputData>,
}

impl Component for TextArea {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextArea {
            link,
            value: props.value,
            placeholder: props.placeholder,
            disabled: props.disabled,
            oninput: props.oninput,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputChange(data) => {
                self.oninput.emit(data);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        self.placeholder = props.placeholder;
        self.oninput = props.oninput;
        self.disabled = props.disabled;
        true
    }

    fn view(&self) -> Html {
        html! {
            <textarea class=("textarea", "new-client", "description")
            placeholder=&self.placeholder
            value=&self.value
            oninput=self.link.callback(|e: InputData| Msg::InputChange(e)) />
        }
    }
}
