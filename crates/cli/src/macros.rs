/// Macro that has a loader until it is finished
/// Like python `with` context
#[macro_export]
macro_rules! loader {
    ($($arg:tt)+) => {
        let loader = Loader::start(LoaderVariant::default());
        $($arg)+
        loader.stop();
    };
}
