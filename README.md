# Energy Oracle Solochain

## About the Project

This is a standalone blockchain built with Substrate (Solochain).

The project implements a custom pallet called **Oracle**, responsible for recording energy deltas on-chain by network and timestamp.

The goal is to explore runtime logic development with FRAME, structured storage, and on-chain event emission.

---

## Pallet Oracle

Location:
pallets/oracle/src/lib.rs

### Objective

Record energy variations (`delta_input`, `delta_output`) associated with:

- `network_id`
- `oracle_timestamp`

Each record is unique by the combination of `network_id + timestamp`.

---

## Storage

The pallet uses a `StorageDoubleMap`:

EnergyLedger:

Key 1:
- `network_id: u32`

Key 2:
- `oracle_timestamp: u64`

Value:
- `(delta_input: u32, delta_output: u32)`

Hashing:
- Blake2_128Concat

This model allows multiple independent networks, each with its temporal history.

---

## Dispatchables

### submit_delta

Parameters:

- `network_id: u32`
- `delta_input: u32`
- `delta_output: u32`
- `oracle_timestamp: u64`

Function:

- Checks if the timestamp already exists for the network
- Inserts the delta into the ledger
- Emits event `DeltaStored`

Overwrite protection:

If a record already exists for the same `network_id` and `timestamp`, the transaction fails with:

TimestampAlreadyExists

---

## Events

DeltaStored(
    network_id,
    oracle_timestamp,
    delta_input,
    delta_output
)

Emitted whenever a new delta is successfully stored.

---

## Errors

TimestampAlreadyExists

Prevents overwriting historical data that has already been recorded.

---

## Current Status

- Node compiles and runs
- Pallet integrated into the runtime
- StorageDoubleMap functional
- Overwrite protection implemented
- Events working correctly
- Tested via Polkadot.js

---

## Running Locally

Build:

cargo build --release

Run in development mode:

./target/release/your-node --dev

Connect via Polkadot.js:

ws://localhost:9944

---

## Project Structure

- node/ → Node implementation
- runtime/ → Runtime configuration and pallet composition
- pallets/oracle/ → Custom pallet implementation

---

## Technologies Used

- Rust
- Substrate FRAME
- StorageDoubleMap
- Aura (block production)
- GRANDPA (finality)

---
