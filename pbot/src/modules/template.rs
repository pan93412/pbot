//! PBot: Modules: Template
//!
//! Description here.
//!
//! You can find every part that may need to modify by
//! finding the prefix `DEVEDIT: `.
//!
//! You would need to include this module in `modules.rs`,
//! and initiate your module in `main.rs`.

use actix::prelude::*;
use log::info;
use pbot_modules_derive::{ModuleActivator, ModuleActor, ModuleMeta};

use super::base::ModuleMessage;

/// The TemplateModule actor.
#[derive(Clone, Default, ModuleActor, ModuleActivator, ModuleMeta)]
#[name = "TemplateModule"]
pub struct TemplateModuleActor {
    // DEVEDIT: You can specify your actor's context here.
}

impl Handler<ModuleMessage> for TemplateModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        // https://github.com/actix/actix/issues/308
        // DEVEDIT: You can clone the variables from self here to workaround this error.

        async move {
            // Destruct msg and get `handle` and `message`.
            let ModuleMessage {
                handle: _,
                message: _,
            } = msg;

            // DEVEDIT: Your logic here.
            //
            // You can separate your logic into different functions
            // for better readability.

            // It worked with no fault errors! 👌
            Ok(())
        }
        .into_actor(self)
        .boxed_local()
    }
}
