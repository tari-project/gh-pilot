use std::time::Instant;

use futures::{future::join_all, TryFutureExt};
use github_pilot_api::{
    models::{DateTime, Event, SimpleUser},
    provider_traits::UserProvider,
    wrappers::GithubHandle,
    GithubProvider,
};
use tokio::spawn;
use tracing::{info, instrument};

#[derive(Debug)]
pub struct Report {
    pub id: GithubHandle,
    pub name: String,
    pub since: DateTime,
    pub total_events: usize,
}

#[instrument]
pub async fn generate_activity_reports(
    provider: &GithubProvider,
    ids: Vec<GithubHandle>,
    since: DateTime,
) -> Result<(), String> {
    let futures = ids
        .into_iter()
        .map(|id| {
            let client = provider.clone();
            let ts = since.clone();
            info!("Spawning task for {id}");
            spawn(generate_activity_report(client, id, ts))
                .map_err(|e| format!("Error spawning activity_report task. {e}"))
        })
        .collect::<Vec<_>>();
    let reports = join_all(futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Result<Vec<_>, String>>()?;
    reports
        .iter()
        .for_each(|r| println!("{}({}): {} since {}", r.name, r.id, r.total_events, r.since));
    Ok(())
}

#[instrument]
pub async fn generate_activity_report(
    provider: GithubProvider,
    id: GithubHandle,
    since: DateTime,
) -> Result<Report, String> {
    let start = Instant::now();
    info!("Start: Generating report for {id}");
    let user = fetch_user(&provider, &id).await?;
    let events = fetch_events_for_user(&provider, &id, since.clone()).await?;
    let dt = start.elapsed().as_secs_f64();
    info!("Finished: Generating report for {id} in {dt:.3} secs");
    Ok(Report {
        id,
        name: user.name.unwrap_or_else(|| "Not provided".into()),
        since,
        total_events: events.len(),
    })
}

async fn fetch_user(provider: &GithubProvider, id: &GithubHandle) -> Result<SimpleUser, String> {
    let user = provider
        .fetch_details(id)
        .await
        .map_err(|e| format!("Error fetching {id}. {e}"))?;
    user.ok_or_else(|| format!("User {id} does not exist"))
}

async fn fetch_events_for_user(
    provider: &GithubProvider,
    id: &GithubHandle,
    since: DateTime,
) -> Result<Vec<Event>, String> {
    let events = provider
        .fetch_events(id, since, true)
        .await
        .map_err(|e| format!("Error fetching events for {id}. {e}"))?;
    Ok(events)
}
