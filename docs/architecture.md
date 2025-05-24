# Javy Bless Plugins Documentation

The Javy Bless Plugins project provides a bridge for JavaScript applications running in the [Javy runtime](https://github.com/bytecodealliance/javy) to access powerful host functionalities offered by the [Blockless Runtime (`bls-runtime`)](https://github.com/blocklessnetwork/bls-runtime).
It achieves this by leveraging the [`blockless-sdk`](https://github.com/blocklessnetwork/sdk-rust) to interact with the host and then exposing these capabilities to the JavaScript environment via Javy's plugin API.

This essentially allows JavaScript developers to easily leverage complex backend functionalities provided by the `bls-runtime` or `b7s-browser` (browser runtime) without needing to write Rust or directly deal with WASM ABIs, thanks to the Javy Bless Plugins acting as an intermediary.

## Overview

When you write JavaScript code to be executed by Javy, and you want to, for example, make an HTTP request or interact with an LLM, you're not directly calling `bls-runtime` host functions. Instead:

1. Your JavaScript code calls functions/objects provided by the Javy Bless Plugins (e.g., `BlessLLM()`, `fetch()`).
2. These JavaScript functions are implemented (or "backed") by Rust code within the `javy-bless-plugins` WASM module.
3. This Rust code, in turn, uses the `blockless-sdk` to make the actual calls to the `bls-runtime`'s (or browser runtime's) host functions.
4. The runtime executes these host functions (e.g., performing the HTTP request via its HTTP driver).
5. Results are passed back up the chain to your JavaScript code.

## Architecture

The following diagram illustrates the relationship between your JavaScript application, Javy Bless Plugins, the Blockless Rust SDK, and the `bls-runtime`.

```mermaid
graph TD
    subgraph "User Application Layer"
        JSApp["JavaScript Application Code (.js)"] -- "Calls" --> JavyPluginsJS_API["Javy Bless Plugins <br/> (JavaScript API, e.g., BlessLLM, fetch)"];
    end

    subgraph "Javy Runtime with Bless Plugins"
        direction LR
        Javy["Javy Runtime (QuickJS)"] -- "Executes" --> JSApp;
        JavyPluginsWASM["javy-bless-plugins <br/> (bless_plugins.wasm)"] -- "Provides Implementation for" --> JavyPluginsJS_API;
        JavyPluginsWASM -- "Contains Rust Glue Code" --> JavyPluginRust["Rust Plugin Logic <br/> (src/lib.rs, src/llm/*, etc.)"];
    end

    subgraph "Blockless SDK & Runtime Interaction (within javy-bless-plugins.wasm)"
        direction LR
        JavyPluginRust -- "Uses" --> BlocklessSDK["blockless-sdk"];
        BlocklessSDK -- "Makes Host Calls via ABI" --> BLSRuntimeABI["bls-runtime Host ABI"];
    end

    subgraph "Blockless Runtime Environment"
        direction LR
        BLSRuntime["bls-runtime Engine"] -- "Implements" --> BLSRuntimeABI;
        BLSRuntime -- "Manages" --> HostDrivers["Host Drivers (HTTP, LLM, etc.)"];
        HostDrivers -- "Access" --> ExternalResources["External Resources (Network, Files, Services)"];
    end

    JSApp -.-> Javy;
    Javy -.-> JavyPluginsWASM;

    style JSApp fill:#C9FFD4,stroke:#333,stroke-width:2px
    style JavyPluginsJS_API fill:#E8DAEF,stroke:#8E44AD,stroke-width:2px
    style Javy fill:#A2D9CE,stroke:#333,stroke-width:2px
    style JavyPluginsWASM fill:#F9E79F,stroke:#B7950B,stroke-width:2px
    style JavyPluginRust fill:#AED6F1,stroke:#1B4F72,stroke-width:2px
    style BlocklessSDK fill:#F5B7B1,stroke:#943126,stroke-width:2px
    style BLSRuntimeABI fill:#D2B4DE,stroke:#6C3483,stroke-width:2px
    style BLSRuntime fill:#87CEFA,stroke:#333,stroke-width:2px
    style HostDrivers fill:#F4A460,stroke:#333,stroke-width:2px
    style ExternalResources fill:#FFB6C1,stroke:#333,stroke-width:2px
```

**Key Components:**

* **JavaScript Application Code (.js):** This is the script you write, utilizing the global objects and functions exposed by Javy Bless Plugins.
* **Javy Runtime (QuickJS):** The JavaScript engine that executes your `.js` code.
* **Javy Bless Plugins (JavaScript API):** These are the JavaScript interfaces (e.g., `BlessLLM` constructor, `fetch` function) that your application code interacts with. These are defined in helper `.js` files (like `src/fetch/fetch.js`) which are embedded into the Javy environment by the plugin.
* **javy-bless-plugins (bless\_plugins.wasm):** This is the core WebAssembly module of the Javy Bless Plugins. It's built from Rust and includes:
    * **Rust Plugin Logic:** The Rust code (e.g., in `src/llm/mod.rs`, `src/fetch/mod.rs`) that implements the functionality for the JavaScript APIs. This code uses `javy-plugin-api` to bridge between Rust and QuickJS.
    * **blockless-sdk:** The Javy Bless Plugins Rust code depends on `blockless-sdk` to make the actual calls to the `bls-runtime`.
* **bls-runtime Host ABI:** The low-level Application Binary Interface through which WebAssembly modules (including `bless_plugins.wasm` via `blockless-sdk`) communicate with the `bls-runtime`.
* **bls-runtime Engine:** The execution environment that runs the Javy WASM module (which itself contains your JS code and the Bless plugins). It handles host function calls and delegates them to appropriate drivers.
* **Host Drivers:** Specialized modules within `bls-runtime` that perform the actual I/O or computation (e.g., making an HTTP request, interacting with an LLM).

### Build Process Flow

The `javy-bless-plugins` themselves are first compiled into a WASM module.
Then, Javy is used to embed your application JavaScript *into* another WASM module, linking against the `bless_plugins.wasm`.

```mermaid
graph TD
    A["Rust Plugin Source Code (e.g., src/llm/mod.rs, src/fetch/mod.rs)"] -- "Uses" --> B[blockless-sdk];
    A -- "Uses" --> JAPI[javy-plugin-api];
    B --> C["`cargo build --target wasm32-wasip1`"];
    JAPI --> C;
    C --> D["Initial WASM <br/> (target/.../bless_plugins.wasm)"];
    D --> E["javy init-plugin <br/> (Wraps with Javy ABI)"];
    E --> F["Javy-Compatible Plugin <br/> (bless_plugins.wasm in project root)"];
    G["User JavaScript Application <br/> (e.g., examples/llm.js)"];
    F --> H["javy build -C plugin=bless_plugins.wasm ..."];
    G --> H;
    H --> I["Final Executable WASM <br/> (e.g., bless-llm.wasm)"];
    I --> J[Run in WASM runtime];

    style A fill:#AED6F1,stroke:#1B4F72
    style B fill:#F5B7B1,stroke:#943126
    style JAPI fill:#E8DAEF,stroke:#8E44AD
    style C fill:#A3E4D7,stroke:#1D8348
    style D fill:#A3E4D7,stroke:#1D8348
    style E fill:#A3E4D7,stroke:#1D8348
    style F fill:#F9E79F,stroke:#B7950B
    style G fill:#C9FFD4,stroke:#333
    style H fill:#A3E4D7,stroke:#1D8348
    style I fill:#98FB98,stroke:#333
    style J fill:#87CEFA,stroke:#333
```

1. **Plugin Compilation:**
  * The Rust source code for Javy Bless Plugins (which uses `blockless-sdk` and `javy-plugin-api`) is compiled into a raw WASM file (`target/.../bless_plugins.wasm`).
  * `javy init-plugin` processes this raw WASM, making it compatible with Javy's plugin

2. **Application Compilation:**
  * You write your JavaScript application (e.g., `examples/llm.js`).
  * `javy build` takes your JavaScript file and the Javy-compatible `bless_plugins.wasm` as input.
  * It compiles your JavaScript into QuickJS bytecode and bundles it with the QuickJS engine and the `bless_plugins.wasm` into a final executable WASM module (e.g., `bless-llm.wasm`).

3. **Execution:**
  * This final WASM module is then executed by the `bls-runtime`.

## Available Plugins (Exposed to JavaScript)

Javy Bless Plugins currently expose the following functionalities to JavaScript, each mapping to a corresponding module in the blockless-sdk-rust:

* BlessLLM (via blockless_llm):
  * Allows JavaScript to initialize LLM sessions, set options, and make chat requests.
  * Also exposes MODELS object for predefined model names.
* fetch (via blockless_http):
    * Provides a fetch-like API for making HTTP requests.
* crypto.getRandomValues (via host's random number generation capabilities, likely through WASI or a custom Blockless extension):
    * Provides a way to get cryptographically strong random values, mimicking the Web Crypto API.

The `src/lib.rs` file in `javy-bless-plugins` is crucial as it initializes the Javy runtime context, registers these global JavaScript objects/functions, and maps them to their underlying Rust implementations.
