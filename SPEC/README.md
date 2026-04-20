# SPEC — Clean-Room Methodology

The `SPEC/` tree is how lecode formalizes concepts and architecture **before**
implementation. Its purpose is twofold: (1) record design intent in
durable, reviewable form, and (2) enforce a clean-room boundary between
copyleft upstream projects that inspire lecode and the permissively
licensed code that ships in this repo.

## Roles

- **Analysts** read inspiration sources (including AGPL projects such as
  Paseo) and write architecture and behavior descriptions in
  `SPEC/concepts/`. Analysts **never** copy source code, comments, or
  prose verbatim. Describe shapes, flows, invariants, and rationales in
  your own words.
- **Implementers** work only from `SPEC/` documents. They **do not read**
  the original inspiration source while writing code that will land in
  `crates/` or `web/`. This separation is what makes the output
  clean-room.

## What belongs in a SPEC doc

Each file in `SPEC/concepts/` should cover:

1. **Scope** — what problem or subsystem is being described.
2. **Behavior** — external contracts, protocols, state transitions,
   invariants. Prose and diagrams, not code.
3. **Data shapes** — the *shape* of messages and records, described
   abstractly (field names and semantics are fine; do not paste type
   definitions from upstream).
4. **Open questions** — anything the implementer must decide.
5. **Attestation** — a footer stating: the author's role (analyst),
   which sources were consulted, and confirmation that no verbatim code
   or prose was transcribed.

## Naming

Use `SPEC/concepts/<area>-<topic>.md`, e.g.
`SPEC/concepts/daemon-agent-lifecycle.md`.

## Review

A SPEC doc is reviewed like code: a PR, a reviewer, and an explicit
acknowledgment that the attestation is accurate. Only after a SPEC
merges do implementers begin writing code against it.
