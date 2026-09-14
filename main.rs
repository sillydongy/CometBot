use poise::serenity_prelude as serenity;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use markov_generator::{AddEdges, HashChain};
use std::sync::LazyLock;
use rwkv_tokenizer::WorldTokenizer;
use serde::Deserialize;
use csv::Reader;

// Define custom app data shared across all commands (e.g., database connections)
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

static SUSSY_CHAIN: LazyLock<Markob> = LazyLock::new(|| {
    let mut tempchain = HashChain::new(3);
    let mut temptopics: HashSet<Vec<u16>> = HashSet::new();
    let mut tempusermap = HashMap::new();
    let tokenizer = WorldTokenizer::new(None).expect("Failed to load RWKV tokenizer");

    let mut rdr = Reader::from_path("topics_with_username.csv").expect("Unable to open file");

    for result in rdr.deserialize() {
        let topic: Topic = result.expect("Unable to assign Topic");
        let username: String = topic.username.to_lowercase();
        let token_ids = tokenizer.encode(&topic.content);
        let mut start_tokens = token_ids.clone();
        if token_ids.clone().len() > 10 {
            start_tokens = (&token_ids[0..10]).to_vec();
        }
        tempusermap.entry(username).or_insert_with(Vec::new).push(start_tokens);
        tempchain.add_all(token_ids.clone().into_iter(), AddEdges::Start);
        temptopics.insert(token_ids);
    }

    Markob{
        chain: tempchain,
        topics: temptopics,
        usermap: tempusermap,
        tokenizer: tokenizer,
    }
});

struct Markob {
    chain: HashChain<u16>,
    topics: HashSet<Vec<u16>>,
    usermap : HashMap<String, Vec<Vec<u16>>>,
    tokenizer: WorldTokenizer,
}

#[derive(Debug, Deserialize)]
struct Topic {
    content: String,
    username: String
}

/// Say hi to Comet
#[poise::command(slash_command)]
async fn hicomet(
    ctx: Context<'_>
) -> Result<(), Error> {
    let strings = [
        "JORP!".to_string(),
        "Meow".to_string(),
        "Nya!".to_string(),
        "Chirp chirp chirp!".to_string(),
        "Mmprrrrrrtt!".to_string(),
        "Hisssssssss!".to_string(),
        "Hello it is me, Comet the cat.".to_string(),
    ];

    let random_index = rand::thread_rng().gen_range(0..strings.len());
    let response: String = strings[random_index as usize].to_string();

    ctx.say(response).await?;
    Ok(())
}

/// Ask Comet for his sage advice
#[poise::command(slash_command)]
async fn ask(
    ctx: Context<'_>,
    #[description = "Question for Comet"] prompt: String,
) -> Result<(), Error> {
    let strings: [&str; 11] = [
        "*Purrs loudly* (It is certain)",
        "Chirp chirp! (Yes definitely)",
        "Mmprrrrrrtt! (You may rely on it)",
        "Mrrrrrow! (As I see it, yes)",
        "Mrrrp! (Outlook good)",
        "*Farts loudly* (Better not tell you now)",
        "Yooooowl! (Don't count on it)",
        "*Bites* (My reply is no)",
        "*Scratches* (My sources say no)",
        "*Spits* (Outlook not so good)",
        "Hissssssss! (Very doubtful)",
    ];

    let random_index = rand::thread_rng().gen_range(0..strings.len());
    let response = format!("Question: {}\n Answer: {}", prompt, strings[random_index as usize]);

    ctx.say(response).await?;
    Ok(())
}

/// Become blessed by the appearance of Comet
#[poise::command(slash_command)]
async fn comet(
    ctx: Context<'_>
) -> Result<(), Error> {
    let strings = [
        "<:comet:1244080987269369867>".to_string(),
        "<:santacomet:1321537744111140874>".to_string(),
        "<:wendycomet:1410066939497283694>".to_string(),
        "<:chonkmet:1293382106218758185>".to_string(),
        "<:spacecat:1257663481797283850>".to_string(),
    ];

    let random_index = rand::thread_rng().gen_range(0..strings.len());
    let response: String = strings[random_index as usize].to_string();

    ctx.say(response).await?;
    Ok(())
}

/// Give Comet a treat
#[poise::command(slash_command)]
async fn givetreat(
    ctx: Context<'_>
) -> Result<(), Error> {
    let random_index = rand::thread_rng().gen_range(0..5);
    let message: String;
    if random_index == 0 {
        message = "*Explodes*".to_string();
    } else {
        message = "*Purrrrr!*".to_string();
    }

    ctx.say(message).await?;
    Ok(())
}

/// Give Comet some pets
#[poise::command(slash_command)]
async fn pet(
    ctx: Context<'_>
) -> Result<(), Error> {
    let random_index = rand::thread_rng().gen_range(0..5);
    let message: String;
    if random_index == 0 {
        message = "*Bites*".to_string();
    } else {
        message = "*Headrub*".to_string();
    }

    ctx.say(message).await?;
    Ok(())
}

