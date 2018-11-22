# Holochain Development Kit for Non-Axiomatic Semantic Graphing
##### Submodule of the `Non-Axiomatic Reasoner` project.

## Overview
`non-axiomatic-grapher-hdk` is a library for Rust-based holochain dApps that makes it easier to develop Holochain Zomes. With Holochain, Zome functions and validation code are represented as WASM binaries. This library provides bindings for Rust.

This HDK is part of the mother project in the field of AGI - further specs for which can be found here: [Non-Axiomatic Reasoner](https://hackmd.io/c/SyoJjvM0X/%2FpjnXpL-OTq--ctgVEQ-vXQ).

## Usage
First, [Rust](https://www.rust-lang.org/en-US/install.html) must be installed on your computer.

Being a Rust library, `non-axiomatic-grapher-hdk` can be added as a dependency to any Rust crate. When you generate Rust based Zomes with [holochain-cmd](https://github.com/holochain/holochain-cmd) add the HDK to the hApp's `Cargo.toml` in the field labelled `hdk`, where `hdk-rust` should already be present.

Documentation for usage will be available shortly.

### Specification for App Development
 - The Narsese Grammar macros can be used simply by calling the macro name with a bang and parentheses, like `<macro_name>!()`. 
  - One macro is implemented for each non-terminal from the Narsese Grammar, with the entirety of the EBNF rules translated to macros.
  - Further specs here: [Dev Storyboard - grammar](https://hackmd.io/hHe_QQUqTTet5k2KbUN5tw).
   
 - Provides tools to write to/read from the graph memory w/ helpful tools specific to Narsese.
 - Implements grammar-to-graph-components' functionality - includes functions to convert to Holochain-interpretable data, and convert back to raw readable format for low-level interpretation.
  - Further specs here: [Dev Storyboard - components](https://hackmd.io/c8JwTvlqQ2mIbl3I7mtdKw).
   
 - Implements functionality for the Inference Rules defined in Appendix B. 
  - Further specs here: [Dev Storyboard - inference](https://hackmd.io/6cRTtOFsT3O1EC_RO-4luA).
  
## Contribute
 - The Non-Axiomatic Reasoner is an open source project. To contribute, contact me via email -> [Suraj Jena](jena.suraj.k@gmail.com).
 - Holochain is also an open source project. To contribute, check out [contributing guidelines](https://github.com/holochain/org/blob/master/CONTRIBUTING.md) for our general practices and protocols on participating in the community.

## Built on (& for)

[![Built for Holochain](https://holochain.org/assets/images/holochain/Holochain_logo.png)](http://holochain.org/)
[![Chat](https://img.shields.io/badge/chat-chat%2eholochain%2enet-blue.svg?style=flat-square)](https://chat.holochain.net)
