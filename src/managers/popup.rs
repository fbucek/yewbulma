use yew::prelude::*;
use std::rc::Rc;

pub struct PopupMessage {
    header: String,
    body: String,
    class_type: String,
}

impl PopupMessage {
    pub fn new_rc(header: String, body: String, class_type: String) -> Rc<Self> {
        Rc::new(PopupMessage { header, body, class_type })
    }
}

pub struct PopupManager {
    // link: ComponentLink<Self>,
    messages: Vec<Rc<PopupMessage>>,
}

impl PopupManager {
    pub fn new() -> Self {

        PopupManager {
            messages: Vec::<Rc<PopupMessage>>::new(),
        }
    }
    
    pub fn add_toast(&mut self, toast: Rc<PopupMessage>) {
        self.messages.push(toast);
    }

    pub fn remove_first(&mut self) {
        self.messages.remove(0);
    }

    pub fn view(&self) -> Html {
        html! {
            <div name="toasts" tag="div" class="c-toasts">
            { self.view_children() }
            //<toast class="toasts-item" v-for="toast in toasts" :toast="toast" :key="toast.id"></toast>
            </div>
        }
    }

    fn view_children(&self) -> Html {
        self.messages.iter().map(|toast| {
            html! {
                <div class=vec!["c-toasts__item", toast.class_type.as_str()]>
                    <span class="c-toasts__item-header">{ toast.header.clone() }</span>
                    { 
                        if ! toast.body.is_empty() {
                            html! { <span class="c-toasts__item-body">{ toast.body.clone() }</span> }
                        } else {
                            html! {}
                        }
                    }
                </div>
            }
        }).collect::<Html>()
    }
}


pub enum Msg {
    TimeoutEvent,
}
