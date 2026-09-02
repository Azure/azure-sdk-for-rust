# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

MODULE_PATH = Path(__file__).with_name("create_hotfix.py")
SPEC = importlib.util.spec_from_file_location("create_hotfix", MODULE_PATH)
assert SPEC and SPEC.loader
CREATE_HOTFIX = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CREATE_HOTFIX)


class StableTagTests(unittest.TestCase):
    def test_parses_stable_tag(self):
        self.assertEqual(
            (1, 2, 3),
            CREATE_HOTFIX.parse_stable_tag("azure_identity", "azure_identity@1.2.3"),
        )

    def test_rejects_prerelease_and_other_formats(self):
        tags = [
            "azure_identity@1.2.3-beta.1",
            "azure_identity-1.2.3",
            "azure_identity_1.2.3",
            "azure_core@1.2.3",
        ]
        self.assertTrue(
            all(
                CREATE_HOTFIX.parse_stable_tag("azure_identity", tag) is None
                for tag in tags
            )
        )

    def test_rejects_invalid_semver_leading_zero(self):
        self.assertIsNone(
            CREATE_HOTFIX.parse_stable_tag("azure_identity", "azure_identity@1.02.3")
        )

    def test_resolves_latest_stable_tag_and_patch(self):
        tag, patch = CREATE_HOTFIX.resolve_base_tag(
            "azure_identity",
            [
                "azure_identity@1.0.0",
                "azure_identity@1.2.0-beta.1",
                "azure_identity@1.10.0",
                "azure_identity@1.9.5",
                "azure_identity@0.9.5",
            ],
        )
        self.assertEqual("azure_identity@1.10.0", tag)
        self.assertEqual("1.10.1", patch)

    def test_errors_without_stable_tag(self):
        with self.assertRaisesRegex(
            CREATE_HOTFIX.CommandError, "no stable release tag"
        ):
            CREATE_HOTFIX.resolve_base_tag(
                "azure_identity", ["azure_identity@1.1.0-beta.1"]
            )

    def test_summarizes_text_and_binary_changes(self):
        summary = CREATE_HOTFIX.summarize_files(
            [
                {
                    "path": "sdk/identity/src/lib.rs",
                    "status": "M",
                    "additions": 3,
                    "deletions": 1,
                },
                {
                    "path": "sdk/identity/test.bin",
                    "status": "A",
                    "additions": None,
                    "deletions": None,
                },
            ]
        )
        self.assertEqual(
            "M sdk/identity/src/lib.rs (+3/-1), "
            "A sdk/identity/test.bin (binary)",
            summary,
        )

    @mock.patch.object(CREATE_HOTFIX, "run")
    def test_reads_tags_from_canonical_upstream(self, run):
        run.return_value = (
            "abc\trefs/tags/azure_identity@1.0.0\n"
            "def\trefs/tags/azure_identity@1.1.0\n"
        )
        self.assertEqual(
            ["azure_identity@1.0.0", "azure_identity@1.1.0"],
            CREATE_HOTFIX.upstream_tags("azure_identity"),
        )
        run.assert_called_once_with(
            "git",
            "ls-remote",
            "--refs",
            "--tags",
            CREATE_HOTFIX.UPSTREAM_URL,
            "refs/tags/azure_identity@*",
        )

    @mock.patch.object(CREATE_HOTFIX, "run")
    def test_excludes_non_publishable_workspace_crates(self, run):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            identity_manifest = (
                root / "sdk" / "identity" / "azure_identity" / "Cargo.toml"
            )
            core_test_manifest = (
                root / "sdk" / "core" / "azure_core_test" / "Cargo.toml"
            )
            run.return_value = json.dumps(
                {
                    "workspace_members": ["publishable", "private"],
                    "packages": [
                        {
                            "id": "publishable",
                            "name": "azure_identity",
                            "version": "1.0.0",
                            "manifest_path": str(identity_manifest),
                            "publish": None,
                        },
                        {
                            "id": "private",
                            "name": "azure_core_test",
                            "version": "0.1.0",
                            "manifest_path": str(core_test_manifest),
                            "publish": [],
                        },
                    ],
                }
            )

            self.assertEqual(
                [
                    {
                        "name": "azure_identity",
                        "path": "sdk/identity/azure_identity",
                        "version": "1.0.0",
                    }
                ],
                CREATE_HOTFIX.workspace_crates(root),
            )


