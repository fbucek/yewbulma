use yew::prelude::*;
use yew::services::console::ConsoleService;

use std::iter::Iterator;


pub struct Table {
    link: ComponentLink<Self>,
    props: Props,
    sort_column: Option<SortColumn>,
}

pub enum Msg {
    SortClicked(usize),
}


pub struct SortColumn {
    pub index: usize,
    pub sort_order: SortOrder,
}

impl SortColumn {
    pub fn new(index: usize) -> Self {
        SortColumn {
            index,
            sort_order: SortOrder::Ascending,
        }
    }

    pub fn invert(&mut self) {
        log::info!("inverting value");
        self.sort_order = match self.sort_order {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        }
    }
}
#[derive(Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl From<bool> for SortOrder {
    fn from(sort: bool) -> Self {
        match sort {
            true => SortOrder::Ascending,
            false => SortOrder::Descending,
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    #[prop_or_default]
    pub sort_column: Option<SortOrder>
}

impl Component for Table {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Table {
            link,
            props,
            sort_column: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SortClicked(index) => {
                log::info!("clicked on index: {}", index);
                if let Some(sort_column) = &mut self.sort_column {
                    if sort_column.index == index {
                        sort_column.invert();
                    } else {
                        self.sort_column = Some(SortColumn::new(index));
                    }
                } else {
                    self.sort_column = Some(SortColumn::new(index));
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        self.table_from_vecs(&self.props.headers, &self.props.rows)
    }
}
impl Table {
    fn render_header(&self, headers: &Vec<String>) -> Html {
        html! {
            <thead>
            <tr>
            { headers.iter()
                .enumerate()
                .map(|(index, header_value)| 
                    html! {
                        <>
                            // <i class="fa fa-chevron-up" aria-hidden="true"></i>
                            <th 
                                onclick=self.link.callback(move |_| Msg::SortClicked(index))
                                ondoubleclick=self.link.callback(move |_| Msg::SortClicked(index))
                            >
                            { 
                                    if let Some(column_sort) = &self.sort_column {
                                        if column_sort.index == index {
                                            if column_sort.sort_order == SortOrder::Ascending {
                                                html! { <i class="fa fa-sort-asc"></i> }
                                            } else {
                                                html! { <i class="fa fa-sort-desc"></i> }
                                            }
                                        } else {
                                            html! {}
                                        }
                                    } else {
                                        html! {}
                                    }
                            }  
                            { header_value }
                            </th>
                        </>
                    }
            ).collect::<Html>()
            }
            </tr>
            </thead>
        }
    }

    fn render_rows(&self, rows: &Vec<Vec<String>>) -> Html {
        // TODO: add sort_by -> https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by
        rows.iter()
            .map(|row| {
                html! {
                    <tr> // 
                    { 
                        row.iter()
                            .enumerate()
                            .map(|(index, column)| {
                                let callback = self.link.callback(move |e| Msg::SortClicked(index));
                                html! {
                                    // Render Column
                                    <td onclick=callback>
                                    { column }
                                    </td>
                                }
                            }).collect::<Html>()
                    }
                    </tr>
                } // html!
            } // map
        ).collect::<Html>()
    }

    fn render_column(&self, index: usize, column: &String) -> Html {
        html! {
            // Render Column
            <td onclick=self.link.callback(move |_| Msg::SortClicked(index))>
            { column }
            </td>
        }
    }

    fn table_from_vecs(&self, header: &Vec<String>, data: &Vec<Vec<String>>) -> Html {
        html! {
            <table id="coivd" class="table table-sm table-responsive-sm">
                { self.render_header(header) }
                <tbody>
                    { self.render_rows(data) }
                </tbody>
            </table>
        }
    }
}

fn table_from_vecs(header: &Vec<String>, data: &Vec<Vec<String>>) -> Html {
    html! {
        <table id="coivd" class="table table-sm table-responsive-sm">
            <thead>
                // Process headers
                <tr>
                { 
                    header.iter()
                        .map(|header_value| html! {
                            <th>{ header_value }</th>
                        }).collect::<Html>()
                }
                </tr>
            </thead>
            <tbody>
                { data.iter()
                    .map(|row| {
                        html! {
                            <tr> // 
                            { 
                                row.iter()
                                    .enumerate()
                                    .map(|(index, column)| {
                                        html! {
                                            // Render Column
                                            <td>
                                            { column }
                                            </td>
                                        }
                                    }).collect::<Html>()
                            }
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}

/// How to test
/// 
/// `wasm-pack test --chrome --headless`
/// 
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_table() {
        
    }
}
