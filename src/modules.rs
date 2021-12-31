use actix::{Handler, Actor};

use crate::modules::base::ModuleActivator;

use self::base::{ModuleMessage, ActivatedModuleInfo};

pub mod base;
pub mod fwd;

pub fn enabled_modules() -> Vec<ActivatedModuleInfo<impl 'static + Actor + Handler<ModuleMessage>>> {
    vec![
        fwd::FwdModuleActor::activate_module()
    ]
}
