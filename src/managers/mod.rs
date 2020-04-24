use std::rc::Rc;
use std::cell::RefCell;

use yew::prelude::*;

pub mod net;
pub mod popup;

use net::RequestManager;
use popup::{PopupMessage, PopupManager};

pub struct Manager {
    pub popup_manager: Rc<RefCell<PopupManager>>,
    pub request_manager: Rc<RefCell<RequestManager>>,
    pub notify_callback: Callback<()>,
    
}

impl Manager {
    pub fn new(callback: Callback<()>) -> Self {
        Manager { 
            popup_manager: Rc::new(RefCell::new(PopupManager::new())),
            request_manager: Rc::new(RefCell::new(RequestManager::new())),
            notify_callback: callback,
        }
    }

    pub fn update_popups(&self) {
        let mut tm = self.popup_manager.borrow_mut();
        tm.remove_first();
    }

    pub fn info(&self, header: &str, body: Option<String>) {
        let popup_message = PopupMessage::new_rc(
            header.to_string(),
            body.unwrap_or("".to_string()),
            "has-background-info".into()
        );
        self.notify_callback.emit(());
        let mut tm = self.popup_manager.borrow_mut();
        tm.add_toast(popup_message);
    }
}
