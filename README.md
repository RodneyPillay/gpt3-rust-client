[![Build](https://github.com/RodneyPillay/gpt3-rust-client/actions/workflows/rust.yml/badge.svg)](https://github.com/RodneyPillay/gpt3-rust-client/actions/workflows/rust.yml)
<a href="http://opensource.org/licenses/MIT"><img src="https://img.shields.io/github/license/RodneyPillay/gpt3-rust-client?color=black"></img></a>
<a href="#"><img src="https://img.shields.io/github/issues/RodneyPillay/gpt3-rust-client"></img></a> 
<a href="#"><img src="https://img.shields.io/github/forks/RodneyPillay/gpt3-rust-client"></img></a> 
<a href="#"><img src="https://img.shields.io/github/stars/RodneyPillay/gpt3-rust-client"></img></a>
<a href="#"><img src="https://img.shields.io/github/repo-size/RodneyPillay/gpt3-rust-client"></img></a>
  <a href="https://github.com/RodneyPillay/gpt3-rust-client/graphs/contributors"><img src="https://img.shields.io/github/contributors/RodneyPillay/gpt3-rust-client?color=blue"></img></a>


# Simple Rust Client for OpenAI
Inspired by https://www.youtube.com/watch?v=5WhJQMnJjik

## Getting Started
* [Install rustup](https://www.rust-lang.org/tools/install)
* [Install VS Code](https://code.visualstudio.com/Download) 
    * [Install Plugin rust-analyser](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) for rust integration.
    * [Install Plugin CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for rust debugging.

## General Articles
* [How not to learn Rust](https://dystroy.org/blog/how-not-to-learn-rust/)

## Concepts Encountered In The Client
* [Derive](https://doc.rust-lang.org/rust-by-example/trait/derive.html)
* [Boxed](https://doc.rust-lang.org/std/boxed/index.html)
* [Tokio](https://docs.rs/tokio/latest/tokio/)
* [Macro vs Function](https://stackoverflow.com/questions/29871967/)
* [Mutable Variables](https://doc.rust-lang.org/std/keyword.mut.html)
* [Pointer Types](https://doc.rust-lang.org/reference/types/pointer.html)
* [OpenAI Tokens](https://help.openai.com/en/articles/4936856-what-are-tokens-and-how-to-count-them)

## Crates Used
* hyper-tls
* tokio
* serde
* clap

## Code Execution
browse to the code folder using a terminal  
``` 
> cargo run 
```
