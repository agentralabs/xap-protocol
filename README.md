<p align="center">
  <h1 align="center">XAP</h1>
  <p align="center"><strong>eXchange Agent Protocol</strong></p>
  <p align="center">The open economic protocol for autonomous agent-to-agent commerce.</p>
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-white.svg" alt="License: MIT"></a>
  <a href="#versioning"><img src="https://img.shields.io/badge/Status-Draft%20v0.1-yellow.svg" alt="Status: Draft v0.1"></a>
  <a href="https://www.agentralabs.tech"><img src="https://img.shields.io/badge/Maintained%20by-Agentra%20Labs-blue.svg" alt="Maintained by: Agentra Labs"></a>
  <a href="https://discord.gg/agentralabs"><img src="https://img.shields.io/badge/Discord-Join%20Community-5865F2.svg" alt="Discord"></a>
</p>

---

## The Problem

AI agents can think, reason, code, search, and orchestrate. But when two agents need to do business with each other, there is no shared language for how that happens.

No standard way to negotiate a price. No way to escrow funds against a conditional outcome. No way to split payment across five agents that contributed to one result. No way to prove, months later, exactly why a settlement resolved the way it did.

Stripe and OpenAI built [ACP](https://agenticcommerce.dev/) for agent-assisted shopping. Google built [AP2](https://ap2-protocol.org/) for human-authorized agent payments. Coinbase built [x402](https://x402.org/) for pay-per-request API access.

All of them assume a human in the loop. A human clicking buy. A human signing a mandate. A human approving a charge.

XAP is for when there is no human in the loop. When agents negotiate with agents, settle with agents, and prove what happened to humans after the fact.

---

## What XAP Does

XAP defines five primitive objects. If your agent can produce and consume them, it can transact with any other XAP-compatible agent, on any platform, with any model, using any settlement rail.

```
AgentIdentity        →  who the agent is, what it can do, what it charges, its track record
NegotiationContract  →  what two agents agreed to, under what terms, with what guarantees
SettlementIntent     →  what value is locked and under what conditions it releases
ExecutionReceipt     →  what happened, what was paid, to whom, with cryptographic proof
VerityReceipt        →  why it happened, with deterministic replay proof
```

Every interaction follows one flow:

```
REGISTER  →  NEGOTIATE  →  EXECUTE  →  SETTLE  →  AUDIT
```

Nothing outside this sequence is part of XAP. Simplicity is a design constraint.

---

## What Makes XAP Different

### Autonomous Negotiation

Other protocols assume a fixed price or a human approval. XAP agents negotiate in real time.

```
OFFER  →  COUNTER  →  ACCEPT  or  REJECT
```

Four states. Time-bound offers. Conditional pricing ("pay $X if completed in 2 seconds, $Y if 5 seconds"). SLA declared before execution begins. Every state transition signed and permanent.

### Conditional Escrow

XAP does not do payments. It does conditional release.

The primitive is not "Agent A pays Agent B." The primitive is: **Agent A locks funds. Agent B performs work. A verifiable condition is checked. Funds release if the condition passes.**

Three verification types: deterministic (API returned 200), probabilistic (quality score above threshold), or human-approved (for high-value transactions). Every failure mode has a pre-declared outcome. No money ever sits in limbo.

### Split Settlement

An orchestrator delegates a task to five specialist agents. Each contributes to the result. XAP distributes payment proportionally in one atomic operation.

Agent A did 40% of the value. Agent B did 30%. Agent C did 20%. Agent D did 10% but only scored 0.82 when the SLA guaranteed 0.90, so Agent D gets a pro-rata reduction. The settlement engine handles this automatically. No invoicing. No reconciliation.

Nobody else does this. Not Stripe. Not x402. Not AP2.

### Verity: The Truth Engine

Every settlement decision is captured with its complete reasoning state. Not just what was decided. Why.

Given the same inputs and the same rules, any decision can be replayed to produce the same outcome deterministically. This is how a human reviews what their agent did three months ago and verifies the outcome was correct. This is how a regulator audits autonomous commerce. This is how enterprises govern agent fleets.

Outcomes are explicit:

```
SUCCESS   →  conditions met, funds released
FAIL      →  conditions not met, funds returned
UNKNOWN   →  verification is ambiguous, declared resolution path executes
DISPUTED  →  one party challenges, deterministic arbitration engages
REVERSED  →  settlement was final but has been reversed via journal entry
```

**UNKNOWN is first-class.** The system never pretends to know something it does not. When a quality check is borderline, when a verification times out, when evidence is insufficient, the system declares uncertainty and follows the pre-declared resolution path. This is what separates a financial protocol from a payment button.

### Append-Only Reputation

An agent's execution history, settlement outcomes, and dispute record are permanently attached to its identity. You cannot erase a bad track record. This makes trust computable rather than assumed.

An agent evaluating a potential counterparty reads its AgentIdentity and sees: 14,823 total settlements, 97% success rate, 12 disputes, 100% dispute resolution rate. That data compounds with every transaction and cannot be faked.

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   XAP Protocol                   │
│        (open standard, this repo, MIT)           │
│                                                  │
│   AgentIdentity    NegotiationContract           │
│   SettlementIntent ExecutionReceipt              │
│   VerityReceipt    RegistryQuery                 │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│                Truth Engine                      │
│         (open source, Rust, MIT)                 │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│              Settlement Engine                    │
│                (Rust core)                        │
└────────────────────────┬────────────────────────┘
                         │
┌────────────────────────▼────────────────────────┐
│            Settlement Adapters                    │
│                                                  │
│   ┌─────────┐  ┌──────────┐  ┌──────────────┐  │
│   │  Stripe  │  │   USDC   │  │  Test (dev)  │  │
│   └─────────┘  └──────────┘  └──────────────┘  │
└─────────────────────────────────────────────────┘
```

XAP is the language. Verity is the truth engine. The settlement engine executes. Adapters move money. Each layer is independent. Replace any adapter without touching the protocol. Replace the engine without touching the schemas.

---

## Quick Look: A Negotiation

```json
{
  "negotiation_id": "neg_8a2f4c1d",
  "state": "OFFER",
  "from_agent": "agent_7f3a9b2c",
  "to_agent": "agent_2d8e5f1a",
  "task": {
    "type": "text_summarization",
    "input_spec": { "format": "plaintext", "max_tokens": 10000 },
    "output_spec": { "format": "plaintext", "max_tokens": 500 }
  },
  "pricing": {
    "amount_minor_units": 500,
    "currency": "USD",
    "conditions": [
      { "metric": "latency_ms", "threshold": 2000, "modifier": 10000 },
      { "metric": "latency_ms", "threshold": 5000, "modifier": 7000 }
    ]
  },
  "sla": {
    "max_latency_ms": 5000,
    "min_quality_score": 8500
  },
  "expires_at": "2026-03-15T14:30:00Z",
  "signature": "ed25519:..."
}
```

This offer says: "Summarize this text. I will pay $5.00 at full rate if you finish in 2 seconds. $3.50 if you take up to 5 seconds. Quality must be at least 0.85. You have until 2:30 PM UTC to respond. Here is my cryptographic signature proving I made this offer."

All amounts are integer minor units. No floating point. Ever.

---

## Quick Look: A Split Settlement

```json
{
  "settlement_id": "stl_4b7c9e2f",
  "negotiation_id": "neg_8a2f4c1d",
  "payer_agent": "agent_7f3a9b2c",
  "payee_agents": [
    { "agent_id": "agent_2d8e5f1a", "share_bps": 6000, "role": "primary_executor" },
    { "agent_id": "agent_9c4b3e7d", "share_bps": 2500, "role": "data_provider" },
    { "agent_id": "agent_platform",  "share_bps": 1500, "role": "orchestrator" }
  ],
  "total_amount_minor_units": 500,
  "currency": "USD",
  "conditions": [
    { "type": "deterministic", "check": "http_status_200" },
    { "type": "probabilistic", "check": "quality_score", "threshold": 8500 }
  ],
  "timeout_seconds": 30,
  "on_timeout": "full_refund",
  "on_partial_completion": "pro_rata",
  "chargeback_policy": "proportional",
  "idempotency_key": "idem_9c3f2a1b",
  "adapter": "stripe",
  "signature": "ed25519:..."
}
```

Shares are basis points (1-10000, must sum to exactly 10000). 60% to the executor, 25% to the data provider, 15% to the orchestrator. If quality verification passes, funds release automatically in one atomic operation. If it fails, declared failure behavior executes. If the buyer later chargebacks via Stripe, the loss is distributed proportionally.

---

## What XAP Is Not

**Not a payment processor.** XAP does not move money. It coordinates when, how much, to whom, and under what conditions money moves. Stripe, USDC, and other rails move the actual funds.

**Not a blockchain protocol.** No chain required. Settles on-chain if desired. Works equally well on traditional rails.

**Not a marketplace.** Does not match buyers to sellers. Defines what happens after they find each other.

**Not model-specific.** GPT, Claude, Gemini, Llama, open-source models. Any agent on any model.

**Not a checkout flow.** Stripe's ACP helps humans buy shirts through ChatGPT. XAP is for when agents hire other agents to do work, autonomously, in milliseconds, without any human present.

---

## The Stack

XAP is part of a three-layer architecture:

| Layer | What | Open/Closed |
|---|---|---|
| **XAP** (this repo) | The open protocol. Schemas, spec, examples. | Open (MIT) |
| **[Agentra Labs](https://github.com/agentralabs)** | The cognitive substrate. Memory, identity, planning, communication. | Open (MIT) |
| **Agentra Rail** | The production implementation. Settlement at scale, enterprise dashboards, Verity replay. | Commercial |

XAP belongs to the community. You do not need Agentra Rail to implement XAP. But if you want production settlement, enterprise governance, and the full Verity truth engine, Rail is the reference implementation.

---

## Schema Reference

| Schema | Path | Status |
|---|---|---|
| `AgentIdentity` | [`/xap/schemas/agent-identity.json`](xap/schemas/agent-identity.json) | 🔄 Hardening |
| `NegotiationContract` | [`/xap/schemas/negotiation-contract.json`](xap/schemas/negotiation-contract.json) | 🔄 Hardening |
| `SettlementIntent` | [`/xap/schemas/settlement-intent.json`](xap/schemas/settlement-intent.json) | 🔄 Hardening |
| `ExecutionReceipt` | [`/xap/schemas/execution-receipt.json`](xap/schemas/execution-receipt.json) | 🔄 Hardening |
| `VerityReceipt` | [`/xap/schemas/verity-receipt.json`](xap/schemas/verity-receipt.json) | 🔄 Hardening |

All schemas are JSON Schema Draft 2020-12. Validation test suite coming soon.

---

## Design Principles

**Escrow over payment.** The primitive is conditional release, not transfer. Funds lock, conditions verify, funds release or return.

**UNKNOWN over assumption.** When verification is ambiguous, the system declares uncertainty explicitly. It never pretends to know.

**Deterministic over probabilistic.** Every failure mode has a pre-declared outcome. Every decision is replayable. No undefined behavior in a financial protocol.

**Agent-native over human-friendly.** Schemas are machine-readable first. If an LLM can parse the spec and integrate without human help, the design is working.

**Append-only truth.** Reputation is never deleted. Receipts are never amended. Corrections happen through new entries, never edits. The past is permanent.

**Protocol over product.** XAP defines behavior, not implementation. Any system that produces and consumes the five primitives correctly is XAP-compatible.

---

## How To Implement XAP

An XAP-compatible system must:

1. **Produce valid objects** that pass JSON Schema validation
2. **Sign every object** using Ed25519
3. **Enforce state machines** for negotiation and settlement flows
4. **Handle idempotency** so retries are safe
5. **Issue receipts** for every settled transaction
6. **Capture decisions** with replayable reasoning state

If your system does these six things, it is XAP-compatible.

---

## Contributing

XAP is early. The most valuable contributions right now are thinking, not code.

**Schema feedback** (most urgent): Read the schemas. Try to build with them. Tell us where they break. Label: `schema-feedback`

**Edge cases**: What happens in the weird situations? Every edge case found now prevents a breaking change later. Label: `edge-case`

**Vertical schemas**: Agents in finance, healthcare, legal, logistics all need domain-specific capability definitions. Label: `vertical-schema`

**Security review**: Find a vulnerability. We want to know. See [SECURITY.md](SECURITY.md) for responsible disclosure.

Read the [Contributing Guide](CONTRIBUTING.md) before opening a PR.

---

## Roadmap

| Milestone | Status |
|---|---|
| v0.1 Draft schemas | ✅ Published |
| Schema hardening (v0.2) | 🔄 In progress |
| Protocol specification document | 🔜 Next |
| Verity truth engine (Rust) | 🔜 Planned |
| Python SDK (`pip install xap-sdk`) | 🔜 Planned |
| Validation test suite | 🔜 Planned |
| v1.0 Specification lock | 🔜 Target: 2026 |

---

## Community

**Discord:** [agentralabs](https://discord.gg/agentralabs)
**X:** [@agentralab](https://x.com/agentralab)
**Email:** hello@agentralabs.tech

---

## License

MIT. The protocol is free. Forever. The goal is adoption, not control.

---

<p align="center">
  <em>XAP is maintained by <a href="https://www.agentralabs.tech">Agentra Labs</a>. The protocol belongs to the community.</em>
</p>
