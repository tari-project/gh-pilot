use std::time::Instant;

use futures::{future::join_all, StreamExt, TryFutureExt};
use github_pilot_api::{
    models::SimpleUser,
    provider_traits::{RepoProvider, UserProvider},
    wrappers::GithubHandle,
    GithubProvider,
};
use log::Level::Debug;
use tokio::spawn;
use tracing::{info, instrument};

#[derive(Debug)]
pub struct Report {
    id: GithubHandle,
    name: String,
}

#[instrument]
pub async fn generate_activity_reports(provider: &GithubProvider, ids: Vec<GithubHandle>) -> Result<(), String> {
    let futures = ids
        .into_iter()
        .map(|id| {
            let client = provider.clone();
            info!("Spawning task for {id}");
            spawn(generate_activity_report(client, id)).map_err(|e| format!("Error spawning activity_report task. {e}"))
        })
        .collect::<Vec<_>>();
    let reports = join_all(futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Result<Vec<_>, String>>()?;
    reports.iter().for_each(|r| println!("{r:?}"));
    Ok(())
}

#[instrument]
pub async fn generate_activity_report(provider: GithubProvider, id: GithubHandle) -> Result<Report, String> {
    let start = Instant::now();
    info!("Start: Generating report for {id}");
    let user = fetch_user(&provider, &id).await?;
    let commits = fetch_events_for_user(&provider, &id).await?;
    let dt = start.elapsed().as_secs_f64();
    info!("Finished: Generating report for {id} in {dt:.3} secs");
    Ok(Report {
        id,
        name: user.name.unwrap_or("Not provided".into()),
    })
}

async fn fetch_user(provider: &GithubProvider, id: &GithubHandle) -> Result<SimpleUser, String> {
    let user = provider
        .fetch_details(&id)
        .await
        .map_err(|e| format!("Error fetching {id}. {e}"))?;
    user.ok_or_else(|| format!("User {id} does not exist"))
}

async fn fetch_events_for_user(provider: &GithubProvider, id: &GithubHandle) -> Result<Vec<()>, String> {
    Ok(vec![])
}
