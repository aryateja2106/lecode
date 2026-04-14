# Issue: Freeze protocol spec v0.1

**Labels:** `area/protocol` `type/spec` `priority/critical` `phase/A.0`
**Milestone:** v0.1.0 Daily Driver
**Story refs:** S-2.1, S-2.2, S-2.6 (from EPICS_AND_STORIES.md)

## Summary

Lock `docs/protocol-v0.1.md` as the canonical wire specification and publish it to the `lesearch-protocol` spec repo. This is the contract every client (CLI, Tauri, web, iOS, third-party A2A) will be built against for Phase A.

## Acceptance criteria

- [ ] `docs/protocol-v0.1.md` ≥ 500 words (currently 1058)
- [ ] Every method and notification documented with request + response + error shape
- [ ] Error code table complete (-32700 through -32008)
- [ ] Versioning policy explicit (MINOR additive-only, MAJOR path-suffixed)
- [ ] A2A surface included (`/.well-known/agent-card.json`, `/message/send`, `/tasks/*`)
- [ ] Binary multiplexing frame layout specified
- [ ] Mirrored to `aryateja2106/lesearch-protocol` as `SPEC.md`
- [ ] Rust types scaffold in `crates/lesearch-protocol/src/lib.rs` exports `PROTOCOL_VERSION`

## Out of scope

- Generated TypeScript types (Day 4+)
- Reference conformance suite (v0.2)
- Deprecated method advertisement (only needed once deprecations begin)

## References

- `docs/PRD.md` §Functional Requirements FR-14-18, FR-29-31
- `docs/SYSTEM_DESIGN.md` §2.1 (crate layout), §3 (protocol framing)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
