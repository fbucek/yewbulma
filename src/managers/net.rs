use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{Callback, format::{Nothing}};
use anyhow::anyhow;

pub struct RequestManager {
    fetch_service: FetchService,
    tasks: Vec<FetchTask>,
}

impl RequestManager {
    pub fn new() -> Self {
        Self {
            fetch_service: FetchService::new(),
            tasks: Vec::new(),
        }
    }

    pub fn get_url(&mut self, url: String, callback: Callback<anyhow::Result<String>>) -> anyhow::Result<()>{
        let request_url = url.clone();
        let handler = move |response: Response<Result<String, anyhow::Error>>| {
            let (meta, data) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                let err = Err(anyhow!("{}: error getting data from url: {} data: {:?}", meta.status, &url, &data));
                callback.emit(err);
            }
        };
        let request = Request::get(request_url.as_str()).body(Nothing).unwrap();
        let task = self.fetch_service.fetch(request, handler.into()).unwrap();
        self.tasks.push(task);
        
        Ok(())
    }


    pub fn get_bin(&mut self, url: String, callback: Callback<Result<Vec<u8>, anyhow::Error>>) -> anyhow::Result<()>{
        let request_url = url.clone();
        let handler = move |response: Response<Result<Vec<u8>, anyhow::Error>>| {
            let (meta, data) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                // format_err! is a macro in crate `failure`
                let err = Err(anyhow!("{}: error getting data from url: {} data: {:?}", meta.status, &url, &data));
                callback.emit(err);
            }
        };

        let request = Request::get(request_url.as_str()).body(Nothing).unwrap();
        let task = self.fetch_service.fetch_binary(request, handler.into()).unwrap();
        self.tasks.push(task);

        Ok(())
    }


    pub fn post(&mut self, url: String, body: String, callback: Callback<anyhow::Result<String>>) -> anyhow::Result<()> {
        let request_url = url.clone();
        // Handler closure - will solve post request
        let handler = move |response: Response<Result<String, anyhow::Error>>| {
            let (meta, data) = response.into_parts();
            if meta.status.is_success() {
                log::trace!("status is_success()");
                callback.emit(data)
            } else {
                log::error!("status is not success()");
                // format_err! is a macro in crate `failure`
                let err = Err(anyhow!("{}: error getting data from url: {} data: {:?}", meta.status, &url, &data));
                callback.emit(err);
            }
        };

        // Initiate request
        let request = Request::post(request_url.as_str())
            .header("Content-Type", "application/json")
            .body(Ok(body));
        match request {
            Ok(request) => {
                if let Ok(task) = self.fetch_service.fetch(request, handler.into()) {
                    log::trace!("trying to post message");
                    self.tasks.push(task);
                }
            },
            Err(_) => log::error!("not possible to create post request"),
        }

        Ok(())
    }

    // pub fn post_json(&mut self, url: String, body: String, callback: Callback<Result<String, anyhow::Error>>) -> FetchTask {
    //     let request_url = url.clone();
    //     let handler = move |response: Response<Result<String, anyhow::Error>>| {
    //         let (meta, data) = response.into_parts();
    //         if meta.status.is_success() {
    //             callback.emit(data)
    //         } else {
    //             // format_err! is a macro in crate `failure`
    //             let err = Err(anyhow!("{}: error getting data from url: {} data: {:?}", meta.status, &url, &data));
    //             callback.emit(err);
    //         }
    //     };

    //     let request = Request::post(request_url.as_str()).body(body).unwrap();
    //     self.fetch_service.fetch(request, handler.into()).unwrap()
    // }

}
