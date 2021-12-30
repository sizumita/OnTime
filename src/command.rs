use std::ops::Add;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue};
use serenity::prelude::Context;
use chrono::{Utc, Duration};
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use serenity::model::prelude::InteractionResponseType::DeferredChannelMessageWithSource;

pub async fn command(ctx: &Context, command: &ApplicationCommandInteraction) {
    match &*command.data.name {
        "focus" => {
            if let ApplicationCommandInteractionDataOptionValue::Integer(diff) =
                command.data.options.get(0).unwrap().resolved.as_ref().unwrap() {
                if diff.clone() <= 0 || diff.clone() > 1440 {
                    let _ = command.create_interaction_response(
                        &ctx.http,
                        |resp| resp
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(
                                |message| message
                                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                                    .content("0から1440分の間で指定してください。")
                            )
                    ).await;
                    return;
                }
                let user_id = command.clone().member.unwrap().clone().user.id;
                let _ = command.create_interaction_response(
                    &ctx.http,
                    |response| {
                        response.interaction_response_data(
                            |data| data
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL
                                )
                        ).kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    },
                ).await.unwrap();
                for guild in &ctx.cache.guilds().await {
                    if guild.member(&ctx, &user_id).await.is_ok() {
                        let _ = guild.edit_member(&ctx.http, &user_id,
                          |edit| edit
                              .disable_communication_until_datetime::<Utc>(Utc::now().add(Duration::minutes(diff.clone())))
                        ).await;
                    }
                }
                let _ = command.create_followup_message(
                    &ctx.http,
                    |message| message
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                        .content("タイムアウトを開始しました。頑張って！")
                ).await;
            }
        }
        _ => {}
    }
}
