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

pub fn gen_semver_tags<'a>(
    for_version: Version,
    existing_versions: &[Version],
    latest_tags: &[&'a str],
) -> Vec<Cow<'a, str>> {
    let mut tags = vec![for_version.to_string().into()];

    if for_version < FIRST_MINOR_RELEASE {
        return tags;
    }

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

    if for_version < FIRST_MAJOR_RELEASE {
        return tags;
    }

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

    if existing_versions
        .iter()
        .all(|existing_version| existing_version <= &for_version)
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

    // TODO test with pre-release versions
    // TODO test with build metadata
    // TODO test with more and with no latest tags
}
