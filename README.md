# gen-semver-tags

Generate a list of semantic versioning tags for a given version.

These can, for example, be used to tag Docker images to allow pulling the latest major or minor version.

## Usage examples

```shell
$ echo "1.0.0" | gen-semver-tags 1.1.0
1.1.0
1.1
1
latest

$ echo "1.0.0" | gen-semver-tags 1.1.0
1.1.0
1.1
1
latest

$ echo "1.0.0\n1.1.0" | gen-semver-tags 1.0.1
1.0.1
1.0

$ echo "1.0.0" | gen-semver-tags --latest-tags latest,stable 1.1.0
1.1.0
1.1
1
latest
stable

$ echo "1.0.0" | gen-semver-tags --latest-tags '' 1.1.0
1.1.0
1.1
1
```

## Docker image

The tool is provided as Docker image [`jgosmann/gen-semver-tags`](https://hub.docker.com/r/jgosmann/gen-semver-tags).
It uses SemVer tags itself.

You can find an example how to use it in a GitHub Actions workflow in the
[
`.github/workflows/docker-image.yml`](https://github.com/jgosmann/gen-semver-tags/blob/5f9c855514faf7d1d8189e3bc35c8109e7bc43ed/.github/workflows/docker.yml#L54-L65)
file.

## Details

Given a `version` and already existing versions via standard input (separated by newlines), a
list of tags following [Semantic Versioning](https://semver.org/) is generated and written
to standard output. For a version `major.minor.patch`, `major.minor.patch` will always be
included. If the version, is the latest minor version of the given major version, `major.minor`
will be included. If the version is the latest major version, `major` and the `latest_tags`
will be included.

This tags can be used, for example, to tag Docker images, such that by using just a major
(or minor) version, the latest image of that major (or minor) version is pulled.

Pre-release versions are never considered to be the latest version.

Build metadata is ignored.
