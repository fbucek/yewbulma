use yew::prelude::*;
use yew::services::Task;

use yew::agent::Dispatched;
use yew::agent::Dispatcher; // needed for ::dispatcher()

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

use std::time::Duration;

use super::popup_bus;

#[derive(Serialize, Deserialize)]
pub struct PopupMessage {
    header: String,
    body: String,
    class_type: String,
}

impl PopupMessage {
    pub fn new_rc(header: String, body: String, class_type: String) -> Rc<Self> {
        Rc::new(PopupMessage {
            header,
            body,
            class_type,
        })
    }
}

pub struct PopupManager {
    pub event_bus: Rc<RefCell<Dispatcher<popup_bus::EventBus>>>, //
}

impl PopupManager {
    pub fn new() -> Self {
        PopupManager {
            event_bus: Rc::new(RefCell::new(popup_bus::EventBus::dispatcher())),
        }
    }

    pub fn info<T: Into<String>>(&self, header: T, body: Option<String>) {
        let popup_message = PopupMessage::new_rc(
            header.into(),
            body.unwrap_or("".to_string()),
            "has-background-info".into(),
        );
        let mut bus = self.event_bus.borrow_mut();
        bus.send(popup_bus::InputMsg::NewMessage(popup_message.clone()));
    }    

    pub fn warn<T: Into<String>>(&self, header: T, body: Option<String>) {
        let popup_message = PopupMessage::new_rc(
            header.into(),
            body.unwrap_or("".to_string()),
            "has-background-warning".into(),
        );
        let mut bus = self.event_bus.borrow_mut();
        bus.send(popup_bus::InputMsg::NewMessage(popup_message.clone()));
    }

    pub fn error<T: Into<String>>(&self, header: T, body: Option<String>) {
        let popup_message = PopupMessage::new_rc(
            header.into(),
            body.unwrap_or("".to_string()),
            "has-background-danger".into(),
        );
        let mut bus = self.event_bus.borrow_mut();
        bus.send(popup_bus::InputMsg::NewMessage(popup_message.clone()));
    }
}

/// Popupmanager is connected using Agent link
pub struct PopupManagerUI {
    // Data
    messages: Vec<Rc<PopupMessage>>,
    timer_tasks: Vec<Box<dyn Task>>,
    // Communication
    link: ComponentLink<Self>,
    _context: Box<dyn Bridge<popup_bus::EventBus>>,
}

impl PopupManagerUI {
    pub fn add_toast(&mut self, toast: Rc<PopupMessage>) {
        self.messages.push(toast);
    }

    pub fn remove_first(&mut self) {
        self.messages.remove(0);
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

#[derive(Serialize, Deserialize)]
pub enum Msg {
    NewMsg(Rc<PopupMessage>),
    TimerDone,
}

impl Component for PopupManagerUI {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|message| message);
        let _context = popup_bus::EventBus::bridge(callback);

        PopupManagerUI {
            link,
            messages: Vec::<Rc<PopupMessage>>::new(),
            timer_tasks: Vec::new(),
            _context,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewMsg(message) => {
                self.add_toast(message);
                // log::info!("New Popup messages XXXX");
                let task = Box::new(yew::services::TimeoutService::spawn(
                    Duration::from_secs(10),
                    self.link.callback(|_| Msg::TimerDone),
                ));
                self.timer_tasks.push(task);
            }
            Msg::TimerDone => {
                self.timer_tasks.remove(0);
                self.remove_first();
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div name="toasts" tag="div" class="c-toasts">
            { self.view_children() }
            //<toast class="toasts-item" v-for="toast in toasts" :toast="toast" :key="toast.id"></toast>
            </div>
        }
    }
}
