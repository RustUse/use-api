# use-webhook

Webhook primitives for RustUse.

Models event names, endpoint URL wrappers, delivery identifiers, attempts, delivery status labels, signature header names, and event envelope metadata without delivering HTTP requests.

## Example

```rust
use use_webhook::{DeliveryStatus, WebhookEventName};

let value = WebhookEventName::new("user.created").expect("valid API primitive");

assert_eq!(value.as_str(), "user.created");
assert_eq!(DeliveryStatus::Pending.to_string(), "pending");
```

## Scope

- Small validated types for API metadata.
- Stable display labels and conservative parsing helpers.
- Dependency-light primitives that can be reused by other RustUse crates.

## Non-goals

- No HTTP delivery.
- No signature verification.
- No retry scheduler.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license