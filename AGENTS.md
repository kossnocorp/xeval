# xeval — Agents Guide

This document gives a quick, high-signal overview of the xeval project for agents and contributors.

## Overview

xeval is an LLM evaluation framework implemented in Rust.

Its source code is organized into a monorepo. All packages and crates live under [`./pkgs/`](./pkgs/).

## Packages

Here's the high-level overview for each of the packages in the repo:

- [`./pkgs/cli/`](./pkgs/cli/) - CLI, the main entry point for interacting with the xeval framework.

- [`./pkgs/global/`](./pkgs/global/) - Global state (`~/.local/share/xeval`) for the xeval framework. Auth, cache, etc.

- [`./pkgs/project/`](./pkgs/project/) - Project (code/repo) management. Config, paths, etc.

- [`./pkgs/openai/`](./pkgs/openai/) - OpenAI API wrapper. Auth, projects (OpenAI projects) & evals management, etc.

## Development Environment

The development environment is managed using Dev Containers. The container image is based on [Mothership](https://github.com/kossnocorp/mothership).

Language, versions, and tools are managed using [**mise-en-place**](https://mise.jdx.dev/). The project configuration is defined in [`./mise.toml`](./mise.toml).

---

If anything here drifts from reality, please update [`./AGENTS.md`](./AGENTS.md) alongside your change.
