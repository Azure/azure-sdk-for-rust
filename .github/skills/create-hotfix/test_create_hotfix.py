# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

import importlib.util
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


if __name__ == "__main__":
    unittest.main()
