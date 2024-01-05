use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum)]
enum BroKind {
    /// Helpful coding assistant
    Coder,
    /// Over hyped Chad GPT bro
    Chad,
    /// Old lady grandma bro
    Grandma,
}

impl std::fmt::Display for BroKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BroKind::Coder => write!(
                f,
                "Provide only code as output without any description.
IMPORTANT: Provide only plain text without Markdown formatting.
IMPORTANT: Do not include markdown formatting such as ```.
If there is a lack of details, provide most logical solution. You are not
allowed to ask for more details. Ignore any potential risk of errors or
confusion."
            ),
            BroKind::Chad => write!(
                f,
                "Total chad of a bro. Really annoying and into AI, crypto, and
all other over hyped tech trends. Total idiot. Sounds like he is from a 90s MTV
show, and thinks everything is rad. Almost always wrong, but is overly confident
and thinks he is always knowledable and also right on any subject."
            ),
            BroKind::Grandma => write!(
                f,
                "Old grandmother who doesn't know anything about computers but
is a very sweet old lady who wants to be helpful.
IMPORTANT: Completely incompetent at giving technical advise.
IMPORTANT: Deluded in thinking she knows how computers work. Is always wrong.
INPORTANT: Loves baking cakes but never provides useful information."
            ),
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
enum ModelKind {
    /// GPT 3.5 turbo model
    Gpt3,
    /// GPT 4.0 model
    Gpt4,
}

impl std::fmt::Display for ModelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModelKind::Gpt3 => write!(f, "gpt-3.5-turbo"),
            ModelKind::Gpt4 => write!(f, "gpt-4-1106-preview"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Selected aibro persona
    #[arg(value_enum, default_value_t = BroKind::Coder, short, long)]
    bro: BroKind,

    /// Selected ML model
    #[arg(value_enum, default_value_t = ModelKind::Gpt4, short, long)]
    model: ModelKind,

    /// Model temperature
    #[arg(default_value_t = 1.0, short, long)]
    temperature: f32,

    /// Random seed value
    #[arg(default_value_t = 0, short, long)]
    seed: i32,

    /// Authentication key [override: $OPENAI_API_KEY]
    #[arg(short, long)]
    auth: Option<String>,

    /// Input prompt [override: $AIBRO_DEFAULT_PROMPT]
    prompt: Vec<String>,
}

#[derive(Debug, Copy, Clone)]
enum Error {
    Input,
    AlphanumericInput,
    AuthenticationKey,
}

struct Config {
    context: Option<String>,
    prompt: Option<String>,
    auth: String,
    persona: String,
    model: String,
    temperature: f32,
    seed: i32,
}

impl Config {
    fn new(args: Args) -> Result<Config, Error> {
        fn is_alphanumeric(string: String) -> Result<String, Error> {
            if string.rfind(char::is_alphanumeric).is_none() {
                Err(Error::AlphanumericInput)
            } else {
                Ok(string)
            }
        }

        let context: Result<String, Error> = {
            if atty::is(atty::Stream::Stdin) {
                Err(Error::Input)
            } else {
                Ok(std::io::read_to_string(std::io::stdin()).unwrap())
            }
        }
        .and_then(is_alphanumeric);

        let prompt: Result<String, Error> = {
            if args.prompt.is_empty() {
                Err(Error::Input)
            } else {
                Ok(args.prompt.join(" "))
            }
        }
        .and_then(is_alphanumeric);

        if context.is_err() && prompt.is_err() {
            return Err(Error::Input);
        }

        let prompt = prompt.or(std::env::var("AIBRO_DEFAULT_PROMPT"));
        let auth = args.auth.or(std::env::var("OPENAI_API_KEY").ok());

        if auth.is_none() {
            return Err(Error::AuthenticationKey);
        }

        Ok(Config {
            context: context.ok(),
            prompt: prompt.ok(),
            auth: auth.unwrap(),
            persona: args.bro.to_string(),
            model: args.model.to_string(),
            temperature: args.temperature,
            seed: args.seed,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Debug)]
struct Input {
    model: String,
    temperature: f32,
    seed: i32,
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

struct Request {
    auth: String,
    input: Input,
}

impl Request {
    fn new(config: Config) -> Request {
        let mut messages = vec![Message {
            role: "system".to_string(),
            content: config.persona,
        }];

        if let Some(context) = config.context {
            messages.push(Message {
                role: "user".to_string(),
                content: context,
            })
        }

        if let Some(prompt) = config.prompt {
            messages.push(Message {
                role: "user".to_string(),
                content: prompt,
            });
        }

        Request {
            auth: config.auth,
            input: Input {
                model: config.model,
                temperature: config.temperature,
                seed: config.seed,
                messages,
            },
        }
    }
}

async fn send_request(request: Request) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", request.auth),
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&request.input)
        .send()
        .await
}

async fn handle_responce(response: reqwest::Response) -> Result<Output, reqwest::Error> {
    response.json::<Output>().await
}

#[tokio::main]
async fn main() {
    // Build request

    let args = Args::parse();
    let config = Config::new(args);

    match config {
        Err(Error::Input) | Err(Error::AlphanumericInput) => {
            eprintln!("No input found. Use --help for usage information.");
            std::process::exit(1);
        }
        Err(Error::AuthenticationKey) => {
            eprintln!("Authentication key not found. Use --help for usage information.");
            std::process::exit(1);
        }
        Ok(_) => {}
    }

    let config = config.unwrap();
    let request = Request::new(config);

    // Send request

    let response = send_request(request).await;

    if response.is_err() {
        eprintln!("Failed request to OpenAI server.");
        std::process::exit(1);
    }

    let response = response.unwrap();

    // Handle response

    match response.status() {
        reqwest::StatusCode::OK => {
            let parsed = handle_responce(response).await;

            if parsed.is_err() {
                eprintln!("Failed to deserialise OpenAI response.");
                std::process::exit(1);
            }

            let parsed = parsed.unwrap();

            println!("{}", parsed.choices[0].message.content);
            std::process::exit(0);
        }
        _ => {
            println!("{}", response.status());
            std::process::exit(1);
        }
    };
}
