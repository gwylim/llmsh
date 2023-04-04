use std::env;
use std::error::Error;
use std::io::{stdin, Read};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use regex::Regex;

fn format_prompt(history: &str, query: &str) -> String {
    format!("This is the bash history:\n\n```bash\n{}\n```Now write a command to accomplish the following (output only a single command): {}", history, query)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let mut input = String::new();
    stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    let query = env::args().nth(1).expect("Usage: llmsh PROMPT");

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a helpful assistant who translates instructions to Bash shell commands.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(format_prompt("", "List files in the current directory, including hidden files"))
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content("```bash\nls -a\n```")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(format_prompt(input.trim(), &query))
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    let shell_regex = Regex::new(r"```bash\n(.*)\n```").unwrap();
    let command = shell_regex
        .captures(&response.choices[0].message.content)
        .and_then(|captures| captures.get(1).map(|group| group.as_str()))
        .expect("No shell command found in GPT completion");

    println!("{}", command);

    Ok(())
}
