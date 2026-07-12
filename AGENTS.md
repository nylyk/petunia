# Petunia

Petunia is a native Signal client, named after the flower.

- **Signal library:** [`presage`](https://github.com/whisperfish/presage) — follow
  its examples for how to link, send, and receive.
- **GUI framework:** [`iced`](https://github.com/iced-rs/iced).
- **Look and feel:** modeled on [Halloy](https://github.com/squidowl/halloy), the
  iced IRC client — including movable, resizable panes, one per chat.

The first version implements only basic features, but the goal is a fully-featured
Signal client. Structure the project so that future features can be added without
reworking what already exists.

## Coding rules

- **KISS.** The simplest thing that works. No abstraction for features that don't
  exist yet.
- **SOLID.** Each module has one responsibility. Depend on interfaces at
  boundaries. Extend by adding, not by editing shared code.
- **Clean code.** Small functions, clear names, no dead code, no speculative
  generality.
- **No comments** unless absolutely necessary — code should explain itself.
  Comment only what stays genuinely surprising (protocol quirks, invariants,
  `unsafe`).
