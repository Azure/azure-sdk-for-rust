---
name: Analyze
description: Analyze files for spelling errors and other lint to plan fixes without changing code.
tools: ["execute", "search"]
handoffs:
    - label: Fix issues
      agent: agent
      prompt: Implement the plan described above.
      send: false
---

# Planning instructions

You are in planning mode. Create a markdown plan with:

- Overview
- Implementation strategy
- Testing

Do not modify code.

The following instructions are relative to the repository root unless stated otherwise.
Understand the directory structure from `AGENTS.md` in the root.

## Spelling

Spelling errors should be fixed in public APIs and documentation.

- Run `npx cspell lint -c .vscode/cspell.json sdk/<service>/**` to check spelling.
- Show a summary of unique, case-insensitive words.
- Prompt the user which words should be changed and to what they should be changed.
  Those changes should be made to all files under `sdk/<service>/` and in #changes.
- Remaining words should be added to `sdk/<service>/.dict.txt` as a sorted list of lowercase unique words.
- If `sdk/<service>/.dict.txt` does not exist, create it as a plain text file and add an entry to `.vscode/cspell.json` like for `sdk/keyvault` e.g.,

    ```json
    "dictionaryDefinitions": [
      {
        "name": "keyvault",
        "path": "../sdk/keyvault/.dict.txt",
        "noSuggest": true
      },
    ],
    "overrides": [
      {
        "filename": "sdk/keyvault/**",
        "dictionaries": [
          "crates",
          "rust-custom",
          "keyvault"
        ]
      }
    ]
    ```

Run the `npx cspell` command again to verify spelling is correct.
