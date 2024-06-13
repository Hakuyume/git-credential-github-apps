use clap::{Args, Parser, ValueEnum};
use http::Uri;
use jsonwebtoken::EncodingKey;
use octocrab::Octocrab;
use secrecy::ExposeSecret;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(long)]
    endpoint: Option<Uri>,
    #[command(flatten)]
    app_id: AppId,
    #[command(flatten)]
    private_key: PrivateKey,
    #[clap(value_enum)]
    operation: Operation,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct AppId {
    #[arg(id = "app-id-from-literal", long, value_name = "APP ID")]
    literal: Option<u64>,
    #[arg(id = "app-id-from-file", long, value_name = "APP ID FILE")]
    file: Option<PathBuf>,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct PrivateKey {
    #[arg(id = "private-key-from-literal", long, value_name = "PRIVATE KEY")]
    literal: Option<String>,
    #[arg(id = "private-key-from-file", long, value_name = "PRIVATE KEY FILE")]
    file: Option<PathBuf>,
}

// https://git-scm.com/docs/gitcredentials#_custom_helpers
#[derive(Clone, Debug, ValueEnum)]
enum Operation {
    Get,
    Store,
    Erase,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Ok(layer) = tracing_journald::layer() {
        tracing_subscriber::registry().with(layer).init();
    }

    let opts = Opts::parse();
    tracing::info!(?opts);

    let app_id = match opts.app_id {
        AppId {
            literal: Some(literal),
            file: None,
        } => literal,
        AppId {
            literal: None,
            file: Some(file),
        } => String::from_utf8(fs::read(file).await?)?.parse()?,
        _ => unreachable!(),
    };
    let private_key = match opts.private_key {
        PrivateKey {
            literal: Some(literal),
            file: None,
        } => literal.into_bytes(),
        PrivateKey {
            literal: None,
            file: Some(file),
        } => fs::read(file).await?,
        _ => unreachable!(),
    };

    let octocrab = if let Some(endpoint) = &opts.endpoint {
        Octocrab::builder().base_uri(endpoint)?
    } else {
        Octocrab::builder()
    }
    .app(app_id.into(), EncodingKey::from_rsa_pem(&private_key)?)
    .build()?;

    if let Operation::Get = opts.operation {
        let mut inputs = HashMap::new();
        // https://git-scm.com/docs/git-credential#IOFMT
        let mut stdin = BufReader::new(io::stdin()).lines();
        while let Some(line) = stdin.next_line().await? {
            tracing::debug!(line);
            if let Some((key, value)) = line.split_once('=') {
                inputs.insert(key.to_owned(), value.to_owned());
            }
        }
        tracing::info!(?inputs);

        let (owner, repo) = inputs
            .get("path")
            .ok_or_else(|| anyhow::format_err!("missing path"))?
            .trim_start_matches('/')
            .split_once('/')
            .ok_or_else(|| anyhow::format_err!("invalid path"))?;
        let repo = if let Some((repo, _)) = repo.split_once('/') {
            repo
        } else {
            repo
        }
        .trim_end_matches(".git");
        tracing::info!(owner, repo);

        let installation = octocrab
            .apps()
            .get_repository_installation(owner, repo)
            .await?;
        tracing::info!(installation.id = installation.id.0);

        let (_, token) = octocrab.installation_and_token(installation.id).await?;
        println!("username=x-access-token");
        println!("password={}", token.expose_secret());
    }

    Ok(())
}
