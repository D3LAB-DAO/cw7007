# CW7007 Smart Contract

This project implements the CW7007 standard, an extension of the CW721 token standard for verifiable AI-generated content (AIGC) tokens.
The CW7007 facilitates the creation of NFTs containing AI-generated content, providing interfaces for `mint`ing and `verify`ing tokens.

The contract is built in Rust using the [Archway Network starter pack](https://github.com/archway-network/cli) and follows the [ERC-7007 specification](https://eips.ethereum.org/EIPS/eip-7007).

<!--
> TODO: Implement example of `verify` function.
-->

## Key Features

- Implements the CW version of ERC-7007 standard with traits.
- Supports OpenSea's metadata extensions format.

---

# Quick Start

## Deploy

```bash
$ archway contracts build

$ archway contracts store cw7007

$ archway contracts instantiate cw7007 --args '{
  "name": "Gateway CW7007",
  "symbol": "G7007",
  "minter": "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a",
  "prompt": "You are a cat. Respond only with one or various cat sounds such as 'MEOW,' 'PURR,' 'HISS,' 'GROWL,' 'CHIRP,' 'TRILL,' 'YOWL,' or 'CATERWAUL,' along with an action in parentheses that a cat would do, such as (purring), (stretching), or (chasing a mouse). Feel free to include any other sounds and actions that a cat might make or do beyond these examples."
}'
```

## Metadata & Premiums

```bash
$ archway contracts metadata cw7007 --owner-address "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a" --rewards-address "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a"

# archway contracts premium cw7007 --premium-fee "1000000000000000000aconst"
```

## Execute

```bash
$ archway contracts execute cw7007 --args '{
  "mint": {
    "token_id": "0",
    "owner": "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a",
    "extension": {
        "description": "Hello"
    }
  }
}'

$ archway contracts execute cw7007 --args '{
  "response": {
    "token_id": "0",
    "output": "World"
  }
}'
```

## Query

```bash
$ archway contracts query smart cw7007 --args '{
  "nft_info": {
    "token_id": "0"
  }
}'

$ archway contracts query smart cw7007 --args '{"prompt": {}}'
$ archway contracts query smart cw7007 --args '{"request_ids": {}}'
$ archway contracts query smart cw7007 --args '{"num_tokens": {}}'
```
