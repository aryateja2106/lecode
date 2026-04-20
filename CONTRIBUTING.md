# Contributing to lecode

Thanks for your interest in contributing! lecode is an open-source,
Apache-2.0 licensed agent-management platform. Contributions of all kinds
are welcome — code, docs, bug reports, feature ideas.

## Licensing (inbound = outbound)

By submitting a contribution, you agree that your work is licensed under
the **Apache License, Version 2.0**, the same license as the rest of the
project. See [LICENSE](./LICENSE).

## Developer Certificate of Origin (DCO)

All commits must be signed off to certify the DCO
(https://developercertificate.org). Sign off by adding a trailer to your
commit message:

```
Signed-off-by: Your Name <your.email@example.com>
```

You can do this automatically with `git commit -s`.

## Workflow

1. Fork the repo and create a feature branch from `main`.
2. Make your change with a clear, focused commit history.
3. Ensure `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test --workspace` pass for any Rust crates you touch.
4. Ensure `npm run typecheck` and `npm run build` pass for any `web/` changes.
5. Open a pull request describing the change and its motivation.

## Tests are required

Any behavior change needs a test. Bug fixes should include a regression
test that fails before the fix and passes after. No test, no merge.

## Clean-room process

lecode uses a clean-room methodology for any architectural patterns drawn
from copyleft projects. See [SPEC/README.md](./SPEC/README.md) before
submitting code that was designed with reference to a non-permissive
upstream.

## Code of conduct

Participation is governed by our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Security

Please report security vulnerabilities privately — see
[SECURITY.md](./SECURITY.md).
