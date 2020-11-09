use yew::prelude::*;

pub struct Table {
    link: ComponentLink<Self>,
    props: Props,
    sort_column: Option<SortColumn>,
}

pub struct TableData {
    pub headers: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
}

impl TableData {
    pub fn new(headers: Vec<String>, data: Vec<Vec<serde_json::Value>>) -> Self {
        TableData { headers, data }
    }
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
    pub rows: Vec<Vec<serde_json::Value>>,
    #[prop_or_default]
    pub sort_column: Option<SortOrder>,
}

impl Component for Table {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Table {
            link,
            props,
            sort_column: None,
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
                                ondblclick=self.link.callback(move |_| Msg::SortClicked(index))
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

    fn render_rows(&self, rows: &Vec<Vec<serde_json::Value>>) -> Html {
        let mut rows = rows.to_vec();
        if let Some(sort_column) = &self.sort_column {
            log::info!("sorting vec rows");
            rows.sort_by(|a, b| {
                let index = sort_column.index;
                let a: &serde_json::Value = &a[index];
                let b: &serde_json::Value = &b[index];
                if let (Some(a), Some(b)) = (a.as_f64(), b.as_f64()) {
                    if sort_column.sort_order == SortOrder::Ascending {
                        a.partial_cmp(&b).unwrap()
                    } else {
                        b.partial_cmp(&a).unwrap()
                    }
                } else if let (Some(a), Some(b)) = (a.as_i64(), b.as_i64()) {
                    if sort_column.sort_order == SortOrder::Ascending {
                        a.partial_cmp(&b).unwrap()
                    } else {
                        b.partial_cmp(&a).unwrap()
                    }
                } else if let (Some(a), Some(b)) = (a.as_str(), b.as_str()) {
                    if sort_column.sort_order == SortOrder::Ascending {
                        a.partial_cmp(b).unwrap()
                    } else {
                        b.partial_cmp(a).unwrap()
                    }
                } else {
                    std::cmp::Ordering::Greater
                }
            })
        }
        rows.iter()
            .map(
                |row| {
                    html! {
                        <tr> //
                        {
                            row.iter()
                                .enumerate()
                                .map(|(index, column)| {
                                    self.render_column(index, column)
                                }).collect::<Html>()
                        }
                        </tr>
                    } // html!
                }, // map
            )
            .collect::<Html>()
    }

    fn render_column(&self, index: usize, column: &serde_json::Value) -> Html {
        let callback = self.link.callback(move |_e| Msg::SortClicked(index));
        html! {
            // Render Column
            <td onclick=callback>
            {
                // hack to remove "" for string values
                if let Some(column) = column.as_str() {
                    format!("{}", column)
                } else if let Some(column) = column.as_f64() {
                    format!("{:.2}", column)
                } else {
                    format!("{}", column)
                }
            }
            </td>
        }
    }

    fn table_from_vecs(&self, header: &Vec<String>, data: &Vec<Vec<serde_json::Value>>) -> Html {
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
    fn test_table() {}

    #[wasm_bindgen_test]
    fn test_table_data() {
        let headers = vec![
            "City".to_string(),
            "Population".to_string(),
            "Rating".to_string(),
        ];
        let rows: Vec<Vec<serde_json::Value>> = vec![
            vec!["Prague".into(), 14000000.into(), 104.into()],
            vec!["London".into(), 86300000.into(), 106.into()],
            vec!["Madrid".into(), 66420000.into(), 106.into()],
        ];

        let table_data = TableData::new(headers, rows);
        assert_eq!(table_data.headers.first().unwrap(), &"City".to_string());

        // Data check
        let prague = table_data.data.first().unwrap().first().unwrap();
        assert_eq!(prague, &"Prague".to_string());
    }
}
