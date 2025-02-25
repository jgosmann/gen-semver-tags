use anyhow::Context;
use clap::Parser;
use gen_semver_tags::gen_semver_tags;
use semver::Version;
use std::io::stdin;

/// Generate a list of semantic versioning tags for a given version.
///
/// Given a `version` and already existing versions via standard input (separated by newlines), a
/// list of tags following [Semantic Versioning](https://semver.org/) is generated and written
/// to standard output. For a version `major.minor.patch`, `major.minor.patch` will always be
/// included. If the version, is the latest minor version of the given major version, `major.minor`
/// will be included. If the version is the latest major version, `major` and the `latest_tags`
/// will be included.
///
/// This tags can be used, for example, to tag Docker images, such that by using just a major
/// (or minor) version, the latest image of that major (or minor) version is pulled.
///
/// Pre-release versions are never considered to be the latest version.
///
/// Build metadata is ignored.
#[derive(Parser)]
#[command(version)]
struct CliOpts {
    /// The version to generate tags for.
    for_version: String,

    /// The latest tags to include.
    #[clap(long, default_value = "latest", value_delimiter = ',')]
    latest_tags: Vec<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let cli_opts = CliOpts::parse();
    let existing_versions = stdin()
        .lines()
        .map(|line| {
            let line = line.with_context(|| "Failed to read line from standard input")?;
            line.parse::<Version>()
                .with_context(|| format!("Failed to parse version '{}'", line))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let tags = gen_semver_tags(
        Version::parse(&cli_opts.for_version)
            .with_context(|| format!("Failed to parse version '{}'", cli_opts.for_version))?,
        &existing_versions,
        &cli_opts
            .latest_tags
            .iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>(),
    );
    for tag in tags {
        println!("{}", tag);
    }
    Ok(())
}
