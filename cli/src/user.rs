use comfy_table::{presets::UTF8_BORDERS_ONLY, Cell, Color, ContentArrangement, Row, Table};
use github_pilot_api::{models::SimpleUser, provider_traits::UserProvider, wrappers::GithubHandle};
use log::*;

pub async fn run_user_cmd<S: AsRef<str>>(provider: &dyn UserProvider, profile: S) -> Result<(), ()> {
    debug!("Fetching user..{}", profile.as_ref());
    let handle = GithubHandle::from(profile.as_ref());
    let details = provider.fetch_details(&handle).await.map_err(|_| ())?;
    match details {
        Some(p) => pretty_print(&p),
        None => info!("User {} was not found", profile.as_ref()),
    }
    Ok(())
}

fn pretty_print(user: &SimpleUser) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic);
    let mut name_row = Row::new();
    name_row
        .add_cell(cc(Color::Green, "User name"))
        .add_cell(cc(Color::Green, user.login.as_str()));
    table
        .add_row(name_row)
        .add_row(["Type", user.name.as_deref().unwrap_or("Unavailable")])
        .add_row(["URL", user.url.as_ref()]);
    println!("{table}");
}

fn cc(color: Color, val: &str) -> Cell {
    Cell::new(val).fg(color)
}