class ConflictTests(unittest.TestCase):
    def test_resolves_cargo_lock_and_returns_other_conflicts(self):
        with tempfile.TemporaryDirectory() as directory:
            marker = Path(directory) / "lock-conflict"
            with (
                mock.patch.object(
                    CREATE_HOTFIX,
                    "conflicted_files",
                    return_value=["Cargo.lock", "src/lib.rs"],
                ),
                mock.patch.object(CREATE_HOTFIX, "lock_conflict_marker") as get_marker,
                mock.patch.object(CREATE_HOTFIX, "run") as run,
            ):
                get_marker.return_value = marker
                conflicts = CREATE_HOTFIX.resolve_cargo_lock_conflict()

            self.assertEqual(["src/lib.rs"], conflicts)
            self.assertTrue(marker.exists())
        self.assertEqual(
            [
                mock.call("git", "checkout", "--ours", "--", "Cargo.lock"),
                mock.call("git", "add", "Cargo.lock"),
            ],
            run.call_args_list,
        )

    def test_continue_requires_an_active_cherry_pick(self):
        args = mock.Mock()
        with (
            mock.patch.object(
                CREATE_HOTFIX, "current_branch", return_value="hotfix/fixture"
            ),
            mock.patch.object(
                CREATE_HOTFIX, "cherry_pick_in_progress", return_value=False
            ),
            mock.patch.object(CREATE_HOTFIX, "lock_conflict_marker") as get_marker,
        ):
            get_marker.return_value = mock.Mock()
            with self.assertRaisesRegex(
                CREATE_HOTFIX.CommandError, "no cherry-pick is in progress"
            ):
                CREATE_HOTFIX.continue_cherry_pick(args)

    def test_cherry_pick_rejects_an_active_sequence(self):
        args = mock.Mock(commits=["abc"])
        with (
            mock.patch.object(
                CREATE_HOTFIX, "current_branch", return_value="hotfix/fixture"
            ),
            mock.patch.object(
                CREATE_HOTFIX, "cherry_pick_in_progress", return_value=True
            ),
        ):
            with self.assertRaisesRegex(
                CREATE_HOTFIX.CommandError, "use the continue command"
            ):
                CREATE_HOTFIX.cherry_pick(args)

    def test_advance_stops_on_non_conflict_failure(self):
        result = CREATE_HOTFIX.subprocess.CompletedProcess(
            args=["git", "cherry-pick", "--continue"],
            returncode=1,
            stdout="",
            stderr="the previous cherry-pick is now empty",
        )
        with (
            mock.patch.object(
                CREATE_HOTFIX, "resolve_cargo_lock_conflict", return_value=[]
            ),
            mock.patch.object(CREATE_HOTFIX, "regenerate_cargo_lock"),
            mock.patch.object(CREATE_HOTFIX, "run_process", return_value=result) as run,
            mock.patch.object(
                CREATE_HOTFIX, "cherry_pick_in_progress", return_value=True
            ),
            mock.patch.object(CREATE_HOTFIX, "conflicted_files", return_value=[]),
        ):
            with self.assertRaisesRegex(
                CREATE_HOTFIX.CommandError, "cherry-pick is now empty"
            ):
                CREATE_HOTFIX.advance_cherry_pick("hotfix/fixture")

        run.assert_called_once()

    def test_regenerates_cargo_lock_after_conflicts_are_resolved(self):
        with tempfile.TemporaryDirectory() as directory:
            marker = Path(directory) / "lock-conflict"
            marker.touch()
            with (
                mock.patch.object(
                    CREATE_HOTFIX,
                    "lock_conflict_marker",
                    return_value=marker,
                ),
                mock.patch.object(CREATE_HOTFIX, "run") as run,
            ):
                CREATE_HOTFIX.regenerate_cargo_lock()

            self.assertFalse(marker.exists())

        self.assertEqual(
            [
                mock.call(
                    "cargo",
                    "generate-lockfile",
                    "--manifest-path",
                    "Cargo.toml",
                ),
                mock.call("git", "add", "Cargo.lock"),
            ],
            run.call_args_list,
        )


if __name__ == "__main__":
    unittest.main()
