---
name: agentic-workflows
description: Route gh-aw workflow design/create/debug/upgrade requests to the right prompts.
---

# Agentic Workflows Router

Use this skill when a user asks to design, create, update, debug, or upgrade GitHub Agentic Workflows in this repository.

This skill is a dispatcher: identify the task type, load the matching workflow prompt/skill file, and follow it directly. Keep responses concise and ask a clarifying question if the correct prompt is unclear.

Repository overlay (optional):

- If `.github/aw/instructions.md` exists, load it with `@.github/aw/instructions.md` after loading the matched prompt/skill.
- Precedence: repository overlay instructions override upstream defaults when they conflict.

## Mandatory preflight

Before editing or compiling workflows:

1. When `gh aw` is available, run `gh extension upgrade aw` and record `gh aw`'s stable version. Then use that updated extension for all compilation.
2. For a repository-wide upgrade, run `gh aw upgrade`; otherwise compile all affected workflows after editing. In MCP-only environments, use the matching agentic-workflows tools and report when extension freshness is externally managed rather than claiming an update occurred.
3. Load the latest relevant files from `github/gh-aw` after the version check. Do not rely on an older copied prompt when current upstream guidance is available.
4. Consult:
   - [How Agentic Workflows Work — Best Practices](https://github.github.com/gh-aw/introduction/how-they-work/#best-practices)
   - [Safe Outputs](https://github.github.com/gh-aw/reference/safe-outputs/)
   - [Safe Outputs for Pull Requests](https://github.github.com/gh-aw/reference/safe-outputs-pull-requests/)

## Tool and output contract

Before compiling, inventory every tool named by workflow frontmatter, imports, custom agents, skills, and prompt instructions. Reconcile the inventory with the runtime access mode:

- Custom-agent `tools:` and workflow frontmatter are cumulative restrictions. Use portable aliases such as `read`, `search`, `execute`, and `web` in reusable agents, plus namespaced MCP tools when needed; the compiled workflow must still expose and constrain each capability for that runtime.
- `tools.github.mode` selects one GitHub read transport. `gh-proxy` exposes authenticated `gh` instead of the GitHub MCP server and requires `gh` in the bash allowlist; local mode exposes the GitHub MCP server. Do not instruct the agent to use both as fallback paths unless the compiler explicitly exposes both.
- CLI-proxied MCP servers require their executable names in the bash allowlist.
- Direct MCP instructions require those tools to remain exposed as MCP tools.
- LSP instructions require an `lsp:` frontmatter entry supported by the selected engine. Include the matching network ecosystem needed to install the language server, and define a file-level fallback without authentication or checkout workarounds.
- PR and issue writes must use configured safe outputs, never raw GitHub tools.
- Prefer dedicated PR review safe outputs over generic comments for line-specific findings, consolidated reviews, and replies.
- Keep the agent job read-only; write permissions belong to generated safe-output jobs.
- Use narrow bash allowlists for workflows processing untrusted PR or issue content. Never enable unrestricted shell access merely to silence a missing-tool report.
- Safe-output calls are real write intents. Never probe them with placeholder calls.
- Review-thread resolution can require a separate token even with `pull-requests: write`; do not enable it unless reliable credentials are explicitly available.

After any workflow change, compile with the updated extension, fix all errors and warnings, and verify every prompt-named tool and safe output is present in the compiled runtime. When asked to refresh the repository, compile every `.github/workflows/*.md` file so no lock file remains stale.

Read only the files you need:
Load these files from `github/gh-aw` (they are not available locally).

- `.github/aw/action-container-substitutions.md`
- `.github/aw/agent-runtime-instructions.md`
- `.github/aw/agentic-chat.md`
- `.github/aw/agentic-workflows-mcp.md`
- `.github/aw/asciicharts.md`
- `.github/aw/campaign.md`
- `.github/aw/charts-trending.md`
- `.github/aw/charts.md`
- `.github/aw/cli-commands.md`
- `.github/aw/configure-agentic-engine.md`
- `.github/aw/context.md`
- `.github/aw/create-agentic-workflow-trigger-details.md`
- `.github/aw/create-agentic-workflow.md`
- `.github/aw/create-shared-agentic-workflow.md`
- `.github/aw/debug-agentic-workflow.md`
- `.github/aw/dependabot.md`
- `.github/aw/deployment-status.md`
- `.github/aw/designer-mappings.md`
- `.github/aw/designer.md`
- `.github/aw/evals.md`
- `.github/aw/experiments.md`
- `.github/aw/github-agentic-workflows.md`
- `.github/aw/github-mcp-server-pagination.md`
- `.github/aw/github-mcp-server.md`
- `.github/aw/instructions.md`
- `.github/aw/linter-workflows.md`
- `.github/aw/llms.md`
- `.github/aw/loop.md`
- `.github/aw/lsp.md`
- `.github/aw/maintainer.md`
- `.github/aw/mcp-clis.md`
- `.github/aw/memory-stateful-patterns.md`
- `.github/aw/memory.md`
- `.github/aw/messages.md`
- `.github/aw/multi-agent-research.md`
- `.github/aw/network.md`
- `.github/aw/optimize-agentic-workflow.md`
- `.github/aw/patterns.md`
- `.github/aw/pr-reviewer.md`
- `.github/aw/release-workflow.md`
- `.github/aw/report.md`
- `.github/aw/reuse.md`
- `.github/aw/safe-outputs-automation.md`
- `.github/aw/safe-outputs-content.md`
- `.github/aw/safe-outputs-management.md`
- `.github/aw/safe-outputs-runtime.md`
- `.github/aw/safe-outputs.md`
- `.github/aw/serena-tool.md`
- `.github/aw/shared-safe-jobs.md`
- `.github/aw/skills.md`
- `.github/aw/subagents.md`
- `.github/aw/syntax-agentic.md`
- `.github/aw/syntax-core.md`
- `.github/aw/syntax-engine.md`
- `.github/aw/syntax-tools-imports.md`
- `.github/aw/syntax.md`
- `.github/aw/test-coverage.md`
- `.github/aw/test-expression.md`
- `.github/aw/token-optimization-caching-budgets.md`
- `.github/aw/token-optimization-observability.md`
- `.github/aw/token-optimization.md`
- `.github/aw/triggers.md`
- `.github/aw/update-agentic-workflow.md`
- `.github/aw/upgrade-agentic-workflows.md`
- `.github/aw/visual-regression.md`
- `.github/aw/workflow-constraints.md`
- `.github/aw/workflow-editing.md`
- `.github/aw/workflow-patterns.md`

After loading the matching workflow prompt or skill, follow it directly:

- Design workflows from scratch via interview: `.github/aw/designer.md`
- Create new workflows: `.github/aw/create-agentic-workflow.md`
- Configure or add declarative engines: `.github/aw/configure-agentic-engine.md`
- Update existing workflows: `.github/aw/update-agentic-workflow.md`
- Debug, audit, or investigate workflows: `.github/aw/debug-agentic-workflow.md`
- Upgrade workflows and fix deprecations: `.github/aw/upgrade-agentic-workflows.md`
- Create shared components or MCP wrappers: `.github/aw/create-shared-agentic-workflow.md`
- Create report-generating workflows: `.github/aw/report.md`
- Fix Dependabot manifest PRs: `.github/aw/dependabot.md`
- Analyze coverage workflows: `.github/aw/test-coverage.md`
- Render compact markdown charts: `.github/aw/asciicharts.md`
- Map CLI commands to MCP usage: `.github/aw/cli-commands.md`
- Choose workflow architecture and patterns: `.github/aw/patterns.md`
- Optimize token usage and cost: `.github/aw/token-optimization.md`
- Design long-running multi-agent research workflows: `.github/aw/multi-agent-research.md`

When the task involves OTEL, OTLP, traces, observability backends, or telemetry-driven analysis, also read and follow `skills/otel-queries/SKILL.md` after loading the matching workflow prompt or skill.
