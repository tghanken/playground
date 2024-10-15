# fullstack-rustapp-template

An example project that outlines how to setup a full webapp using rust, vite, and nix

## Getting Started

1. Install `nix`
    - You may want to add the [garnix cache](https://garnix.io/docs/caching) to your nix configuration to speed up builds by reusing CI artifacts.
2. Install `docker`
3. Enable nix flakes
4. Install `nix-direnv` and `direnv`
5. Run `cp .env.example .env` and fill in your environment variables & secrets
6. Run `direnv allow` to allow direnv to manage your shell when editing this repo
7. Run `nix build .#dockerImage && ./result | docker load` to build the docker image
8. Run `just run` to run the full stack! `just stop` will tear down the environment

## Developing

In order to run all the CI checks locally while developing, run `just check`.
