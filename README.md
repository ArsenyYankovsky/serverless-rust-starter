# Serverless Rust Starter

A simple starter project with Rust and Serverless Framework

✔️ Serverless Framework 2

✔️ Typed requests and responses with aws_lambda_events `0.5.0`

✔️ Lambda Runtime `0.4.1`

✔️ Serverless Rust plugin `0.3.8`

## Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Yarn](https://classic.yarnpkg.com/en/)

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
