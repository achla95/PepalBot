use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::{
    async_trait,
    model::gateway::Ready,
    // prelude::*,
};
use std::collections::HashMap;
use tokio;
use dotenv;

mod structz{
    pub mod users;
}
#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "A simple test bot").await?;

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}
#[command]
async fn notes(ctx: &Context, msg: &Message) -> CommandResult {
    let username = String::from("achbabi");
    let password = String::from("Ay30ch09&*");
    let user: structz::users::User = structz::users::User::new(username,password); // on créer notre un user PS : ajouter mut
    let mut login_info: HashMap<&str,&str> = HashMap::new();
    login_info.insert("login", user.username.as_str());
    login_info.insert("pass", user.password.as_str());
    let notes = user.get_notes(&login_info).await?;
    // msg.channel_id.say(&ctx.http, format!("Notes :\n {:?}", notes)).await?;
    let msg = msg
    .channel_id
    .send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Vos notes");
            for element in notes{
                e.field(format!("{} ",element[1].as_str()), format!("{}", element[2].as_str()), true);
            }
            e.color((135,206,250));
            e.footer(|f| {
                f.text("PepalBot made by achla95");

                f
            });
            e
        });
        m
    })
    .await;

if let Err(why) = msg {
    println!("Error sending message: {:?}", why);
}
    Ok(())
}

#[command]
async fn prez(ctx: &Context, msg: &Message) -> CommandResult {
    let username = String::from("achbabi");
    let password = String::from("Ay30ch09&*");
    let user: structz::users::User = structz::users::User::new(username,password); // on créer notre un user PS : ajouter mut
    let mut login_info: HashMap<&str,&str> = HashMap::new();
    login_info.insert("login", user.username.as_str());
    login_info.insert("pass", user.password.as_str());
    user.set_presence(&login_info).await?;
    let msg = msg
    .channel_id
    .send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Validation présence Pepal");
            e.description("presence set ");
            e.color((135,206,250));
            e.footer(|f| {
                f.text("PepalBot made by achla95");
                f
            });
            e
        });
        m
    })
    .await;

if let Err(why) = msg {
    println!("Error sending message: {:?}", why);
}
    Ok(())
}

#[group]
#[commands(about, ping,notes,prez)]
struct General;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    dotenv::from_filename(".env").expect("failed to find .env");
    let token = std::env::var("DISCORD_TOKEN")?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        // The `#[group]` (and similarly, `#[command]`) macro generates static instances
        // containing any options you gave it. For instance, the group `name` and its `commands`.
        // Their identifiers, names you can use to refer to these instances in code, are an
        // all-uppercased version of the `name` with a `_GROUP` suffix appended at the end.
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token).event_handler(Handler).framework(framework).await?;


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}
