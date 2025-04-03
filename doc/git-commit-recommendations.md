# Recommendations for Git Commits

* Git commits should follow the standard form:

  ```markdown
  title of no more than 50 characters

  multi-line descriptions
  ```

* Summarize changes for a title of no more than 50 character ideally, with a hard stop at 80 characters.
* Descriptions can be multiple lines or even paragraphs.
* Descriptions can be detailed but should not be overly verbose and repeat a large amount of content found within changed files.
* Summarize changes for each file if no more than 10 files are changed.
* If multiple files contain the same changes, group them together when summarizing changes.
* Emphasize why changes were made and less about what was changed.
* If any comments reference fixing an issue like "Fixes #1234" include that same text in the description.
* If the branch name includes an issue number like "issue1234" include "Fixes #1234" in the description.
