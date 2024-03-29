# Serverless Rust Starter

A simple starter project with Rust and Serverless Framework.
This starter aims to use the latest versions of Serverless Framework, packages, and crates.

✔️ Serverless Framework 3

✔️ Typed requests and responses with aws_lambda_events `0.10.0`

✔️ Lambda Runtime `0.8.1`

✔️ Serverless Rust plugin `0.3.8`

## Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Yarn](https://classic.yarnpkg.com/en/)
- [Cargo Lambda](https://www.cargo-lambda.info/)
- [Zig](https://ziglang.org/)

### Setup

1. Generate a project

```bash
npx serverless install --url https://github.com/ArsenyYankovsky/serverless-rust-starter.git --name my-new-app
```

2. Install dependencies

```bash
yarn && cargo fetch
```

### Deploy

To deploy the project just run

```bash
npx sls deploy
```

Read [here](https://www.serverless.com/framework/docs/) more on Serverless Framework commands
