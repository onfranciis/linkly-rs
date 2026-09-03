# Contributing to Linkly RS

Thanks for your interest in contributing! Linkly RS is a URL shortener service built in Rust with [Rocket](https://rocket.rs/), Redis, and PostgreSQL. See [README.md](README.md) for the API surface and [LEARN.md](LEARN.md) for a quick walkthrough of how a shorten/redirect request flows through the service.

## Prerequisites

- The [Rust toolchain](https://www.rust-lang.org/tools/install) (stable). The project targets edition 2021; no specific version is pinned.
- A reachable PostgreSQL instance.
- A reachable Redis instance.

## Local setup

1. Clone the repo and create a `.env` file in the project root with:

   ```bash
   ROCKET_PORT=1234
   REDIS_URL=redis://<url>
   DATABASE_URL=postgres://<username>:<password>@<url>/linkly_rs
   ```

   `DATABASE_URL` must point to a live, reachable database — `sqlx`'s query macros validate against it. Avoid spaces or quotes in the `.env` file, which can cause issues in Docker setups.

2. Build and run:

   ```bash
   cargo build
   cargo run
   ```

   The server starts on `ROCKET_PORT`, connects to Postgres (creating the `url` table if it doesn't exist) and Redis on startup.

## Project layout

- `src/main.rs` — app entry point, Rocket config/figment setup, route mounting.
- `src/controllers/` — request handlers (add/get/list URLs).
- `src/catchers/` — error catchers (404/500/503) and the health-check route.
- `src/util/` — shared helpers: DB/Redis setup, response types, short-code generation.

## Coding style

Please run the following before submitting a change:

```bash
cargo fmt
cargo clippy
```

There's no CI enforcing this yet, so it's on the honor system for now — keeping formatting and lint output clean makes review faster.

## Tests

There isn't a test suite in the repo yet. If you're adding non-trivial logic, consider adding `#[cfg(test)]` unit tests alongside it. Growing test coverage is welcome as its own contribution too.

## Commit messages

Commits loosely follow [Conventional Commits](https://www.conventionalcommits.org/), e.g.:

```
feat: add redis support
fix: abstract current_time and fix late match
```

Use a `feat:`/`fix:` (or similar) prefix followed by a short, lowercase, imperative description.

## Submitting changes

1. Branch off `main`.
2. Make your change, following the style and commit conventions above.
3. Open a pull request against `main` describing what changed and why.

## Reporting issues

Open a [GitHub issue](https://github.com/onfranciis/linkly-rs/issues) with:

- Steps to reproduce
- Expected vs. actual behavior
- Relevant environment details (OS, Rust version, etc.)

## License

Linkly RS is licensed under the GNU General Public License v3 — see [COPYING.txt](COPYING.txt). By contributing, you agree that your contributions will be licensed under the same terms.

## Questions

For anything else, reach out via [hello@onfranciis.dev](mailto:hello@onfranciis.dev).
