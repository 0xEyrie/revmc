use std::sync::Once;
use tokio::runtime::Runtime;

static mut RUNTIME: Option<Runtime> = None;
static INIT: Once = Once::new();

#[allow(static_mut_refs)]
pub(crate) fn get_runtime() -> &'static Runtime {
    unsafe {
        INIT.call_once(|| {
            RUNTIME = Some(Runtime::new().unwrap());
        });
        RUNTIME.as_ref().unwrap()
    }
}