/// Try to give Comet a belly rub...
#[poise::command(slash_command,)]
async fn bellyrub(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say("So you have chosen death...").await?;
    Ok(())
}

/// Give Comet a french fry
#[poise::command(slash_command)]
async fn givefry(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say("*sogs it* :fries: <:comet:1244080987269369867>").await?;
    Ok(())
}

/// Ask Comet for an AI Peter topic suggestion
#[poise::command(slash_command)]
async fn suggest(
    ctx: Context<'_>,
    #[description = "Optional start to chain"] prompt: Option<String>,
) -> Result<(), Error> {
    if prompt.clone().is_none() {
        let mut token_length = 61;
        let mut encoded: Vec<u16> = Vec::new();
        while token_length > 60 {
            encoded = SUSSY_CHAIN.chain.generate().copied().collect();
            token_length = encoded.len();
            if SUSSY_CHAIN.topics.contains(&encoded) {
                token_length = 61;
            }
        }

        let mut result = SUSSY_CHAIN.tokenizer.decode(encoded).expect("Invalid UTF-8");
        result.truncate(500);
        ctx.say(result).await?;
    } else {
        let input = prompt.clone().unwrap();
        let mut result: String = {
            let mut token_ids = SUSSY_CHAIN.tokenizer.encode(&input);
            let mut generator = SUSSY_CHAIN.chain.generate();

            generator.set_state(token_ids.iter());

            let mut new_tokens: Vec<u16> = generator.by_ref().copied().collect();
            while new_tokens.len() == 0 || new_tokens.len() > 60 {
                if new_tokens.len() == 0 || new_tokens.len() > 60 {
                    let temp_input = prompt.clone().unwrap();
                    token_ids = SUSSY_CHAIN.tokenizer.encode(&temp_input);
                    let last_three = if token_ids.len() >= 3 {
                        token_ids[token_ids.len() - 3..].to_vec()
                    } else {
                        token_ids.to_vec()
                    };
                    generator = SUSSY_CHAIN.chain.generate();
                    generator.set_state(last_three.iter());
                    new_tokens = generator.by_ref().copied().collect();
                }

                if new_tokens.len() == 0 || new_tokens.len() > 60 {
                    let temp_input = "Peter";
                    token_ids = SUSSY_CHAIN.tokenizer.encode(&temp_input);
                    generator = SUSSY_CHAIN.chain.generate();

                    generator.set_state(token_ids.iter());

                    new_tokens = generator.by_ref().copied().collect();
                }
            }

            SUSSY_CHAIN.tokenizer.decode(new_tokens).expect("Decode error")
        };

        result = format!("{}{}", input, result);
        ctx.say(result).await?;
    }

    Ok(())
}

/// Ask Comet for a topic that starts by the specified user
#[poise::command(slash_command)]
async fn suggestby(
    ctx: Context<'_>,
    #[description = "Optional start to chain"] user: Option<String>,
) -> Result<(), Error> {
    let username: String;
    if user.is_none() {
        username = ctx.author().name.to_lowercase();
    } else {
        username = user.unwrap().to_lowercase();
    }
    if let Some(starts) = SUSSY_CHAIN.usermap.get(&username) {
        let random_index = rand::thread_rng().gen_range(0..starts.len());
        let start_tokens = &starts[random_index];
        let last_three = if start_tokens.len() >= 3 {
            start_tokens[start_tokens.len() - 3..].to_vec()
        } else {
            start_tokens.to_vec()
        };
        let mut result: String = {
            let mut new_tokens = Vec::new();
            for _i in 0..100 {
                if new_tokens.len() < 15 || new_tokens.len() > 60 {
                    let mut generator = SUSSY_CHAIN.chain.generate();
                    generator.set_state(last_three.iter());
                    new_tokens = generator.by_ref().copied().collect();
                }
            }
            SUSSY_CHAIN.tokenizer.decode(new_tokens).expect("w9feiwefiojweof")
        };
        let start = SUSSY_CHAIN.tokenizer.decode(start_tokens.to_vec()).expect("w9feiwefiojweof");
        result = format!("{}{}", start, result);
        ctx.say(result).await?;

    } else {
        ctx.say("Invalid username").await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Load the Discord bot token from your environment variables
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN environment variable");

    let target_guild_id = serenity::GuildId::new(1115999278654042316);

    let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
        commands: vec![hicomet(), ask(), comet(), suggest(), givetreat(), pet(), bellyrub(), givefry(), suggestby()],
             ..Default::default()
    })
    .setup(move |ctx, _ready, framework| {
        Box::pin(async move {
            serenity::Command::set_global_commands(&ctx.http, vec![])
            .await
            .expect("Failed to clear global commands");

            // Register the commands ONLY to the specified guild
            poise::builtins::register_in_guild(
                ctx,
                &framework.options().commands,
                target_guild_id
            ).await?;

            println!("Guild-specific commands registered!");
            Ok(Data {})
        })
    })
    .build();

    let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::empty())
    .framework(framework)
    .await;

    SUSSY_CHAIN.tokenizer.encode("Wake up");
    println!("Bot is booting up...");
    client.unwrap().start().await.unwrap();
}
