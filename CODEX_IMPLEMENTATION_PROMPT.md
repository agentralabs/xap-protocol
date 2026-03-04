You are implementing the ACP (Agent Commerce Protocol) 
Python reference library.

Read these files first before writing any code:
1. CODEX_IMPLEMENTATION_NOTE.md — your full implementation brief
2. schemas/agent-identity.json — source of truth for AgentIdentity
3. schemas/negotiation-contract.json — source of truth for NegotiationContract
4. schemas/settlement-intent.json — source of truth for SettlementIntent
5. schemas/execution-receipt.json — source of truth for ExecutionReceipt

Implementation rules:
- The schemas are the source of truth. 
  If the note conflicts with a schema, the schema wins.
- Start with crypto.py. Nothing else works without it.
- Every class must round-trip: create → to_dict → from_dict → verify
- Every mutation endpoint must require an idempotency_key
- Every state transition must be validated — invalid transitions 
  raise ACPStateError
- All timestamps are ISO 8601 UTC
- Run the full test suite after each module. 
  Do not move to the next module until tests pass.

Build order:
1. crypto.py — Ed25519 key generation, signing, verification
2. identity.py — AgentIdentity create, sign, verify, serialize
3. negotiation.py — NegotiationContract state machine
4. settlement.py — SettlementIntent escrow logic
5. receipt.py — ExecutionReceipt issuance and verification
6. tests/ — one test file per module

When you finish each module, tell me:
- What you built
- What the tests cover
- What you had to invent that the schema did not specify
  (these are schema gaps — flag them explicitly)

Do not implement VerityReceipt. It is v0.2.
Do not implement settlement adapters (Stripe/USDC). 
That is Rail's job, not the SDK's job.

The SDK's job is: produce valid, signed, schema-compliant 
ACP objects. Nothing more.
