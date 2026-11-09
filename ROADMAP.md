# Huddle Roadmap

Huddle is a self-hostable video meeting app built with a Rust (Axum) backend and a SvelteKit frontend. It's a portfolio project meant to show two things: Rust skills, and agent-driven development.

Each phase is tagged by who writes it:

- ✍️ **By hand:** core logic I write myself to learn how it works.
- 🤖 **Agent:** scaffolding, tooling and polish I delegate and then review.

Each phase ends with a **Done when** line that says what has to work before moving on.

---

## Phase 0: Foundation ✅

- [x] Axum server with env-based config (`HUDDLE_ADDR`), request tracing and graceful shutdown
- [x] `GET /health` with router tests
- [x] Removed better-auth and Drizzle from SvelteKit; the backend owns data and auth

## Phase 1: WebSocket basics ✍️

Goal: understand what a WebSocket is before building on it.

- [ ] Echo server at `GET /ws` (enable axum's `ws` feature), tested with `websocat`
- [ ] Broadcast chat: every client sees every message
- [ ] Clean up properly when a client disconnects

**Learn:** the HTTP upgrade handshake; frame types (text, binary, ping, close); splitting a socket into a sink and a stream; one task per connection; `tokio::sync::broadcast` vs a map of `mpsc` senders behind a `Mutex`.

**Done when:** three `websocat` terminals can chat, and killing one doesn't break the others.

## Phase 2: Rooms and signaling server ✍️

Goal: the backend half of a meeting app.

- [ ] Room registry: `POST /rooms` and `GET /rooms/{id}`
- [ ] `GET /rooms/{id}/ws`: join a room, get a peer id
- [ ] Protocol as serde tagged enums: `welcome`, `peer_joined`, `peer_left`, `signal {to/from, data}`, `error`
- [ ] Relay `signal` to a single peer without inspecting `data`
- [ ] Bounded queues: a slow client must not stall the others or use unlimited memory
- [ ] Integration tests with `tokio-tungstenite`

**Learn:** shared state in Axum; why holding a lock across `.await` is a bug; backpressure; protocol design; testing async code.

**Done when:** two test clients join a room, exchange a `signal`, and the other one sees `peer_left` when one disconnects.

## Phase 3: Frontend lobby and socket client 🤖 UI / ✍️ client

- [ ] 🤖 Home page with a "New meeting" button; `/room/[id]` page; layout and styling
- [ ] 🤖 Vite dev proxy from `/api` and `/ws` to `localhost:8080`, so no CORS is needed
- [ ] ✍️ Typed TypeScript client for the signaling protocol, with Svelte state for the peer list
- [ ] Unknown room shows a "not found" page before asking for the camera

**Done when:** two browser tabs on the same room each see the other's peer id appear and disappear.

## Phase 4: First 1:1 video call ✍️

The core of the whole project.

- [ ] `getUserMedia` camera preview
- [ ] One `RTCPeerConnection` per remote peer
- [ ] Offer and answer exchanged over the signaling socket
- [ ] Trickle ICE candidates, using a public STUN server for now

**Learn:** SDP, ICE, STUN vs TURN, and the "perfect negotiation" pattern (polite and impolite peers) that avoids glare, where both sides send an offer at the same time.

**Done when:** a video call works between two tabs, then between two devices on the same Wi-Fi.

## Phase 5: Group calls (mesh) ✍️ logic / 🤖 UI

- [ ] One peer connection per other participant
- [ ] 🤖 Responsive video grid
- [ ] Mute, camera off and leave controls
- [ ] Display names (sent with the join message)

**Done when:** a 4-person call works, and anyone leaving tidies up everyone else's grid.

## Phase 6: Reliability ✍️

- [ ] Server pings to detect dead connections
- [ ] Client reconnects with backoff and rejoins the room
- [ ] Handle ICE failure with an ICE restart
- [ ] TURN server (coturn) for strict NATs
- [ ] Clear UI states: connecting, reconnecting, permission denied, room full

**Done when:** pulling the network cable for 5 seconds recovers without a page refresh.

## Phase 7: Persistence and auth ✍️ auth / 🤖 plumbing

- [ ] 🤖 Postgres with `sqlx` and migrations; `docker compose` for dev
- [ ] ✍️ Accounts: argon2 password hashing and cookie sessions
- [ ] Only signed-in users can create rooms; guests can join by link
- [ ] Rate-limit room creation
- [ ] Host controls: lock room, remove participant

**Done when:** a guest can join a room by link, and only signed-in users can create one.

## Phase 8: In-meeting features ✍️

- [ ] Text chat (decide: through the server or over a WebRTC data channel, and why)
- [ ] Screen sharing (`getDisplayMedia`, replacing the video track)
- [ ] Active speaker highlight (Web Audio API)

## Phase 9: Self-hosting 🤖

- [ ] Multi-stage Dockerfile for the backend; SvelteKit with `adapter-node`
- [ ] One `compose.yaml` with backend, frontend, Postgres, coturn and Caddy (automatic HTTPS)
- [ ] Docs: configuration, ports (UDP for TURN), domain setup

Note: browsers only allow camera access over HTTPS (or on localhost), so TLS is required, not optional.

**Done when:** `docker compose up` on a fresh VPS with a domain name gives a working meeting.

## Phase 10: Portfolio polish 🤖

- [ ] CI with GitHub Actions: `cargo fmt --check`, `clippy -D warnings`, tests, `bun run check`
- [ ] README with an architecture diagram, demo GIF and quickstart
- [ ] Short architecture decision records (ADRs): why mesh, why the server relays opaque payloads, why Rust owns auth
- [ ] Write-up of the agent-driven workflow: what was hand-written vs delegated, and how agent output was reviewed

---

## Stretch goals

- **SFU in Rust** (with `str0m` or `webrtc-rs`): the server forwards media streams, so calls scale past the 6–8 person limit of mesh. This is the biggest Rust showcase on the list.
- Recording
- End-to-end encryption with insertable streams
- Waiting room / knock to join
