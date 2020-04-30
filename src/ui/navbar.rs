use yew::prelude::*;
use yew::services::console::ConsoleService;

use crate::menu::MenuItem;
use super::menubar::MenuBar;

pub struct Navbar {
    link: ComponentLink<Self>,
    console: ConsoleService,
    inner: String,
    show_burger: bool,
    menu_list: Vec<MenuItem>,
    // onsignal: Option<Callback<crate::Msg>>,
}

pub enum Msg {
    BurgerClicked,
    // Clicked,
    //MenuClicked(MenuItem),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub inner: String,
    #[prop_or_default]
    pub menu_list: Vec<MenuItem>,
    #[prop_or_default]
    pub show_burger: bool,
    // pub onsignal: Option<Callback<crate::Msg>>,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            link,
            console: ConsoleService::new(),
            menu_list: props.menu_list,
            inner: props.inner,
            show_burger: props.show_burger,
            // onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Msg::Clicked => {
            //     self.console.log("Msg::Clicked");
            //     // self.onsignal.emit(());
            // }
            Msg::BurgerClicked => {
                self.console.log("Msg::BurgerClicked");
                // self.onsignal.emit(());
                self.show_burger = !self.show_burger;
                return true;
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.inner = props.inner;
        // self.onsignal = props.onsignal;
        self.show_burger = props.show_burger;
        true
    }

    fn view(&self) -> Html {
        let class = if self.show_burger {
            "is-active"
        } else {
            ""
        };

        html! {
            <nav class="navbar is-hidden-tablet is-unselectable">
                <div class="navbar-brand">
                    { self.inner.clone() }
                    // hamburger menu
                    <div class=vec!["navbar-burger", "is-hidden-tablet",  "burger", class] onclick=self.link.callback(|_| Msg::BurgerClicked)>
                        <span></span>
                        <span></span>
                        <span></span>
                    </div>
                </div> //  navbar-brand

                // desktop, tablet visible
                <div id="navMenuExample" class=vec!["navbar-menu", class]>
                    <div class="navbar-start">
                    <MenuBar menu_list=self.menu_list.clone()/>
                    // <MenuBar onsignal=self.onsignal.clone(), menu_list=self.menu_list.clone()/>
                    </div>
                </div>
            </nav>
        }
    }
}
