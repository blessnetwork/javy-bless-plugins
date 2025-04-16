# Bless Javy Plugins

These are the plugins for the [Javy](https://github.com/blessnetwork/bls-javy) runtime.

## Plugins

| Plugin | Description | [Browser Runtime](https://github.com/blocklessnetwork/b7s-browser) supported | [Native Runtime](https://github.com/blessnetwork/bls-runtime) supported |
|--------|-------------|--------------------------|--------------------------|
| `BlessLLM` | A plugin for interacting with LLMs | ✅ | ✅ |
| `BlessFetch` | A plugin for interacting with HTTP / fetch | ✅ | ✅ |
| `BlessCrypto` | A plugin for interacting with the crypto library | ✅ | ✅ |
| `Base64` | A plugin for base64 encoding and decoding | ✅ | ✅ |

## Architecture

```mermaid
flowchart TD
    subgraph "Rust Source Code"
        BC["BlessCrypto"]:::source
        BF["BlessFetch"]:::source
        BL["BlessLLM"]:::source
        B64["Base64"]:::source
    end

    subgraph "Build Pipeline"
        CBP["Cargo Build Process"]:::build
        IWP["Initial Wasm Plugin (bless_plugins.wasm)"]:::build
        JI["javy init-plugin"]:::build
        JB["javy build"]:::build
        FWP["Final Wasm Plugin (bless-llm.wasm)"]:::output
        EJS["Example JS"]:::js
    end

    subgraph "Deployment"
        BROWSER["Browser Runtime"]:::deploy
        NATIVE["Native Runtime"]:::deploy
    end

    subgraph "Build Tools"
        MF["Makefile"]:::automation
        GHA["GitHub Actions (.github/workflows)"]:::automation
    end

    %% Connections: Rust Modules to Build Pipeline
    BC -->|"compile"| CBP
    BF -->|"compile"| CBP
    BL -->|"compile"| CBP
    B64 -->|"compile"| CBP

    %% Build Pipeline Flow
    CBP -->|"wasm_build"| IWP
    IWP -->|"plugin_wrap"| JI
    JI -->|"js_integration"| JB
    EJS -->|"supplies_js"| JB
    JB -->|"produce"| FWP

    %% Deployment Connections
    FWP -->|"deploy"| BROWSER
    FWP -->|"deploy"| NATIVE

    %% Build Tools Automating Build Process
    MF ---|"automates"| CBP
    GHA ---|"automates"| CBP

    %% Click Events
    click BC "https://github.com/blessnetwork/javy-bless-plugins/tree/main/src/crypto"
    click BF "https://github.com/blessnetwork/javy-bless-plugins/tree/main/src/fetch"
    click BL "https://github.com/blessnetwork/javy-bless-plugins/tree/main/src/llm"
    click B64 "https://github.com/blessnetwork/javy-bless-plugins/tree/main/src/b64"
    click EJS "https://github.com/blessnetwork/javy-bless-plugins/blob/main/examples/llm.js"
    click CBP "https://github.com/blessnetwork/javy-bless-plugins/blob/main/Cargo.toml"
    click MF "https://github.com/blessnetwork/javy-bless-plugins/tree/main/Makefile"
    click GHA "https://github.com/blessnetwork/javy-bless-plugins/tree/main/.github/workflows"

    %% Styles
    classDef source fill:#AED6F1,stroke:#1B4F72,stroke-width:2px;
    classDef build fill:#A3E4D7,stroke:#1D8348,stroke-width:2px;
    classDef output fill:#F9E79F,stroke:#B7950B,stroke-width:2px;
    classDef deploy fill:#F5B7B1,stroke:#943126,stroke-width:2px;
    classDef automation fill:#D2B4DE,stroke:#6C3483,stroke-width:2px;
    classDef js fill:#E8DAEF,stroke:#8E44AD,stroke-width:2px;
```

## Pre-Requisites

- [javy-cli](https://github.com/javy-dev/javy-cli)
  - Get the latest release from [here](https://github.com/bytecodealliance/javy/releases)

## Build

```sh
# build bless plugins
cargo build --target=wasm32-wasip1 --release

# rebuild the plugin-wasm with javy runtime CLI
javy init-plugin ./target/wasm32-wasip1/release/bless_plugins.wasm -o bless_plugins.wasm

# compile javascript to wasm with javy runtime and plugin
javy build -C plugin=bless_plugins.wasm ./examples/llm.js -o bless-llm.wasm
```
