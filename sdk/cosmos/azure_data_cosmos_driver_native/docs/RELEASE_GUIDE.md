<!--
Copyright (c) Microsoft Corporation. All rights reserved.
Licensed under the MIT License.
-->

# Native driver release guide

Use this runbook to publish `azure_data_cosmos_driver_native` version `X.Y.Z`.
The source tag is immutable and identifies the exact reviewed release commit.

## 1. Prepare the release

Before opening the release-preparation pull request:

- `Cargo.toml`, `cosmos_version()`, and `AZURECOSMOSDRIVER_H_VERSION` must report
  `X.Y.Z`;
- `CHANGELOG.md` must contain `## X.Y.Z (Unreleased)`;
- generated outputs must already match their source; do not edit generated
  files directly; and
- all required CI and native release validation must pass.

In the release-preparation pull request, change only the matching heading:

```diff
-## X.Y.Z (Unreleased)
+## X.Y.Z (YYYY-MM-DD)
```

Use the UTC release date. Do not change another version's section.

## 2. Identify the release commit

After the release-preparation pull request merges, obtain its exact merge
commit from GitHub. Do not tag the pull-request head or a later `main` commit.

```powershell
$version = 'X.Y.Z'
$releasePr = '<RELEASE_PREPARATION_PR_NUMBER>'
$releaseCommit = gh pr view $releasePr `
  --repo Azure/azure-sdk-for-rust `
  --json mergeCommit `
  --jq '.mergeCommit.oid'

git fetch origin main --tags
git cat-file -e "$releaseCommit^{commit}"
if ($LASTEXITCODE -ne 0) { throw 'Release merge commit was not found.' }

git merge-base --is-ancestor $releaseCommit origin/main
if ($LASTEXITCODE -ne 0) { throw 'Release commit is not on origin/main.' }
```

Inspect that commit and confirm the package, header, and dated changelog entry
all use `X.Y.Z`.

## 3. Create the source tag

Create one annotated tag on the verified merge commit:

```powershell
$tag = "azure_data_cosmos_driver_native@$version"
$existingCommit = git rev-parse -q --verify "refs/tags/$tag^{commit}"
if ($LASTEXITCODE -eq 0) {
  if ($existingCommit -ne $releaseCommit) {
    throw "Tag $tag already points to a different commit. Stop the release."
  }
  throw "Tag $tag already exists. Verify the existing release; do not recreate it."
}

git tag -a $tag $releaseCommit -m "Release $tag"

$tagType = git cat-file -t "refs/tags/$tag"
$tagCommit = git rev-list -n 1 $tag
if ($tagType -ne 'tag' -or $tagCommit -ne $releaseCommit) {
  throw 'Annotated tag verification failed.'
}

git push origin "refs/tags/$tag:refs/tags/$tag"
```

Push only that tag. Release tags must never be moved, reused, force-pushed, or
deleted. A tag that already exists on another commit is a hard stop.

## 4. Run the publish pipeline

Run the registered Azure DevOps native publish pipeline with this exact source
ref:

```text
refs/tags/azure_data_cosmos_driver_native@X.Y.Z
```

In the Azure DevOps UI, select the registered native pipeline, choose **Run
pipeline**, and select or enter the tag ref above. With Azure CLI, queue the
same registered pipeline conceptually as:

```powershell
az pipelines run `
  --id <REGISTERED_NATIVE_PIPELINE_ID> `
  --branch "refs/tags/$tag"
```

An equivalent REST queue request sets `sourceBranch` to the same full tag ref.
Use the registered native pipeline's actual name or ID; this repository does
not define that registration.

Azure DevOps derives `Build.SourceBranch` from the selected tag ref and
`Build.SourceVersion` from its target commit. Do not override either value.
Selecting `main`, or running `/azp run` on a pull request, validates the build
but does not publish the downstream release.

## 5. Verify the run

The pipeline must fail closed unless:

- Cargo, header, changelog, tag, and source commit agree;
- the tag is annotated and points to `Build.SourceVersion`;
- the FFI change matches the pre-`1.0.0` SemVer rule;
- every required target builds and links; and
- hashes, provenance, signed SPDX, and the evidence allowlist validate.

A successful run produces the native artifacts and signed release evidence,
then opens a draft pull request in `Azure/azure-cosmos-driver`.

Verify:

- every expected platform module is present;
- artifact hashes match `SHA256SUMS` and provenance;
- signed evidence is present without operational logs;
- the downstream draft pull request references the tagged source commit; and
- no unrelated downstream paths changed.

## Retry and failure rules

Rerun the same tag only when the tagged source is unchanged, such as after a
transient agent, network, or service failure.

If source, changelog, generated output, dependencies, or build inputs must
change, stop. Merge a new release-preparation pull request, increment the native
version, and create a new annotated tag. Never repair a release by moving or
replacing an existing tag.
