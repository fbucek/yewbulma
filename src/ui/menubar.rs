use yew::prelude::*;
use yew::Html;
use yew_router::prelude::*;

use crate::menu::MenuItem;

pub struct MenuBar {
    link: ComponentLink<Self>,
    menu_list: Vec<MenuItem>,
}

pub enum Msg {
    ItemClicked(MenuItem),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub menu_list: Vec<MenuItem>,
    // #[prop_or_default]
}

impl MenuBar {
    fn separator(&self, item: MenuItem) -> Html {
        html! {
            <p class="menu-label"> { item.name.clone() } </p>
        }
    }
    fn children(&self, item: MenuItem) -> Html {
        // let url = self.url_href(item.clone());
        let mut fa_class = "fa ".to_string();
        if let Some(icon) = &item.icon {
            fa_class.push_str(&icon.to_string());
        }
        // Add class for expanded
        let collapsible_class = if item.expanded {
            "submenu-icon ml-auto menu-opened"
        } else {
            "submenu-icon ml-auto menu-closed"
        };

        html! {
            <>
                <a onclick=self.link.callback({let item = item.clone(); move |_| Msg::ItemClicked(item.clone())}) > //  @click="item.expanded = !item.expanded">
                    // <span class="icon"><i class=fa_class></i></span>
                    <span>{ item.name.clone() }</span>
                    <span class=collapsible_class /> //"submenu-icon ml-auto" :class="item.expanded ? 'menu-closed' : 'menu-opened'"></span>
                </a>
                {
                    if item.expanded {
                        html!{ <ul class="submenu"> {
                            item.children.iter().map(|subitem|  {
                                self.menuitem(subitem.clone(), true)
                            }).collect::<Html>()
                        }</ul> }
                    } else {
                        html!{}
                    }
                }
            </>
        }
    }

    fn menuitem(&self, item: MenuItem, _is_subitem: bool) -> Html {
        let url = self.url_href(item.clone());
        let mut class = "fa ".to_string();
        if let Some(icon) = &item.icon {
            class.push_str(&icon.to_string());
        }

        let item_html = html! { <span>{ item.name.clone() }</span> };

        // let item_html = if is_subitem {
        //     html! {
        //         <>
        //             <span>{ item.name.clone() }</span>
        //         </>
        //     }
        // } else {
        //     html! {
        //         <>
        //             // <span class="icon"><i class=class></i></span>
        //             <span>{ item.name.clone() }</span>
        //         </>
        //     }
        // };

        if url.starts_with("http") {
            html! {
                <a href=url class="">
                { item_html }
                </a>
            }
        } else {
            html! {
                <RouterAnchor<String> route=url>
                { item_html }
                </RouterAnchor<String>>
            }
        }
    }

    fn url_href(&self, item: MenuItem) -> String {
        match &item.link {
            Some(link) => link.to_string(),
            None => "".to_string(),
        }
    }
}

impl Component for MenuBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        MenuBar {
            link,
            menu_list: props.menu_list,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        // fn mounted(&mut self) -> ShouldRender {
        // Handle colapse
        // -> when url match .link -> item.expanded = false/
        let path = yew::utils::window()
            .location()
            .pathname()
            .unwrap_or_else(|_| "".to_string());
        log::trace!("checking expanding: path is  {}", &path);

        // If any children has matching path -> expand
        for item in &mut self.menu_list {
            for subitem in &item.children {
                log::trace!("checking expanding: subitem is  {}", &subitem.name);

                if let Some(link) = &subitem.link {
                    log::trace!("link is  {}, path is{}", &link, &path);
                    if path == *link {
                        log::trace!("Expanding item: {}", &link);
                        item.expanded = true;
                    } else {
                    }
                }
            }
        }

        log::trace!("not mounted is called");
        // true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ItemClicked(menuitem) => {
                // log::trace!("Now handling: {}", menuitem.name);
                // expand / collapse when has children
                if !menuitem.children.is_empty() {
                    for item in &mut self.menu_list.iter_mut() {
                        if menuitem == *item {
                            // invert bool
                            item.expanded = !item.expanded;
                            //log::trace!("found item: {}", menuitem.name);
                            return true;
                        }
                    }
                } else {
                    // // emit message to parent
                    // log::trace!("item clicked: {}", menuitem.name);
                    // if let Some(callback) = &self.onsignal {
                    //     callback.emit(crate::Msg::MenuClicked(menuitem));
                    // }
                }
                true
            }
        }
    }

    // fn change(&mut self, props: Self::Properties) -> ShouldRender {
    //     self.onsignal = props.onsignal;
    //     true
    // }

    fn view(&self) -> Html {
        html! {
            <ul class="is-unselectable">
                // { for item in &self.menu_list {
                { self.menu_list.iter().map(|item| {
                    html! {
                        <li class="menu-list">
                            {
                                if item.separator {
                                    self.separator(item.clone())
                                } else if ! item.children.is_empty() {
                                    self.children(item.clone())
                                } else {
                                    self.menuitem(item.clone(), false)
                                }
                            }
                        </li>
                    }
                }).collect::<Html>()
                }
            </ul>
        }
    }
}
