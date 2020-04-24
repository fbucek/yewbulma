#[derive(Default, Debug, Clone, PartialEq)] // serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(default)]
pub struct MenuItem {
    pub name: String,
    pub separator: bool,
    pub icon: Option<String>,
    pub inner: Option<String>,
    pub children: Vec<MenuItem>,
    pub link: Option<String>,
    pub expanded: bool,
    pub id: Option<String>,
}

impl MenuItem {
    pub fn url_href(&self, origin: String) -> String {
        if !self.children.is_empty() {
            "#".to_string()
        } else if let Some(link) = &self.link {
            link.to_string()
        } else {
            format!(
                "{}/#{}",
                origin,
                &self.name.to_lowercase().replace(' ', "-")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: change to wasm-test

    fn test_item(name: &str, link: Option<&str>, origin: &str, expected: &str) {
        let mut item = MenuItem::default();
        item.name = name.to_string();
        item.link = link.map(|link| link.to_string());

        let url = item.url_href(origin.to_string());
        assert_eq!(url, expected);
    }

    ///
    #[test]
    fn test_url_href() {
        test_item("dashboard", Some("/menu"), "localhost", "/menu");
        test_item("dashboard", None, "localhost", "localhost/#dashboard");
        test_item(
            "dashboard",
            None,
            "http://127.0.0.1:9000",
            "http://127.0.0.1:9000/#dashboard",
        );
    }
}
