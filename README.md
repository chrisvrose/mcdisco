# mcdisco

A smol discord bot to help start a server, without needing to ssh in.

## Commands

- [X] `start` - Start the server
- [X] `status` - Get the server status
- [X] `help` - Get commands
- [X] `stop` - Stop the server

## Env vars

```env
DISCORD_TOKEN=xyz
ENV_PATH=/home/atreyab/server
RUNSCRIPT=./start.sh

GLOBAL_RUST_LOG=warn
RUST_LOG=trace
```