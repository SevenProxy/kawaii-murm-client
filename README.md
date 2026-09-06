# murm

**murm** is a decentralized, privacy-first messaging and social client. It follows a relay-based architecture inspired by the Nostr model: every message is a *signed event* authenticated with an Ed25519 keypair that lives only on your device — there is no central account, no server database and no password.

This repository is the reference **client**, written in Rust and compiled to WebAssembly with [Yew](https://yew.rs), featuring a native CLI build that acts as a minimal MIP-03 relay client.

> Read this in [Português (BR)](README.pt-BR.md)

---

## Table of Contents

- [Highlights](#highlights)
- [How it works](#how-it-works)
- [The murm protocol](#the-murm-protocol)
- [Tech stack](#tech-stack)
- [Roadmap](#roadmap)
- [Getting started](#getting-started)
- [Project layout](#project-layout)
- [Contributing](#contributing)
- [Contributors](#contributors)

## Highlights

- **Self-sovereign identity** — an Ed25519 keypair is generated locally in the browser and stored in `localStorage`. Your private key never leaves your device.
- **Cryptographically signed content** — every event (post, profile, reaction) is hashed and signed. Tampering is always detectable via local verification.
- **No trusted middlemen** — relays are treated as untrusted storage; signatures are validated on the client (MIP-05).
- **Web + native** — the same codebase builds a responsive web app (WASM) and a native binary with a relay client for publish/fetch/scan.
- **Discord-like UX** — channels, a clean dark theme, and an in-app profile editor.

## How it works

1. On first visit, the client generates an Ed25519 keypair and persists it in the browser.
2. You publish *signed events* — posts, replies, reactions or profile data — to the network.
3. Relays store and serve these events to any client that asks for them.
4. Every client verifies event IDs and signatures locally before trusting any content.

Because content lives on relays and is signed by your keypair, you can move between clients or relays without ever losing your identity.

## The murm protocol

`murm` defines its own set of improvement proposals (MIPs) that shape events, IDs and relay communication:

| MIP | What it defines |
| --- | --- |
| MIP-01 | Event shape and the 2 MiB size limit |
| MIP-02 | Canonical payload, SHA-256 event IDs and Ed25519 signatures |
| MIP-03 | HTTP relay binding — `submit`, `fetch` and `scan` endpoints |
| MIP-04 | Initial event kinds: `profile (0)`, `post (1)`, `comment (2)`, `reaction (3)` |
| MIP-05 | Relays are not trusted — clients must always verify locally |

## Tech stack

### Currently in use

**Client core**
- [Rust](https://www.rust-lang.org/) (edition 2024)
- [Yew](https://yew.rs/) `0.23` — reactive web framework (CSR), function components
- [yew-router](https://docs.rs/yew-router) — client-side routing (`/`, `/generate`, `/app`)
- [Trunk](https://trunkrs.dev/) — WASM build tool and dev server
- [Tailwind CSS](https://tailwindcss.com/) (via CDN) + custom CSS (Inter / JetBrains Mono)

**Cryptography & identity**
- [ed25519-dalek](https://docs.rs/ed25519-dalek) — Ed25519 signing/verification for identities
- [sha2](https://docs.rs/sha2) — SHA-256 digest for event IDs (MIP-02)
- [rand](https://docs.rs/rand) — CSPRNG-backed key generation
- [hex](https://docs.rs/hex) — hex encoding of keys, IDs and signatures

**Web platform**
- [gloo-storage](https://docs.rs/gloo-storage) — `localStorage` persistence of identity and profile
- [web-sys](https://docs.rs/web-sys) — DOM bindings for inputs and textareas

**Serialization & protocol**
- [serde](https://serde.rs/) / [serde_json](https://docs.rs/serde_json) — JSON events and canonical serialization
- [anyhow](https://docs.rs/anyhow) — ergonomic error handling

**Native side (non-WASM target)**
- [reqwest](https://docs.rs/reqwest) — async HTTP relay client (MIP-03)
- [tokio](https://tokio.rs/) — async runtime
- [clap](https://docs.rs/clap) — CLI argument parsing

## Roadmap

Planned next steps for the client:

- [ ] Wire relay publishing/scanning into the browser build (connect the WASM client to the MIP-03 relay client over HTTP/WebSocket)
- [ ] Working timeline: actually read posts from relays and render them
- [ ] Threads — replies and comments wired to `root` / `parent` tags
- [ ] Reactions UI using the `reaction` event kind
- [ ] End-to-end encrypted direct messages (new event kind)
- [ ] NIP-05-style identifier verification (`name@domain` indicator)
- [ ] Multi-relay support with automatic failover
- [ ] Notifications
- [ ] Broader MIP-03 bindings (WebSocket transport)
- [ ] Expanded test suite and CI

## Getting started

### Prerequisites

- Rust toolchain (stable)
- `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev/)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Run the web app

```bash
trunk serve
```

Open `http://localhost:8080`.

### Native binary & tests

The crate also builds for the host with a native entrypoint that exercises event signing and the relay client:

```bash
# run the native demo (creates/loads ./.murm_id and signs a sample event)
cargo run

# run the test suite (cryptography, events, profiles)
cargo test
```

## Project layout

```
src/
├── config/     # client configuration (default relay URL, scan limits)
├── event/      # MIP protocol primitives
│   ├── event.rs    # Event + Payload, canonical payload, signing and verification
│   ├── filters.rs  # MIP-03 event filters
│   ├── kinds.rs    # MIP-04 event kinds
│   ├── tags.rs     # tag helpers (root, parent, target, topic, lang)
│   └── profile.rs  # kind-0 author profiles
├── identity/   # Ed25519 identity (generate, load, save, sign)
├── relay/      # MIP-03 relay client (submit / fetch / scan)
└── web/        # Yew frontend
    ├── components/  # reusable UI (mesh background)
    ├── pages/       # home, generate-identity and app views
    └── router.rs    # route definitions
```

## Contributing

Contributions are welcome. Open an issue or a pull request, and follow the existing code conventions — the codebase is organized around the MIP modules and keeps protocol logic independent of the UI.

## Contributors

- [SevenProxy](https://github.com/SevenProxy) — author and maintainer