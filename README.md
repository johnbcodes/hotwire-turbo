# hotwire-turbo

> Generate HTML custom element payloads compatible with [Hotwire Turbo](https://github.com/hotwired/turbo) and [Turbo Power](https://github.com/marcoroth/turbo_power).

A Rust implementation of [turbo-rails TagHelper](https://github.com/hotwired/turbo-rails/blob/main/app/models/turbo/streams/tag_builder.rb)
and [turbo_power-rails StreamHelper] (https://github.com/marcoroth/turbo_power-rails/blob/main/lib/turbo_power/stream_helper.rb).

### TODOs

- [ ] README
  - [ ] Simplest example
  - [ ] Axum support example
- [ ] Much more API documentation
- [ ] Benchmarking
- [ ] Performance optimization(?, inline, String generation, etc)
- [ ] Examples
- [ ] [TurboBoost Streams](https://github.com/hopsoft/turbo_boost-streams/) support
- [ ] [TurboBoost Commands](https://github.com/hopsoft/turbo_boost-commands/) support


[![Build status](https://github.com/johnbcodes/hotwire-turbo/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/johnbcodes/sqlite-es/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hotwire-turbo)](https://crates.io/crates/hotwire-turbo)
[![Crates.io](https://img.shields.io/crates/v/hotwire-turbo-axum)](https://crates.io/crates/hotwire-turbo-axum)
[![docs](https://img.shields.io/badge/API-docs-blue.svg)](https://docs.rs/hotwire-turbo)
[![docs](https://img.shields.io/badge/API-docs-blue.svg)](https://docs.rs/hotwire-turbo-axum)

Generate HTML custom element payloads compatible with Hotwire Turbo and Turbo Power.