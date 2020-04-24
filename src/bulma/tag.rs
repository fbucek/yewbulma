use yew::prelude::*;

pub struct TagItem {
    props: Props,
    // title: String,
    // disabled: bool,
    // onclick: Callback<()>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub prefix: String,
    pub href: String,
    #[prop_or_default]
    pub class: Option<String>,
}

impl Component for TagItem {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TagItem {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }


    fn view(&self) -> Html {
        let class = match &self.props.class {
            Some(class) => format!("tag {}", class),
            None => "tag is_primary".to_string(),
        };

        html! {
            <div class="control">
                <div class="tags has-addons">
                    <span class=class> { self.props.prefix.clone() } </span>
                    <span class="tag">
                        <a target="_blank" rel="noopener noreferrer" href=self.props.href.clone() > { self.props.value.clone() } </a>
                    </span>
                </div>
            </div>
        }
    }
}



