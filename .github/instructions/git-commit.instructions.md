# Instructions for Git Commits

- Git commits should follow the standard form:

    ```markdown
    title of no more than 50 characters

    multi-line descriptions
    ```

- Summarize changes for a title of no more than 50 characters ideally, with a hard stop at 80 characters.
- Descriptions can be multiple lines or even paragraphs.
- Descriptions may contain markdown.
- Descriptions should not repeat a lot of content already found in the changed files.
- Summarize changes for each file if no more than 10 files are changed.
- If multiple files contain the same changes, group them together when summarizing changes.
- Try to reason why changes were made and not about what was changed.
- If any added comments in changed files reference fixing an issue like "Fixes #1234" include that same text in the description.
- If the branch name includes an issue number like "issue1234" include "Fixes #1234" in the description.
