# Slack Markov
A markov chain bot for <a href="https://slack.com/">Slack</a> written in Rust

## Overview

This bot utilizes <a href="https://docs.rs/tiny_http/latest/tiny_http/">Tiny HTTP</a> to communicate to Slack via a <a href="https://api.slack.com/messaging/webhooks">Webhook</a>.  All messages are parsed and concatenated to the internal content.  At random intervals or if the bot is mentioned, it will respond with a <a href="https://docs.rs/markov/latest/markov/">Markov Chain</a> generated from the stored content.

Inspiration is care of <a href="https://github.com/grantmd/slack-markov">Slack Markov in Go</a>.

## Configuration

Configuration is hard coded at the top of `main.rs`:
```
IP - local network IP to serve from
PORT - port that needs to be open to WAN
BOT_NAME - defaults to athena
RESPONSE_CHANCE - defaults to 5%
```
