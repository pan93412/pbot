pub mod base;
pub mod fwd;

use self::base::Module;

pub fn enabled_modules() -> Vec<impl Module + Clone + Send> {
    vec![
        fwd::FwdModule::default(),
    ]
}
