use std::fmt::{Error, Formatter};
use std::future::Future;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{Component, ComponentLink};

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), not(cargo_web)))]
/// This method processes a Future that returns a message and sends it back to the component's
/// loop.
///
/// # Panics
/// If the future panics, then the promise will not resolve, and will leak.
pub fn send_future<COMP: Component, F>(link: &ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    use wasm_bindgen_futures::future_to_promise;

    let link = link.clone();
    let js_future = async move {
        link.send_message(future.await);
        Ok(JsValue::NULL)
    };

    let _promise = future_to_promise(js_future);
}

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

/// Fetches Url
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn fetch_url(url: String) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;

    // Using native browser to get data
    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

pub async fn fetch_text<T: Into<String>>(url: T) -> String {
    match fetch_url(url.into()).await {
        Ok(data) => data,
        Err(_) => "".to_string(),
    }
}
