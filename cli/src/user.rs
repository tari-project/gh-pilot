use crate::Context;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Cell, Color, ContentArrangement, Row, Table};
use gh_pilot::models::{GithubHandle, UserDetails};

pub async fn run_user_cmd<S: AsRef<str>>(ctx: &Context<'_>, profile: S) -> Result<(), ()> {
    if let Some(provider) = ctx.user_provider() {
        println!("Fetching user..{}", profile.as_ref());
        let handle = GithubHandle::from(profile.as_ref());
        let details = provider.fetch_details(&handle).await.map_err(|_| ())?;
        match details {
            Some(p) => pretty_print(&p),
            None => println!("User {} was not found", profile.as_ref()),
        }
    } else {
        println!("No user provider installed");
    }
    Ok(())
}

fn pretty_print(user: &UserDetails) {
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
        .add_row(&["Name", user.name.as_str()])
        .add_row(&["URL", user.url.as_str()]);
    println!("{table}");
}

fn cc(color: Color, val: &str) -> Cell {
    Cell::new(val).fg(color)
}
