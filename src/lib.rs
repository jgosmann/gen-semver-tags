//! Methods to generate tags for semantic versions.
pub use semver::Version;
use semver::{BuildMetadata, Comparator, Op, Prerelease, VersionReq};
use std::borrow::Cow;

const FIRST_MINOR_RELEASE: Version = Version {
    major: 0,
    minor: 1,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};

const FIRST_MAJOR_RELEASE: Version = Version {
    major: 1,
    minor: 0,
    patch: 0,
    pre: Prerelease::EMPTY,
    build: BuildMetadata::EMPTY,
};

/// Generate a list of semantic versioning tags for a given version.
///
/// Given a `for_version` and already `existing_versions`, a list of tags following
/// [Semantic Versioning](https://semver.org/) is generated. For a version `major.minor.patch`,
/// `major.minor.patch` will always be included. If the version, is the latest minor version of
/// the given major version, `major.minor` will be included. If the version is the latest major
/// version, `major` and the `latest_tags` will be included.
///
/// This tags can be used, for example, to tag Docker images, such that by using just a major
/// (or minor) version, the latest image of that major (or minor) version is pulled.
///
/// Pre-release versions are never considered to be the latest version.
///
/// Build metadata is ignored.
pub fn gen_semver_tags<'a>(
    for_version: Version,
    existing_versions: &[Version],
    latest_tags: &[&'a str],
) -> Vec<Cow<'a, str>> {
    let for_version = Version {
        build: BuildMetadata::EMPTY,
        ..for_version
    };
    let mut tags = vec![for_version.to_string().into()];

    if !for_version.pre.is_empty() {
        return tags;
    }

    let existing_versions = existing_versions
        .iter()
        .filter(|&v| v.pre.is_empty())
        .collect::<Vec<_>>();

    if for_version >= FIRST_MINOR_RELEASE {
        let gt_for_version = Comparator {
            op: Op::Greater,
            major: for_version.major,
            minor: Some(for_version.minor),
            patch: Some(for_version.patch),
            pre: for_version.pre.clone(),
        };

        let minor_version_req = VersionReq {
            comparators: vec![
                gt_for_version.clone(),
                Comparator {
                    op: Op::Less,
                    major: for_version.major,
                    minor: Some(for_version.minor + 1),
                    patch: None,
                    pre: Prerelease::EMPTY,
                },
            ],
        };
        if !existing_versions
            .iter()
            .any(|existing_version| minor_version_req.matches(existing_version))
        {
            tags.push(format!("{}.{}", for_version.major, for_version.minor).into());
        }

        if for_version >= FIRST_MAJOR_RELEASE {
            let major_version_req = VersionReq {
                comparators: vec![
                    gt_for_version,
                    Comparator {
                        op: Op::Less,
                        major: for_version.major + 1,
                        minor: None,
                        patch: None,
                        pre: Prerelease::EMPTY,
                    },
                ],
            };
            if !existing_versions
                .iter()
                .any(|existing_version| major_version_req.matches(existing_version))
            {
                tags.push(format!("{}", for_version.major).into());
            }
        }
    }

    if existing_versions
        .iter()
        .all(|&existing_version| existing_version <= &for_version)
    {
        tags.extend(latest_tags.iter().copied().map(Cow::Borrowed));
    }

    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_semver_tags() {
        let existing_versions: Vec<Version> = [
            "0.0.1", "0.0.2", "0.1.0", "0.1.1", "0.2.0", "1.0.0", "1.0.1", "1.1.0", "1.1.1",
            "2.0.0", "2.0.1", "2.1.0", "2.1.1",
        ]
        .into_iter()
        .map(Version::parse)
        .collect::<Result<_, _>>()
        .unwrap();
        let latest_tags = vec!["latest"];

        for (version, expected_tags) in [
            ("0.0.1", vec!["0.0.1"]),
            ("0.0.2", vec!["0.0.2"]),
            ("0.0.3", vec!["0.0.3"]),
            ("0.1.0", vec!["0.1.0"]),
            ("0.1.1", vec!["0.1.1", "0.1"]),
            ("0.1.2", vec!["0.1.2", "0.1"]),
            ("0.2.0", vec!["0.2.0", "0.2"]),
            ("0.3.0", vec!["0.3.0", "0.3"]),
            ("1.0.0", vec!["1.0.0"]),
            ("1.0.1", vec!["1.0.1", "1.0"]),
            ("1.0.2", vec!["1.0.2", "1.0"]),
            ("1.1.0", vec!["1.1.0"]),
            ("1.1.1", vec!["1.1.1", "1.1", "1"]),
            ("1.1.2", vec!["1.1.2", "1.1", "1"]),
            ("1.2.0", vec!["1.2.0", "1.2", "1"]),
            ("2.0.0", vec!["2.0.0"]),
            ("2.0.1", vec!["2.0.1", "2.0"]),
            ("2.1.0", vec!["2.1.0"]),
            ("2.1.1", vec!["2.1.1", "2.1", "2", "latest"]),
            ("2.1.2", vec!["2.1.2", "2.1", "2", "latest"]),
            ("2.2.0", vec!["2.2.0", "2.2", "2", "latest"]),
            ("3.0.0", vec!["3.0.0", "3.0", "3", "latest"]),
        ] {
            assert_eq!(
                gen_semver_tags(
                    Version::parse(version).unwrap(),
                    &existing_versions,
                    &latest_tags
                ),
                expected_tags,
                "for version {}",
                version
            );
        }
    }

    #[test]
    fn test_applies_latest_tags() {
        for (version, existing_versions) in [
            ("0.0.1", vec!["0.0.1"]),
            ("0.0.2", vec!["0.0.1"]),
            ("0.1.0", vec!["0.1.0"]),
            ("0.1.1", vec!["0.1.0"]),
            ("0.2.0", vec!["0.2.0"]),
            ("1.0.0", vec!["1.0.0"]),
            ("1.0.1", vec!["1.0.0"]),
            ("1.1.0", vec!["1.0.0"]),
            ("2.0.0", vec!["1.0.0"]),
        ] {
            let existing_versions = existing_versions
                .into_iter()
                .map(Version::parse)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            assert!(
                gen_semver_tags(
                    Version::parse(version).unwrap(),
                    &existing_versions,
                    &["latest"]
                )
                .contains(&Cow::from("latest")),
                "for version {}",
                version
            );
        }
    }

    #[test]
    fn test_applies_multiple_latest_tags() {
        let tags = gen_semver_tags(Version::parse("1.2.3").unwrap(), &[], &["latest", "stable"]);
        assert!(tags.contains(&Cow::from("latest")));
        assert!(tags.contains(&Cow::from("stable")));
    }

    #[test]
    fn test_existing_prereleases_are_ignored() {
        let latest_tags = vec!["latest"];
        for (version, existing_versions, expected_tags) in [
            ("0.0.1", vec!["0.0.1", "0.0.2-pre"], vec!["0.0.1", "latest"]),
            ("0.0.2", vec!["0.0.1", "0.0.2-pre"], vec!["0.0.2", "latest"]),
            (
                "0.1.0",
                vec!["0.1.0", "0.1.1-pre", "0.2.0-pre"],
                vec!["0.1.0", "0.1", "latest"],
            ),
            (
                "0.1.1",
                vec!["0.1.0", "0.1.1-pre", "0.2.0-pre"],
                vec!["0.1.1", "0.1", "latest"],
            ),
            (
                "0.2.0",
                vec!["0.1.0", "0.1.1-pre", "0.2.0-pre"],
                vec!["0.2.0", "0.2", "latest"],
            ),
            (
                "1.0.0",
                vec!["1.0.0", "1.0.1-pre", "1.1.0-pre", "2.0.0-pre"],
                vec!["1.0.0", "1.0", "1", "latest"],
            ),
            (
                "1.0.1",
                vec!["1.0.0", "1.0.1-pre", "1.1.0-pre", "2.0.0-pre"],
                vec!["1.0.1", "1.0", "1", "latest"],
            ),
            (
                "1.1.0",
                vec!["1.0.0", "1.0.1-pre", "1.1.0-pre", "2.0.0-pre"],
                vec!["1.1.0", "1.1", "1", "latest"],
            ),
            (
                "2.0.0",
                vec!["1.0.0", "1.0.1-pre", "1.1.0-pre", "2.0.0-pre"],
                vec!["2.0.0", "2.0", "2", "latest"],
            ),
        ] {
            let existing_versions = existing_versions
                .into_iter()
                .map(Version::parse)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            assert_eq!(
                gen_semver_tags(
                    Version::parse(version).unwrap(),
                    &existing_versions,
                    &latest_tags
                ),
                expected_tags,
                "for version {}",
                version
            );
        }
    }

    #[test]
    fn test_prelease_version_does_not_generate_additional_tags() {
        for version in ["0.0.1-pre", "0.1.0-pre", "1.0.0-pre"] {
            assert_eq!(
                gen_semver_tags(Version::parse(version).unwrap(), &[], &["latest"]),
                [version],
                "for version {}",
                version
            );
        }
    }

    #[test]
    fn test_ignores_build_metadata() {
        for (version, expected) in [
            ("0.0.1+build", "0.0.1"),
            ("0.1.0+build", "0.1.0"),
            ("1.0.0+build", "1.0.0"),
        ] {
            assert_eq!(
                gen_semver_tags(
                    Version::parse(version).unwrap(),
                    &["0.1.1", "1.0.1"]
                        .into_iter()
                        .map(Version::parse)
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap(),
                    &[]
                ),
                [expected],
                "for version {}",
                version
            );
        }
    }
}
