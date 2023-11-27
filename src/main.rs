use clap::{ArgEnum, Parser};
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
    #[clap(long)]
    app_id: u64,
    #[clap(long)]
    private_key: PathBuf,
    #[clap(arg_enum)]
    operation: Operation,
}

// https://git-scm.com/docs/gitcredentials#_custom_helpers
#[derive(Clone, Debug, ArgEnum)]
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

    let octocrab = if let Some(endpoint) = &opts.endpoint {
        Octocrab::builder().base_uri(endpoint)?
    } else {
        Octocrab::builder()
    }
    .app(
        opts.app_id.into(),
        EncodingKey::from_rsa_pem(&fs::read(&opts.private_key).await?)?,
    )
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
            .split_once('/')
            .ok_or_else(|| anyhow::format_err!("invalid path"))?;
        let repo = repo.trim_end_matches(".git");
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
