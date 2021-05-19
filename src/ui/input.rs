use yew::prelude::*;

pub struct TextInput {
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

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextInput {
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
            <input class="input new-client firstname"
            placeholder=self.placeholder.clone()
            value=self.value.clone()
            disabled=self.disabled
            oninput=self.link.callback(Msg::InputChange) />

            // <button class="button is-small" disabled=self.disabled oninput=self.link.callback(|_| Msg::InputChange)>
            //     { &self.value }
            //     { &self.value }
            // </button>
        }
    }
}
