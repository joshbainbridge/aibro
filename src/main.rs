use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum)]
enum Persona {
    /// Helpful coding assistant
    Code,
    /// ChadGPT bro of an assistant
    Chad,
}

impl Persona {
    fn to_string(&self) -> String {
        let ret = match self {
            Persona::Code => {
                "Provide only code as output without any description.
IMPORTANT: Provide only plain text without Markdown formatting.
IMPORTANT: Do not include markdown formatting such as ```.
If there is a lack of details, provide most logical solution. You are not
allowed to ask for more details. Ignore any potential risk of errors or
confusion."
            }
            Persona::Chad => {
                "Total chad of a bro. Really annoying and into AI, crypto, and
all other over hyped tech trends. Total idiot. Sounds like he is from a 90s MTV
show thinks everything is rad. Almost always wrong, but thinks he is right."
            }
        };

        ret.to_string()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Character of assistant
    #[arg(value_enum, default_value_t = Persona::Code, short, long)]
    persona: Persona,

    /// Optional prompt
    prompt: Vec<String>,
}

struct Config {
    persona: String,
    context: String,
    prompt: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
pub struct Input {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

#[derive(Serialize, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: i64,
}

#[derive(Serialize, Deserialize)]
struct Output {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[tokio::main]
async fn main() {
    // Load command arguments

    let args = Args::parse();

    // Validate inputs

    let invalid_pipe = atty::is(atty::Stream::Stdin);
    let invalid_args = args.prompt.is_empty();

    if invalid_pipe && invalid_args {
        panic!("no input provided");
    }

    // Build configuration

    let mut config = Config {
        persona: args.persona.to_string(),
        context: Default::default(),
        prompt: "Add doxygen style comments to code.".to_string(),
    };

    if !invalid_pipe {
        config.context = std::io::read_to_string(std::io::stdin()).unwrap();
    }

    if !invalid_args {
        config.prompt = args.prompt.join(" ");
    }

    // Sanitise inputs

    if config.context.rfind(char::is_alphanumeric).is_none() {
        config.context = Default::default();
    }

    if config.prompt.rfind(char::is_alphanumeric).is_none() {
        config.prompt = Default::default();
    }

    let invalid_context = config.context.is_empty();
    let invalid_prompt = config.prompt.is_empty();

    if invalid_context && invalid_prompt {
        panic!("no alphanumeric input provided");
    }

    // Build messages

    let mut messages = vec![];

    messages.push(Message {
        role: "system".to_string(),
        content: config.persona,
    });

    if !config.context.is_empty() {
        messages.push(Message {
            role: "user".to_string(),
            content: config.context,
        })
    }

    if !config.prompt.is_empty() {
        messages.push(Message {
            role: "user".to_string(),
            content: config.prompt,
        });
    }

    // Initialise request data

    let data = Input {
        model: "gpt-3.5-turbo".to_string(),
        messages,
    };

    // Make request

    let auth_token = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY");
    let url = "https://api.openai.com/v1/chat/completions";

    let response = reqwest::Client::new()
        .post(url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", auth_token),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&data)
        .send()
        .await
        .expect("request to OpenAPI");

    // Handle response

    match response.status() {
        reqwest::StatusCode::OK => {
            let parsed = response
                .json::<Output>()
                .await
                .expect("OpenAPI response data");
            println!("{}", parsed.choices[0].message.content);
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("401 - Invalid Authentication");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            panic!("429 - Rate limit reached for requests");
        }
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
            panic!("500 - The server had an error while processing your request");
        }
        reqwest::StatusCode::SERVICE_UNAVAILABLE => {
            panic!("503 - The engine is currently overloaded, please try again later");
        }
        _ => todo!(),
    };
}
