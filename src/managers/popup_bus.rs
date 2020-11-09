use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::rc::Rc;
use yew::worker::*;

use super::popup;

#[derive(Serialize, Deserialize, Clone)]
pub enum InputMsg {
    NewMessage(Rc<popup::PopupMessage>),
}

#[derive(Serialize, Deserialize)]
pub enum OutputMsg {
    Notify,
}

pub struct EventBus {
    link: AgentLink<EventBus>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = InputMsg;
    type Output = popup::Msg;

    fn create(link: AgentLink<Self>) -> Self {
        EventBus {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            InputMsg::NewMessage(message) => {
                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, popup::Msg::NewMsg(message.clone()));
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
