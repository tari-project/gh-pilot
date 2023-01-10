use github_pilot_api::{graphql::org_activity::OrgActivity, provider_traits::OrganizationProvider};

#[derive(Debug)]
pub struct DateRange {
    from: String,
    to: String,
}

impl DateRange {
    pub fn new(from: &str, to: &str) -> Self {
        DateRange {
            from: from.into(),
            to: to.into(),
        }
    }
}

#[derive(Debug)]
pub enum OrganizationCmd {
    Activity(String, DateRange),
}

impl OrganizationCmd {
    pub async fn execute(self, provider: &dyn OrganizationProvider) -> Result<(), String> {
        match self {
            OrganizationCmd::Activity(owner, dates) => {
                match provider
                    .fetch_activities(owner.as_str(), dates.from.as_str(), dates.to.as_str())
                    .await
                {
                    Ok(results) => {
                        print_org_activity(results);
                        Ok(())
                    },
                    Err(e) => Err(format!("📈 Organization Activity query failed. {e}")),
                }
            },
        }
    }
}

fn print_org_activity(activity: OrgActivity) {
    // let mut out = std::fs::File::create("./org_activity.json").unwrap();
    // let json = serde_json::to_string(&activity).unwrap();
    // let _ = out.write(json.as_bytes()).unwrap();
    let OrgActivity { issues, pull_requests } = activity;
    issues.into_iter().for_each(|iss| {
        println!("Issue {}#{} - {}", iss.repository, iss.number, iss.title);
        println!(
            "Created: {}. By {}. {} Comments\n",
            iss.created_at,
            iss.author.display_name(),
            iss.comments.total_comments
        )
    });
    pull_requests.into_iter().for_each(|pr| {
        println!(
            "PR {}#{} - {}",
            pr.base_repository.as_deref().unwrap_or("None"),
            pr.number,
            pr.title
        );
        println!(
            "Created: {}. By {}. {} Files changed. +{} -{}\n",
            pr.created_at,
            pr.author.display_name(),
            pr.changed_file_count,
            pr.total_additions,
            pr.total_deletions
        );
    });
}
