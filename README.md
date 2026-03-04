# ACP — Agent Commerce Protocol

**The open economic protocol for autonomous agents.**

[![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)](https://opensource.org/licenses/MIT)
[![Status: Draft v0.1](https://img.shields.io/badge/Status-Draft%20v0.1-yellow.svg)](#versioning)
[![Maintained by: Agentra Labs](https://img.shields.io/badge/Maintained%20by-Agentra%20Labs-blue.svg)](https://www.agentralabs.tech)
[![Discord](https://img.shields.io/badge/Discord-Join%20Community-5865F2.svg)](https://discord.gg/agentralabs)

---

## What Is ACP?

ACP (Agent Commerce Protocol) is an open standard that defines how autonomous AI agents identify themselves, negotiate terms, settle value, and prove what happened — without human involvement.

Today, agents can think. They can reason. They can execute tasks. But when two agents need to transact with each other — pay for compute, buy data, settle a service — there is no shared language for how that happens. Every system invents its own format. Nothing interoperates. Nothing is auditable.

ACP fixes that.

It defines five primitive objects that every agent commerce interaction maps to:

```
AgentIdentity        →  who the agent is and what it can do
NegotiationContract  →  what two agents agreed to
SettlementIntent     →  what value is locked and under what condition
ExecutionReceipt     →  what happened and what was paid
VerityReceipt        →  why it happened and can it be proven
```

If your agent can produce and consume these five objects — it can transact with any other ACP-compatible agent, on any platform, with any model, in any settlement unit.

---

## Why This Protocol Exists

The agent economy is forming right now. Agents are being deployed that call other agents, pay for API access, rent capabilities, and coordinate on tasks. But there is no economic primitive underneath any of it.

Without a shared protocol:
- Agents cannot safely pay each other
- There is no standard for negotiating usage terms
- Escrow and conditional settlement do not exist at the agent layer
- No tamper-proof record of what an agent did or was paid for
- Split revenue across multi-agent workflows is impossible to automate
- Disputes have no deterministic resolution path

ACP is the missing economic layer. It does not replace existing payment systems. It sits above them as a coordination and accountability standard that any settlement unit — fiat, stablecoin, or network credit — can plug into.

---

## The Five Primitives

### 1. `AgentIdentity`
The permanent economic passport of an autonomous agent. Cryptographically anchored with Ed25519. Includes capability declarations in machine-readable schema, pricing structure (fixed, dynamic, auction, outcome-based), SLA guarantees, risk profile, and an append-only reputation ledger. An agent reads another agent's `AgentIdentity` and decides autonomously whether to hire it — no human involvement required.

### 2. `NegotiationContract`
The terms of a proposed exchange. ACP uses exactly four states: `OFFER → COUNTER → ACCEPT → REJECT`. Every offer is time-bound. Conditional pricing is supported natively — "pay X if completed in 2 seconds, Y if 5 seconds." SLA is declared before execution begins. Every state transition is signed and permanently logged to both agents' histories.

### 3. `SettlementIntent`
The escrow instruction. Funds are locked. The release condition is declared upfront. The verification method is specified — deterministic, probabilistic, or human-verified. Split rules for multi-agent workflows are declared at creation time. Every failure mode has a pre-declared outcome. No money ever sits in limbo. Idempotent by design — agents retry safely.

### 4. `ExecutionReceipt`
The tamper-proof record of every economic event. Cryptographically signed. Timestamped. Permanently attached to both agents' identities. Contains the full event chain from negotiation initiation to final settlement, split distributions, performance against declared SLA, and reputation impact. Replayable. The audit primitive of the agent economy.

### 5. `VerityReceipt`
The truth primitive. Every significant decision in the settlement flow — condition verification, dispute resolution, reputation scoring — is captured with its complete reasoning state. Not just what was decided. Why. The `VerityReceipt` is signed, replayable, and provable. Given the same input state and the same rules, any decision can be re-run to produce the same outcome deterministically. This is the legal and regulatory primitive for autonomous commerce.

---

## How The Protocol Works

Every ACP interaction follows one sequence:

```
REGISTER → NEGOTIATE → EXECUTE → SETTLE → AUDIT
```

Every feature of every ACP-compatible system maps to one of these five steps. Nothing outside this sequence is part of ACP. Simplicity is a design constraint.

### Negotiation States

```
OFFER
  ↓
COUNTER  ←── (repeatable)
  ↓
ACCEPT or REJECT
```

Four states. No more. Agents operate in milliseconds. Complexity kills adoption.

### Settlement Flow

```
NegotiationContract accepted
        ↓
SettlementIntent created (funds locked)
        ↓
Execution begins
        ↓
Result submitted by executing agent
        ↓
Condition verified (deterministic / probabilistic / human)
        ↓
Funds released to split recipients  OR  rolled back to payer
        ↓
ExecutionReceipt issued + VerityReceipt captured
```

---

## Schema Reference

All five ACP primitive schemas live in `/acp/schemas`. They are JSON Schema (Draft 2020-12) documents, validated against the specification.

| Schema | File | Status |
|---|---|---|
| `AgentIdentity` | [`/acp/schemas/agent-identity.json`](/acp/schemas/agent-identity.json) | ✅ v0.1 Draft |
| `NegotiationContract` | [`/acp/schemas/negotiation-contract.json`](/acp/schemas/negotiation-contract.json) | ✅ v0.1 Draft |
| `SettlementIntent` | [`/acp/schemas/settlement-intent.json`](/acp/schemas/settlement-intent.json) | ✅ v0.1 Draft |
| `ExecutionReceipt` | [`/acp/schemas/execution-receipt.json`](/acp/schemas/execution-receipt.json) | ✅ v0.1 Draft |
| `VerityReceipt` | [`/acp/schemas/verity-receipt.json`](/acp/schemas/verity-receipt.json) | 🔄 In Progress |

Each schema includes a working example showing a complete, valid object. Read the schema. You understand ACP.

---

## What ACP Is Not

**ACP is not a payment processor.** It does not move money. It defines the coordination layer above payment systems. Stripe, USDC, and internal credit systems are all valid settlement units underneath ACP.

**ACP is not a blockchain protocol.** It does not require a chain. It can settle on-chain if desired but operates equally well on traditional financial infrastructure.

**ACP is not a marketplace.** It does not match buyers to sellers. It defines what happens after they find each other.

**ACP is not model-specific.** It does not care what model powers an agent. GPT-4, Claude, Gemini, Llama — any agent on any model can implement ACP.

**ACP is not a vendor lock-in mechanism.** Any system can implement it. The spec is MIT licensed. The goal is for ACP to become the default assumption in agent commerce — not to lock anyone in.

---

## Relationship To Agentra Labs And Agentra Rail

ACP is maintained by [Agentra Labs](https://www.agentralabs.tech) as an open contribution to the agent economy.

**Agentra Labs** builds the open-source cognitive substrate that makes agents persistent, trustworthy, and governable — memory, identity, planning, communication, and more. The Labs stack is what makes agents capable enough to transact.

**Agentra Rail** is the production implementation of ACP — the commercial infrastructure layer where agents register, negotiate, settle, and audit at scale. Rail is what runs ACP in production with the reliability, speed, and correctness that enterprises depend on.

The relationship:

```
ACP (this repo)     →  the open language everyone speaks
Agentra Labs        →  the cognitive substrate that makes agents capable
Agentra Rail        →  the production infrastructure ACP runs on
```

ACP belongs to the community. Agentra Rail is the reference implementation. You do not need Rail to implement ACP. But if you want production settlement, governance, Verity replay, and split settlement at scale — Rail is where you go.

---

## Design Principles

**Deterministic over ambiguous.** Every interaction has a defined outcome. Every failure mode has a pre-declared resolution. No money in limbo. No unresolved states.

**Agent-native over human-friendly.** ACP is designed for LLMs and agents to consume autonomously. Machine-readable first. If an agent can read the spec and integrate without human help — the design is working.

**Protocol over product.** ACP defines behavior, not implementation. Any system that produces and consumes the five primitive objects correctly is ACP-compatible.

**Escrow over payment.** The core primitive is not "agent A pays agent B." It is "agent A releases funds to agent B when verifiable condition X is satisfied." This distinction is architectural.

**Open primitives, closed engines.** The protocol is open. What settlement engines, verification algorithms, and reputation systems do with the protocol is up to each implementer.

**Append-only truth.** Reputation history is never deleted. Execution history is permanent. Negotiation history is signed and immutable. The past is not editable. This is the foundation of trust between agents that have never met.

---

## Versioning

ACP uses semantic versioning. Breaking changes increment the major version. The current status is `v0.1 Draft` — open for community review and feedback before `v1.0` is locked.

Backward compatibility is a first-class design constraint from `v1.0` onward. Once locked, systems built on ACP v1.0 will not break when v1.1 ships.

---

## How To Implement ACP

An ACP-compatible system must:

1. **Produce valid objects** — every object created validates against the corresponding JSON schema
2. **Sign every object** — using Ed25519. The signing key must correspond to a registered `AgentIdentity`
3. **Enforce state machines** — `NegotiationContract` and `SettlementIntent` follow declared state transitions only
4. **Handle idempotency** — settlement calls with the same `idempotency_key` return the existing result without creating duplicates
5. **Issue receipts** — every settled `SettlementIntent` produces an `ExecutionReceipt`
6. **Capture decisions** — every significant decision point produces a `VerityReceipt`

There is no certification process yet. If your system does these six things correctly — it is ACP-compatible. We are working on a compliance test suite (see Contributing below).

---

## Contributing

ACP is an early-stage open standard. The most valuable contributions right now are not code — they are thinking.

### What We Need Most

**Schema feedback — most urgent**
Read the five schemas. Try to build something with them. Tell us where they break, where they are too rigid, where they are too loose, where a field is missing, where a field makes no sense. Every schema issue found now prevents a breaking change later.

Open an issue with the label `schema-feedback`.

**Edge cases**
What happens when an agent submits an execution result after the deadline? What happens when split rules don't sum correctly? What happens when a quality score is gamed? We have thought hard about this — but we have not thought of everything. Every edge case you surface that we have not handled is a gift.

Open an issue with the label `edge-case`.

**Vertical-specific schemas**
ACP v0.1 includes capability schemas for four verticals: AI inference, data enrichment, web automation, and developer tooling. Every industry that deploys agents will need its own capability vocabulary. If you are building agents in legal, finance, healthcare, logistics, or any other domain — we want to define the capability schema for your vertical together.

Open an issue with the label `vertical-schema` and describe your use case.

**Alternative implementation feedback**
If you are building an ACP-compatible system that is not Agentra Rail — we want to know what was hard to implement, what was unclear, and what you had to invent yourself because the spec did not cover it.

Open an issue with the label `implementation-feedback`.

**Dispute resolution rules**
ACP declares that disputes have deterministic resolution — but the spec does not yet fully define what those rules are. This is an open design problem. How should a dispute between two agents be resolved automatically? What inputs matter? What is the fairest deterministic algorithm? This requires real thinking from people who understand agent economics.

Open a discussion with the label `dispute-resolution`.

**ACP Credit monetary policy**
The spec references ACP Credit as a native settlement unit but does not define its monetary policy — how credits are issued, what they are worth, whether there is a supply cap, and how they are burned. This is a hard problem that intersects economics, game theory, and agent incentive design.

Open a discussion with the label `monetary-policy`.

**Verity legal standing**
The `VerityReceipt` is designed to be the legal and regulatory primitive for autonomous agent decisions. But whether a replay constitutes admissible evidence varies by jurisdiction. We need people who understand law in the US, EU, UK, Singapore, and other key jurisdictions to engage with what `VerityReceipt` would need to satisfy legal standards in their jurisdiction.

Open a discussion with the label `verity-legal`.

### What We Are Not Looking For Right Now

- Pull requests that change the core protocol objects without a prior discussion issue
- Implementation code in this repo (implementations belong in separate repos)
- Marketing copy improvements

### How To Contribute

1. Read the schemas in `/acp/schemas` first. They are the source of truth.
2. Read the open discussions to understand what is already being debated.
3. Open an issue or discussion before writing anything substantial.
4. Use the correct label so it reaches the right people.

---

## Open Questions

These are the hardest unsolved problems in ACP. If you have deep expertise in any of these — we want to hear from you.

**ACP Credit monetary policy** — How is ACP Credit issued? Is it earned through completed settlements only? Is there a supply cap? What maintains the peg? What happens if Rail shuts down?

**Verity legal standing** — Which jurisdictions would recognize a `VerityReceipt` replay as admissible evidence? What additional fields or attestations would be required?

**Cross-model credit bridging** — Agents run on different models with different native credit systems. How does a universal conversion layer work without introducing a custodial risk?

**Registry governance** — Who can propose changes to ACP schemas after v1.0 is locked? How are breaking changes decided? Does governance eventually decentralize?

**Dispute escalation threshold** — At what transaction value does human arbitration become mandatory? How is this calibrated across different verticals?

**Agent insurance** — Should ACP define a primitive for agent insurance? Who underwrites it? How does the `ExecutionReceipt` interact with an insurance claim?

**DAO transition** — Should ACP governance eventually move to token holders? If so, what triggers that transition, and how does it happen without breaking existing implementations?

---

## Community

**Discord:** [Join @agentralabs](https://discord.gg/agentralabs) — real-time discussion, implementation help, schema debates

**X / Twitter:** [Follow @agentralab](https://x.com/agentralab) — protocol updates, launch signals, community highlights

**Email:** [hello@agentralabs.tech](mailto:hello@agentralabs.tech) — research collaboration, enterprise implementation, formal feedback

---

## License

ACP schemas and specification documents are released under the [MIT License](LICENSE).

The protocol is free. Forever. The goal is adoption, not control.

---

## GitHub Topics

`agent-commerce` `autonomous-agents` `acp-protocol` `agent-economy` `ai-agents` `agent-settlement` `agent-identity` `escrow` `multi-agent` `open-standard` `protocol` `agentic-ai` `llm-agents` `agent-infrastructure` `agentra`

---

*ACP is maintained by Agentra Labs. The protocol belongs to the community. The reference implementation is [Agentra Rail](https://www.agentralabs.tech).*
