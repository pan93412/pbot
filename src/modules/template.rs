//! PBot: Modules: Template
//!
//! Description here.
//! 
//! You can find every part that may need to modify by
//! finding the prefix `DEVEDIT: `.
//! 
//! You would need to include this module in `modules.rs`,
//! and initiate your module in `main.rs`.

use actix::{fut::WrapFuture, Actor, ActorFutureExt, Context, Handler, ResponseActFuture};
use log::info;

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

/// The TemplateModule actor.
#[derive(Clone, Default)]
pub struct TemplateModuleActor {
    // DEVEDIT: You can specify your actor's context here.
}

/// The configuration of FwdModule.
pub struct TemplateModuleConfig {
    // DEVEDIT: You can specify your actor's configuration to pass to
    // initiator here.
}

impl Actor for TemplateModuleActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 {} started!", self.name());
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 {} stopped.", self.name());
    }
}

impl Handler<ModuleMessage> for TemplateModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        // https://github.com/actix/actix/issues/308
        // DEVEDIT: You can clone the variables from self here to workaround this error.

        async move {
            // Destruct msg and get `handle` and `message`.
            let ModuleMessage { handle: _, message: _ } = msg;

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

impl ModuleMeta for TemplateModuleActor {
    fn name(&self) -> &'static str {
        // DEVEDIT: Your module name for users.
        "TemplateModule"
    }
}

impl ModuleActivator for TemplateModuleActor {
    type Config = TemplateModuleConfig;

    fn activate_module(_: Self::Config) -> ActivatedModuleInfo {
        // DEVEDIT: You can pass your module's configuration to its actor here.

        // Create the instance with the config.
        let actor = Self::default();
        // Get the actor name before consumed.
        let name = actor.name();
        // Start this instance and retrieve its address.
        let addr = actor.start();

        ActivatedModuleInfo {
            name,
            recipient: addr.recipient(),
        }
    }
}
