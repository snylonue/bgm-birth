use clap::{arg, command, Parser};
use futures::future::join_all;
use reqwest::{Client, RequestBuilder, Url};
use scraper::{Html, Selector};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const BGM_API_ENDPOINT: &str = "https://api.bgm.tv";

#[derive(Debug)]
struct BangumiApi {
    endpoint: Url,
}

impl BangumiApi {
    fn character(&self, character_id: u32) -> Url {
        self.endpoint
            .join(&format!("v0/characters/{character_id}"))
            .unwrap()
    }
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
struct Stat {
    pub comments: u32,
    pub collects: u32,
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
struct CharacterDetail {
    pub id: u32,
    pub name: String,
    pub summary: String,
    pub locked: bool,
    pub stat: Stat,
}

async fn request_json<T: DeserializeOwned>(req: RequestBuilder) -> reqwest::Result<T> {
    req.send().await?.error_for_status()?.json().await
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct App {
    /// show json response
    #[arg(short, long)]
    json: bool,
    /// show locked subjects
    #[arg(long)]
    locked: bool,
    /// filter subjects with too few comments
    #[arg(long)]
    min_comments: Option<u32>,
    #[arg(short, long)]
    summary: bool,
}

async fn _main() -> anyhow::Result<()> {
    let cli = App::parse();

    let api = BangumiApi {
        endpoint: Url::parse(BGM_API_ENDPOINT).unwrap(),
    };

    let client = Client::builder().user_agent("bgm-birth/0.1.0").build()?;

    let mono_page = client
        .get("https://bgm.tv/mono")
        .send()
        .await?
        .text()
        .await?;

    let parsed = Html::parse_document(&mono_page);
    let selector =
        Selector::parse("div.side:nth-child(1) > dl > dt:nth-child(1) > a:nth-child(1)").unwrap();

    let characters = parsed
        .select(&selector)
        .map(|e| {
            // expects a link in format of '/character/{id}'
            e.attr("href")
                .and_then(|l| l.rsplit_once('/'))
                .unwrap()
                .1
                .parse::<u32>()
                .unwrap()
        })
        .map(|id| api.character(id))
        .map(|url| request_json::<CharacterDetail>(client.get(url)));

    let res = join_all(characters).await;

    for (idx, character) in res
        .into_iter()
        .filter_map(|r| r.inspect_err(|e| eprintln!("Error: {e}")).ok())
        .filter(|ch| cli.locked || !ch.locked)
        .filter(|ch| cli.min_comments.map_or(true, |min| ch.stat.comments >= min))
        .enumerate()
    {
        match (cli.json, cli.summary) {
            (true, _) => println!("{}", serde_json::to_string_pretty(&character).unwrap()),
            (_, true) => println!("{idx}: {}: {}\n", character.name, character.summary),
            _ => println!("{}", character.name),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    _main().await
}
