use github_pilot_api::{graphql::org_activity::OrgActivity, provider_traits::OrganizationProvider};

use crate::pilot_command::activity_report::ActivityReports;

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
    let report = ActivityReports::from(activity);
    let mut std_out = std::io::stdout();
    if let Err(e) = report.write_csv(&mut std_out) {
        eprintln!("Error writing report: {e}");
    }
}
