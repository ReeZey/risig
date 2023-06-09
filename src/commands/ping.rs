use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("pong")
}

pub async fn run() -> String {
    "pong".to_string()
}


