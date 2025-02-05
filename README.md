<div align="center">
    <h1>Telchar</h1>
    <img alt="GitHub" src="https://img.shields.io/github/license/txpipe/telchar" />
    <img alt="Release" src="https://github.com/txpipe/telchar/actions/workflows/release.yml/badge.svg" />
    <img alt="Publish" src="https://github.com/txpipe/telchar/actions/workflows/publish.yml/badge.svg" />
    <hr/>
</div>

## Introduction

Telchar will be a toolchain that improves the developer experience of integrating Plutus validators in off-chain processes. We'll leverage the information provided in blueprint files (CIP-57) to improve on two areas: discovery and codegen (code generation).

- **discovery**: we'll implement a public registry of validator interfaces, allowing authors and 3rd-parties to discover and integrate on-chain protocols easily.
- **codegen**: the toolchain will include a mechanism to generate boilerplate code for different languages and libraries, allowing developers to build txs that interact with on-chain protocols directly, without having to deal directly with the implementation details of each one.

The toolchain will include 3 different deliverables (components):

## Blueprint Registry

- publish blueprints: authors of on-chain validators will be able to publish their blueprint files as part of their development workflow for others to discover.
- search blueprints: protocol integrators will be able to search existing blueprint files and retrieve the interface definition data, both programmatically or manually.
- parametrized scripts: scripts that depend on parameters will be supported by the registry, allowing integrators to customize them for concrete uses.
- known parameterizations: the registry will allow devs to post known parameterizations to be linked to their source blueprint, allowing a direct match to on-chain transactions.
- reference scripts: the registry will allow devs to track on-chain UTxOs that holds scripts for particular blueprint so that consumers can use them as reference scripts in their own transactions.
- source code link: devs will have metadata fields available to link complementary information to the blueprint, such as source code location (if relevant)
- public registry: a hosted version of the registry will be publicly available and will be offered as default instance, but not mandatory.
- private registry: teams will be able to run their own independent version of the registry for either private workflows or to avoid dependencies on external services.

## Telchar CLI

- publish command: a CLI command to upload blueprint files from the local development environment onto the registry.
- codegen: a CLI command to automatically generate code for type declarations from blueprint files to be used in libraries such as lucid, meshsdk, pycardano, plu-ts, pallas and potentially other tx builders in the ecosystem.
- plutus data builder: an interactive CLI command for devs to build Plutus data payloads that match the blueprint specification. This can be used for testing or tx building.
- deploy reference script: a CLI command to deploy fully-defined scripts as on-chain utxos, making them available to be used as reference scripts by other transactions (Plutus v2).
- github actions: the CLI will be available as a Github action, allowing devs to use the tools as part of their continuous integration pipelines.

## Telchar library

- Rust crate: the same functionality provided in the CLI will be available as Rust crate for other tooling developers to embed similar flows into their own tools.
- FFI bindings: we'll include FFI bindings for languages such as Python, NodeJs and potentially any FFI capable language, allowing other tooling developers to embed similar flows into their own tools.
- Blueprint parser: the library will contain functions to parse Blueprint files into memory structs, allowing devs to use them as primitives in complex workflows.
- Registry client: the library will provide a client implementation, allowing devs to interact with blueprint registries programmatically.
