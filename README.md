# Simple Pallet for an Oracle Event Feed

- An event is arbitrary length bytes

- Only a single authorised account may post an event

- The pallet should store the last 1 hour of events

- Notes down any known security issues, or things to be improved if you are running out of time

**Important Note:**
- The pallet is at pallets/oracle-event-feed
- Unit test is added at /oracle-event-feed/tests.rs#L78 to verify the functionality

```
cargo test --package oracle-event-feed --lib -- tests::test --exact --nocapture
```


**Outstanding Items:**
- fixed length bytes to arbitrary length
- removing events after 1 hour
- code cleanup, handle edge cases and auditing for security considerations
