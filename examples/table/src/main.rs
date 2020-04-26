fn main() {
    if console_log::init_with_level(log::Level::Trace).is_err() {
        log::error!("Not possible to init logger / message irrelevant -> log not working");
    }
    yew::start_app::<exampletable::Model>();
}
