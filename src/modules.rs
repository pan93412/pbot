use crate::modules::base::ModuleActivator;

use self::base::ActivatedModuleInfo;

pub mod base;
pub mod fwd;

pub fn enabled_modules() -> Vec<ActivatedModuleInfo> {
    vec![fwd::FwdModuleActor::activate_module()]
}
