# lecode

[![CI](https://github.com/aryateja2106/lecode/actions/workflows/ci.yml/badge.svg)](https://github.com/aryateja2106/lecode/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)

> **One terminal. Every agent. Every machine.**

lecode is an open-source agent-management platform. It runs a small
daemon on any machine you own, a thin CLI for local control, and a
companion web experience — so you can launch, observe, and steer local
AI coding agents (Claude Code, Codex, OpenCode, and more) from anywhere,
without giving up custody of your source tree. Your code never leaves
your hardware; lecode just makes it easier to drive the agents that run
on top of it.

## Status

**v0.0.0 — scaffolding.** This repository currently contains only the
umbrella layout, license, and CI. The daemon (`crates/`) and web
companion (`web/`) will land in subsequent waves. See
[SPEC/README.md](./SPEC/README.md) for the clean-room process that
governs how concepts move from inspiration into code.

## Quick start

_Placeholder — the daemon and CLI are not yet bootstrapped._

```bash
# Future workflow:
# cargo run -p lecode-daemon
# cargo run -p lecode-cli -- ls
# cd web && npm ci && npm run dev
```

## Repository layout

```
lecode/
  crates/              Rust workspace members (daemon, cli, relay — coming soon)
  web/                 Next.js companion site (coming soon)
  SPEC/
    README.md          Clean-room methodology
    concepts/          Architecture and behavior descriptions
  .github/workflows/   CI
  LICENSE              Apache-2.0 full text
  NOTICE               Attribution and code-lineage notes
  SECURITY.md          Responsible disclosure
  CONTRIBUTING.md      How to contribute (DCO, inbound=outbound)
  CODE_OF_CONDUCT.md   Contributor Covenant 2.1
```

## How to contribute

1. Read [CONTRIBUTING.md](./CONTRIBUTING.md).
2. For any feature that takes design cues from an external project, read
   [SPEC/README.md](./SPEC/README.md) first — we use a clean-room
   workflow.
3. Sign off your commits (`git commit -s`) per the DCO.
4. Open a PR with a clear motivation.

## License

Licensed under the [Apache License, Version 2.0](./LICENSE). See
[NOTICE](./NOTICE) for attribution and code-lineage notes.
