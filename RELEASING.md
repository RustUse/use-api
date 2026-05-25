# Releasing

This repository uses a focused-first release flow rather than publishing the
`use-api` facade crate first.

## Current release state

Publish the focused crates before the `use-api` umbrella crate. The facade comes
last after the focused crates are visible on crates.io.

## Current automation

The repository includes the release files and workflows that match this shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`

## Initial publish order

For the first public crates.io wave, publish in this order:

1. `use-rest`
2. `use-openapi`
3. `use-graphql`
4. `use-grpc`
5. `use-rpc`
6. `use-endpoint`
7. `use-api-route`
8. `use-api-version`
9. `use-pagination`
10. `use-cursor`
11. `use-rate-limit`
12. `use-api-error`
13. `use-api-key`
14. `use-webhook`
15. `use-idempotency`
16. `use-content-negotiation`
17. `use-api-auth`
18. `use-api-request`
19. `use-api-response`
20. `use-api-schema`
21. `use-api-param`
22. `use-api-header`
23. `use-api-media-type`
24. `use-api-deprecation`
25. `use-api-resource`
26. `use-api-operation`
27. `use-api`

## Validation

Run the full local validation path before publishing:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```

## Follow-up release automation

After the initial manual crates.io wave is complete, the repository can use the
`release-plz` workflows for follow-up releases.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request from `release-plz.toml`

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: manual dispatch only
- Required input: `post-initial-release = true`
- Required secret: `CARGO_REGISTRY_TOKEN`

The release publish workflow confirms that every focused crate already exists on
crates.io before it runs `release-plz release`.

## Permanent version warning

Published crates.io versions are permanent. Verify the crate metadata,
packaging, and changelog inputs before any real publish.
