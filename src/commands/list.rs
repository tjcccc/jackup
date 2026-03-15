use anyhow::{anyhow, Context};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::cli::{ListArgs, SortField};
use crate::core::config::{Config, Source};
use crate::core::format::truncate;
use crate::core::paths::get_config_path;

const DEFAULT_PATH_WIDTH: usize = 60;

pub fn run(args: ListArgs) -> anyhow::Result<()> {
    let config_path = get_config_path().context("Get config file")?;
    let config = Config::load(
        config_path
            .to_str()
            .ok_or_else(|| anyhow!("Failed to load configuration."))?,
    )?;

    let mut sources: Vec<&Source> = config.sources.iter().collect();
    sort_sources(&mut sources, args.sort);

    if sources.is_empty() {
        println!("(no sources)");
        return Ok(());
    }

    if args.verbose {
        print_verbose_table(&sources);
    } else {
        print_default_table(&sources);
    }
    Ok(())
}

fn sort_sources(sources: &mut [&Source], sort: SortField) {
    match sort {
        SortField::Name => sources.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        SortField::Created => sources.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
        SortField::Updated => sources.sort_by(|a, b| a.updated_at.cmp(&b.updated_at)),
    }
}

fn print_default_table(sources: &[&Source]) {
    let mut name_w = "Name".len();
    let path_w = DEFAULT_PATH_WIDTH;

    for s in sources {
        name_w = name_w.max(s.name.len());
    }

    println!(
        "{:<name_w$}  {:<path_w$}  {}",
        "Name",
        "Path",
        "Enabled",
        name_w = name_w,
        path_w = path_w
    );
    println!("{}", "-".repeat(name_w + path_w + "Enabled".len() + 4));

    for s in sources {
        let path = truncate(&s.path.display().to_string(), path_w);
        println!(
            "{:<name_w$}  {:<path_w$}  {}",
            s.name,
            path,
            s.enabled,
            name_w = name_w,
            path_w = path_w
        );
    }
}

fn print_verbose_table(sources: &[&Source]) {
    let rows: Vec<[String; 8]> = sources
        .iter()
        .map(|s| {
            [
                s.id.clone(),
                s.name.clone(),
                s.path.display().to_string(),
                s.enabled.to_string(),
                s.follow_symlinks.unwrap_or(false).to_string(),
                if s.exclude.is_empty() {
                    "-".to_string()
                } else {
                    s.exclude.join(", ")
                },
                friendly_time(&s.created_at),
                friendly_time(&s.updated_at),
            ]
        })
        .collect();

    let headers = [
        "ID",
        "Name",
        "Path",
        "Enabled",
        "FollowSymlinks",
        "Exclude",
        "CreatedAt",
        "UpdatedAt",
    ];
    let mut widths = headers.map(|h| h.len());

    for row in &rows {
        for (i, value) in row.iter().enumerate() {
            widths[i] = widths[i].max(value.len());
        }
    }

    println!(
        "{:<w0$}  {:<w1$}  {:<w2$}  {:<w3$}  {:<w4$}  {:<w5$}  {:<w6$}  {:<w7$}",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        headers[6],
        headers[7],
        w0 = widths[0],
        w1 = widths[1],
        w2 = widths[2],
        w3 = widths[3],
        w4 = widths[4],
        w5 = widths[5],
        w6 = widths[6],
        w7 = widths[7]
    );
    println!("{}", "-".repeat(widths.iter().sum::<usize>() + 14));

    for row in rows {
        println!(
            "{:<w0$}  {:<w1$}  {:<w2$}  {:<w3$}  {:<w4$}  {:<w5$}  {:<w6$}  {:<w7$}",
            row[0],
            row[1],
            row[2],
            row[3],
            row[4],
            row[5],
            row[6],
            row[7],
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2],
            w3 = widths[3],
            w4 = widths[4],
            w5 = widths[5],
            w6 = widths[6],
            w7 = widths[7]
        );
    }
}

fn friendly_time(value: &Option<String>) -> String {
    match value {
        Some(raw) => {
            if let Ok(ts) = OffsetDateTime::parse(raw, &Rfc3339) {
                format!(
                    "{} {:02}:{:02}:{:02} UTC",
                    ts.date(),
                    ts.hour(),
                    ts.minute(),
                    ts.second()
                )
            } else {
                raw.clone()
            }
        }
        None => "-".to_string(),
    }
}

