# ADR 0001: License the project under Apache 2.0

**Status:** Accepted
**Date:** 2026-04-14
**Deciders:** Arya Teja Rudraraju

## Context

LeSearch is a new self-hosted agent control plane competing in a space where the most visible incumbent (paseo) is AGPL-3.0. The license choice materially shapes who can adopt, fork, embed, and commercialize the code — and whether this project can attract enterprise users, plugin authors, and downstream products (including Arya's own other products: arya-cmux, karna, CloudAGI, nl2shell).

Four licenses were considered:

1. **MIT** — permissive, minimal
2. **Apache-2.0** — permissive with explicit patent grant and contribution clauses
3. **AGPL-3.0** — copyleft extending to network use (matches paseo)
4. **BSL** (Business Source License) — time-delayed open source; converts to Apache after N years

## Decision

License LeSearch under **Apache License 2.0**.

All first-party crates (`lesearch-*`), the protocol spec repo (`lesearch-protocol`), native apps (`lesearch-macos`, `lesearch-ios`), and auxiliary libraries (`lesearch-macos-display`, `lesearch-avm`) ship under Apache-2.0. External dependencies MUST be MIT, Apache-2.0, BSD, or ISC. AGPL and GPLv3 are disallowed.

## Consequences

### Positive

- **Broad downstream adoption** — enterprises can self-host, embed in products, ship commercial forks without copyleft contagion
- **Clean coexistence with paseo (AGPL-3.0)** — LeSearch contributions to paseo are licensed as paseo requires (AGPL-3.0); LeSearch's own moats remain unencumbered
- **Patent grant** — Apache's explicit patent clause is stronger than MIT; important when the project involves crypto, protocol design, and potential enterprise use
- **Compatible with Arya's four-product portfolio** — arya-cmux (CGVirtualDisplay isolation), karna (Rust agent runtime), CloudAGI (x402 economy), and nl2shell (local LLM shell) can freely depend on LeSearch crates without licensing conflict
- **Hiring signal** — "I built Apache-2.0 infrastructure used by X companies" outranks "I built AGPL tool that only our own products use"

### Negative

- **No copyleft enforcement** — someone can fork LeSearch, add proprietary features, and ship closed-source. Mitigation: moat features (CGVirtualDisplay isolation, AVM policy engine, A2A dispatch layer) live in separate repos; some of them can ship BSL or dual-license later if differentiation erodes.
- **AGPL-3.0 dependency exclusion** — we cannot absorb paseo's code, only its ideas. This is a feature (forces clean-room implementations) and a cost (we rebuild instead of depend).
- **Contribution ergonomics** — Apache CLAs/DCO add friction vs MIT. Mitigation: use a Developer Certificate of Origin (`Signed-off-by` on commits) rather than a CLA; low overhead.

### Neutral

- NOTICE file is required. We maintain one listing all third-party dependencies and pattern sources (see `NOTICE`).
- Apache 2.0 requires modified files to carry prominent change notices. Handled by git history; no per-file comment burden.

## Alternatives Considered

- **MIT** — simpler, no patent clause. Rejected because Apache's patent clause is meaningfully stronger for infrastructure that touches crypto and may attract patent trolls.
- **AGPL-3.0** — matches paseo. Rejected because it blocks the downstream-portfolio strategy: CloudAGI and nl2shell products want to embed LeSearch crates without AGPL contagion.
- **BSL** — interesting for monetizable products. Rejected for v0.x because the project is pre-revenue and the BSL community signal (e.g., MongoDB, Sentry) is "we will start charging"; wrong signal for a self-hosted developer tool trying to attract contributors.

## Compliance

- Every source file gets the SPDX header: `// SPDX-License-Identifier: Apache-2.0` (deferred to the CI linter; not a Day-1 blocker)
- `LICENSE` contains the full Apache 2.0 text
- `NOTICE` lists all third-party dependencies with their licenses
- External contributions require DCO (`Signed-off-by` line); enforced by CI once PRs start flowing

## References

- [Apache License 2.0 text](https://www.apache.org/licenses/LICENSE-2.0.txt)
- [Choosealicense.com — Apache 2.0](https://choosealicense.com/licenses/apache-2.0/)
- [paseo LICENSE](https://github.com/paseo-ai/paseo/blob/main/LICENSE) (AGPL-3.0, for contrast)
