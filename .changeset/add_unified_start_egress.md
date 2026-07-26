---
livekit-api: minor
---

Add a unified `EgressClient::start_egress` that calls the v2 `Egress.StartEgress` RPC with a `StartEgressRequest`, alongside the existing per-type helpers.
