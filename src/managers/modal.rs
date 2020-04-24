use yew::prelude::*;
use std::rc::Rc;

pub struct ModalManager {
}

pub struct MoadlComponent {
    link: ComponentLink<Self>,
    title: String,
    pub show_dialog: bool,
    body: String,
    ok_text: String,
    onclick: Callback<()>,
    modal_manager: Rc<ModalManager>,
}
pub enum Msg {
    Confirmed,
    Cancel,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub ok_text: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub body: String,
    #[prop_or_default]
    pub show_dialog: bool,
    modal_manager: Rc<ModalManager>,
}

impl Component for MoadlComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MoadlComponent {
            link,
            title: props.title,
            show_dialog: props.show_dialog,
            body: props.body,
            ok_text: props.ok_text,
            // onclick: props.onclick,
            onclick: Callback::noop(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Confirmed => {
                self.onclick.emit(());
            }
            Msg::Cancel => {
                self.show_dialog = false;
                return true;
            }
        }
        false // wont redraw
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.show_dialog = props.show_dialog;
        true
    }

    fn view(&self) -> Html {
        let cancel = self.link.callback(|_| Msg::Cancel);
        let confirm = self.link.callback(|_| Msg::Confirmed);
        html! {
            <div class=if self.show_dialog { "modal is-active" } else { "modal " }>
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                    <p class="modal-card-title"> { &self.title } </p>
                    <button class="delete" aria-label="close" onclick=&cancel />
                    </header>
                    <section class="modal-card-body">
                    { &self.body }
                    </section>
                    <footer class="modal-card-foot">
        
                    <button class="button is-success" onclick=confirm>{ &self.ok_text }</button>
                    <button class="button" onclick=&cancel> { "Cancel" }</button>
                    </footer>
                </div>
            </div>
        }
    }
}
