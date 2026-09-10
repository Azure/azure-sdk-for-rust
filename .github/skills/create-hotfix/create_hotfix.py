#!/usr/bin/env python3

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

UPSTREAM_URL = "https://github.com/Azure/azure-sdk-for-rust"
STABLE_VERSION = re.compile(r"(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)")


class CommandError(RuntimeError):
    pass


def run_process(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        args,
        check=False,
        encoding="utf-8",
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def command_error(
    args: tuple[str, ...], result: subprocess.CompletedProcess[str]
) -> CommandError:
    detail = result.stderr.strip() or result.stdout.strip()
    return CommandError(f"{' '.join(args)} failed: {detail}")


def run(*args: str) -> str:
    result = run_process(*args)
    if result.returncode:
        raise command_error(args, result)
    return result.stdout


def repository_root() -> Path:
    return Path(run("git", "rev-parse", "--show-toplevel").strip()).resolve()


def workspace_crates(root: Path) -> list[dict[str, str]]:
    metadata = json.loads(
        run("cargo", "metadata", "--no-deps", "--format-version", "1")
    )
    workspace_members = set(metadata["workspace_members"])
    crates = []

    for package in metadata["packages"]:
        if package["id"] not in workspace_members or package["publish"] == []:
            continue

        manifest = Path(package["manifest_path"]).resolve()
        crate_dir = manifest.parent
        try:
            relative_dir = crate_dir.relative_to(root)
        except ValueError:
            continue

        crates.append(
            {
                "name": package["name"],
                "path": relative_dir.as_posix(),
                "version": package["version"],
            }
        )

    return sorted(crates, key=lambda crate: (crate["name"], crate["path"]))


def find_crate(root: Path, crate_name: str) -> dict[str, str]:
    matches = [crate for crate in workspace_crates(root) if crate["name"] == crate_name]
    if not matches:
        raise CommandError(f"crate is not a workspace member: {crate_name}")
    if len(matches) > 1:
        paths = ", ".join(crate["path"] for crate in matches)
        raise CommandError(f"crate name is ambiguous ({crate_name}): {paths}")
    return matches[0]


def parse_stable_tag(crate_name: str, tag: str) -> tuple[int, int, int] | None:
    prefix = f"{crate_name}@"
    if not tag.startswith(prefix):
        return None

    match = STABLE_VERSION.fullmatch(tag[len(prefix) :])
    if not match:
        return None

    return tuple(int(part) for part in match.groups())


def resolve_base_tag(crate_name: str, tags: list[str]) -> tuple[str, str]:
    stable_tags = []
    for tag in tags:
        version = parse_stable_tag(crate_name, tag)
        if version is not None:
            stable_tags.append((version, tag))

    if not stable_tags:
        raise CommandError(
            f"no stable release tag found matching {crate_name}@<major>.<minor>.<patch>"
        )

    version, tag = max(stable_tags)
    next_version = f"{version[0]}.{version[1]}.{version[2] + 1}"
    return tag, next_version


def upstream_tags(crate_name: str) -> list[str]:
    output = run(
        "git",
        "ls-remote",
        "--refs",
        "--tags",
        UPSTREAM_URL,
        f"refs/tags/{crate_name}@*",
    )
    return [
        line.split("\t", 1)[1].removeprefix("refs/tags/")
        for line in output.splitlines()
    ]


def upstream_main_commit() -> str:
    output = run(
        "git", "ls-remote", "--heads", UPSTREAM_URL, "refs/heads/main"
    ).strip()
    if not output:
        raise CommandError(f"main branch not found at {UPSTREAM_URL}")
    return output.split("\t", 1)[0]


def current_branch() -> str:
    branch = run("git", "branch", "--show-current").strip()
    if not branch:
        raise CommandError("HEAD is detached")
    return branch


def worktree_is_clean() -> bool:
    return not run("git", "status", "--porcelain").strip()


def git_path(name: str) -> Path:
    return Path(run("git", "rev-parse", "--git-path", name).strip())


def cherry_pick_in_progress() -> bool:
    return git_path("CHERRY_PICK_HEAD").exists()


def lock_conflict_marker() -> Path:
    return git_path("create-hotfix-cargo-lock-conflict")


def conflicted_files() -> list[str]:
    output = run("git", "diff", "--name-only", "--diff-filter=U", "-z")
    return sorted(path for path in output.split("\0") if path)


def resolve_cargo_lock_conflict() -> list[str]:
    conflicts = conflicted_files()
    if "Cargo.lock" not in conflicts:
        return conflicts

    run("git", "checkout", "--ours", "--", "Cargo.lock")
    run("git", "add", "Cargo.lock")
    lock_conflict_marker().touch()
    return [path for path in conflicts if path != "Cargo.lock"]


def regenerate_cargo_lock() -> None:
    marker = lock_conflict_marker()
    if not marker.exists():
        return

    run("cargo", "generate-lockfile", "--manifest-path", "Cargo.toml")
    run("git", "add", "Cargo.lock")
    marker.unlink()


def is_ancestor(ancestor: str, descendant: str) -> bool:
    result = subprocess.run(
        ("git", "merge-base", "--is-ancestor", ancestor, descendant),
        check=False,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        encoding="utf-8",
    )
    if result.returncode == 0:
        return True
    if result.returncode == 1:
        return False
    raise CommandError(
        f"git merge-base --is-ancestor failed: {result.stderr.strip()}"
    )


def ensure_hotfix_branch(crate_name: str, version: str, base_tag: str) -> str:
    branch = current_branch()
    if branch.startswith("hotfix/"):
        return branch

    if not worktree_is_clean():
        raise CommandError(
            "working tree must be clean before creating or switching hotfix branches"
        )

    branch = f"hotfix/{crate_name}-{version}"
    existing = run("git", "branch", "--list", branch).strip()
    if existing:
        if not is_ancestor(base_tag, branch):
            raise CommandError(
                f"existing branch {branch} is not based on {base_tag}"
            )
        run("git", "switch", branch)
    else:
        run("git", "switch", "--create", branch, base_tag)
    return branch


def changed_files(commit: str, crate_path: str) -> list[dict[str, object]]:
    name_status = run(
        "git",
        "diff-tree",
        "--no-commit-id",
        "--name-status",
        "--no-renames",
        "-r",
        commit,
        "--",
        crate_path,
    )
    numstat = run(
        "git",
        "diff-tree",
        "--no-commit-id",
        "--numstat",
        "--no-renames",
        "-r",
        commit,
        "--",
        crate_path,
    )

    stats = {}
    for line in numstat.splitlines():
        additions, deletions, path = line.split("\t", 2)
        stats[path] = {
            "additions": None if additions == "-" else int(additions),
            "deletions": None if deletions == "-" else int(deletions),
        }

    files = []
    for line in name_status.splitlines():
        parts = line.split("\t")
        status = parts[0]
        path = parts[-1]
        file_change = {"path": path, "status": status}
        file_change.update(stats.get(path, {"additions": None, "deletions": None}))
        files.append(file_change)
    return files


def summarize_files(files: list[dict[str, object]]) -> str:
    summaries = []
    for file_change in files:
        additions = file_change["additions"]
        deletions = file_change["deletions"]
        if additions is None or deletions is None:
            stats = "binary"
        else:
            stats = f"+{additions}/-{deletions}"
        summaries.append(
            f"{file_change['status']} {file_change['path']} ({stats})"
        )
    return ", ".join(summaries)


def candidate_commits(crate_path: str) -> list[dict[str, object]]:
    output = run(
        "git",
        "log",
        "--no-color",
        "--reverse",
        "--no-merges",
        "--cherry-pick",
        "--right-only",
        "--format=%H%x09%s",
        "HEAD...FETCH_HEAD",
        "--",
        crate_path,
    )
    commits = []
    for line in output.splitlines():
        full_sha, subject = line.split("\t", 1)
        files = changed_files(full_sha, crate_path)
        commits.append(
            {
                "sha": full_sha,
                "summary": f"{full_sha[:12]} {subject}: {summarize_files(files)}",
            }
        )
    return commits


def list_crates(_: argparse.Namespace) -> dict[str, object]:
    root = repository_root()
    return {"crates": workspace_crates(root)}


def prepare(args: argparse.Namespace) -> dict[str, object]:
    root = repository_root()
    crate = find_crate(root, args.crate)

    run("git", "fetch", UPSTREAM_URL, "--tags")
    tags = upstream_tags(args.crate)
    base_tag, patch_version = resolve_base_tag(args.crate, tags)
    branch = ensure_hotfix_branch(args.crate, patch_version, base_tag)

    run("git", "fetch", UPSTREAM_URL)
    if run("git", "rev-parse", "FETCH_HEAD").strip() != upstream_main_commit():
        raise CommandError(f"{UPSTREAM_URL} HEAD does not point to main")
    commits = candidate_commits(crate["path"])
    return {
        "crate": crate,
        "base_tag": base_tag,
        "patch_version": patch_version,
        "branch": branch,
        "candidates": commits,
    }


def advance_cherry_pick(branch: str) -> dict[str, object]:
    while True:
        conflicts = resolve_cargo_lock_conflict()
        if conflicts:
            return {"status": "waiting", "branch": branch, "conflicts": conflicts}

        regenerate_cargo_lock()
        command = ("git", "-c", "core.editor=true", "cherry-pick", "--continue")
        result = run_process(*command)
        if result.returncode == 0:
            return {"status": "complete", "branch": branch}
        if not cherry_pick_in_progress():
            raise command_error(command, result)
        if not conflicted_files():
            raise command_error(command, result)


def cherry_pick(args: argparse.Namespace) -> dict[str, object]:
    branch = current_branch()
    if not branch.startswith("hotfix/"):
        raise CommandError("cherry-pick must run from a hotfix/ branch")
    if cherry_pick_in_progress():
        raise CommandError(
            "a cherry-pick is already in progress; use the continue command"
        )

    lock_conflict_marker().unlink(missing_ok=True)
    command = ("git", "cherry-pick", *args.commits)
    result = run_process(*command)
    if result.returncode == 0:
        return {
            "status": "complete",
            "branch": branch,
            "cherry_picked": args.commits,
        }
    if not cherry_pick_in_progress():
        raise command_error(command, result)
    if not conflicted_files():
        raise command_error(command, result)
    return advance_cherry_pick(branch)


def continue_cherry_pick(_: argparse.Namespace) -> dict[str, object]:
    branch = current_branch()
    if not branch.startswith("hotfix/"):
        raise CommandError("continue must run from a hotfix/ branch")
    if not cherry_pick_in_progress():
        lock_conflict_marker().unlink(missing_ok=True)
        raise CommandError("no cherry-pick is in progress")
    return advance_cherry_pick(branch)


def create_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Prepare a crate hotfix branch from its latest stable release tag."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    list_parser = subparsers.add_parser(
        "list-crates", help="List workspace crates as JSON."
    )
    list_parser.set_defaults(handler=list_crates)

    prepare_parser = subparsers.add_parser(
        "prepare", help="Create or reuse a hotfix branch and list candidate fixes."
    )
    prepare_parser.add_argument("crate", help="Workspace crate name.")
    prepare_parser.set_defaults(handler=prepare)

    cherry_pick_parser = subparsers.add_parser(
        "cherry-pick", help="Cherry-pick selected commits in the given order."
    )
    cherry_pick_parser.add_argument("commits", nargs="+", help="Commit SHAs.")
    cherry_pick_parser.set_defaults(handler=cherry_pick)

    continue_parser = subparsers.add_parser(
        "continue", help="Continue after resolving and staging other conflicts."
    )
    continue_parser.set_defaults(handler=continue_cherry_pick)
    return parser


def main() -> int:
    try:
        args = create_parser().parse_args()
        print(json.dumps(args.handler(args), indent=2))
        return 0
    except (CommandError, json.JSONDecodeError) as error:
        print(json.dumps({"error": str(error)}), file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
