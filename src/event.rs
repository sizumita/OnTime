use serenity::async_trait;
use serenity::model::prelude::{Activity, Interaction, Ready};
use serenity::prelude::{Context, EventHandler};
use crate::command::command;


#[async_trait]
impl EventHandler for crate::Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("on ready: {}", ready.user.name);
        ctx.set_activity(
            Activity::playing("Focus your work...")
        ).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(cmd) => {
                command(&ctx, &cmd).await;
            }
            _ => {}
        }
    }
}
