<!-- BEGIN README.md -->
# Star – Honorary LP Fee Router (Bounty Submission)

This repo contains a minimal, Anchor-compatible Solana program that:

* mints a quote-only LP position in a DAMM v2 pool, and  
* once per day “cranks” the claimed quote fees, streaming them to investors pro-rata and forwarding the remainder to the creator wallet.

## Build / Test locally

```bash
# Install Anchor CLI if you haven't already
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# run unit + integration tests on a local validator
anchor test
