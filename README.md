
A Rust-based cryptocurrency trading bot that fetches live price data from CoinGecko, calculates simple moving averages (SMAs) using a circular buffer, and simulates trades (buy/sell) based on SMA crossover signals. The bot also includes risk management via stop-loss and can send real-time Telegram alerts.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation and Setup](#installation-and-setup)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Live Trading Mode](#live-trading-mode)
  - [Backtesting Mode](#backtesting-mode)
- [Design Decisions](#design-decisions)
- [Project Structure](#project-structure)
- [Troubleshooting and Debugging](#troubleshooting-and-debugging)
- [Submission Guidelines](#submission-guidelines)
- [License](#license)

---

## Overview

This project is designed for the *Software Engineer Assessment for RavenCast Labs*. The bot:
- Connects to CoinGecko API to fetch live cryptocurrency prices.
- Uses a circular buffer (VecDeque) to maintain recent price data for efficient SMA calculations.
- Generates trade signals:
  - *Buy Signal:* When the short-term SMA (default: 5 prices) rises above the long-term SMA (default: 20 prices).
  - *Sell Signal:* When the short-term SMA falls below the long-term SMA.
- Simulates trades and logs buy/sell actions along with timestamps, prices, and profit/loss details.
- Implements risk management with a configurable stop-loss percentage.
- Sends Telegram alerts using the Teloxide library for real-time notifications.

---

## Features

- *API Integration:* Polls the CoinGecko API every minute for real-time Bitcoin price data.
- *Trade Simulation:* Uses two SMA calculations (short-term and long-term) to generate buy/sell signals.
- *Efficient Data Handling:* Implements a circular buffer to store only the most recent prices, ensuring constant-time updates.
- *Risk Management:* Automatically triggers a sell if the loss exceeds a set threshold.
- *Telegram Alerts:* Notifies you about trading signals and stop-loss events.
- *Backtesting Mode:* Allows simulation of trading using historical data provided in a CSV file.

---

## Prerequisites

Before running the bot, make sure you have the following installed:

- *Rust and Cargo:* Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
- *Git:* To clone and push your repository (download from [git-scm.com](https://git-scm.com/downloads)).

---

## Installation and Setup

1. *Clone the Repository:*

   ```bash
   git clone https://github.com/yourusername/crypto-trader.git
   cd crypto-trader
Build the Project:

The project uses Cargo for dependency management. Run the following command to compile the project:

bash
Copy
Edit
cargo build --release
This command downloads all dependencies (as specified in Cargo.toml) and builds the project.

Configuration
The application is configurable via command-line arguments using the clap library. Below are the main options:

--backtest: Run in backtesting mode using historical data.
--historical-data <PATH>: Path to the CSV file with historical prices (default: historical_prices.csv).
--sma-short <NUMBER>: Number of recent prices for short-term SMA calculation (default: 5).
--sma-long <NUMBER>: Number of recent prices for long-term SMA calculation (default: 20).
--stop-loss <PERCENT>: Percentage for stop-loss (default: 5.0).
--telegram-bot-token: Your Telegram bot token (can also be set via environment variable TELEGRAM_BOT_TOKEN).
--telegram-chat-id: Your Telegram chat ID (can also be set via environment variable TELEGRAM_CHAT_ID).
Telegram Setup
To enable alerts:

Create a Telegram Bot: Use @BotFather on Telegram.

Obtain the Bot Token: Follow BotFather’s instructions.

Get Your Chat ID: Use @getidsbot or inspect messages sent by your bot.

Set Environment Variables:

bash
Copy
Edit
export TELEGRAM_BOT_TOKEN="your_bot_token"
export TELEGRAM_CHAT_ID="your_chat_id"
Usage
Live Trading Mode
To run the bot in live trading mode (fetching real-time data from CoinGecko):

bash
Copy
Edit
cargo run --release -- --sma-short 7 --sma-long 21 --stop-loss 3.5
The bot polls the CoinGecko API every 60 seconds.
It computes SMAs and executes trades based on the configured thresholds.
Telegram alerts are sent if you have configured your bot token and chat ID.
Backtesting Mode
To simulate trading with historical data:

Prepare a CSV file (default name: historical_prices.csv) with the following format:

csv
Copy
Edit
timestamp,price
2023-01-01T00:00:00Z,42000.0
2023-01-01T01:00:00Z,42250.0
2023-01-01T02:00:00Z,41900.0
Run the backtest:

bash
Copy
Edit
cargo run --release -- --backtest --historical-data historical_prices.csv --sma-short 10 --sma-long 30
The bot reads historical data from the CSV, simulates trades, and logs the final profit.
Design Decisions
Language Choice (Rust): Chosen for its performance, memory safety, and modern concurrency features.
Circular Buffer (VecDeque): Provides efficient constant-time insertions and removals to maintain a fixed-size window of recent prices.
SMA Calculation: Simple, yet effective method to generate trading signals.
Telegram Integration: Provides real-time notifications for trading signals and alerts, ensuring you are informed on-the-go.
Error Handling and Logging: Uses the log and env_logger libraries for detailed runtime logging and error tracking.
Asynchronous Operations: Powered by tokio to manage I/O efficiently (polling API, handling delays, etc.).
Configuration with Clap: Ensures a flexible, user-friendly command-line interface with built-in help documentation.
Project Structure
bash
Copy
Edit
crypto-trader/
├── Cargo.toml         # Project dependencies and metadata
├── README.md          # This file
└── src/
    └── main.rs        # Main Rust source code
Cargo.toml: Lists all dependencies (clap, tokio, reqwest, serde, csv, teloxide, log, env_logger).
src/main.rs: Contains the complete trading bot implementation.
historical_prices.csv: (Optional) Used for backtesting mode.
Troubleshooting and Debugging
Build Issues: Run cargo check to quickly identify compilation errors.
Formatting: Run cargo fmt to format the code according to Rust standards.
Linting: Run cargo clippy for hints and potential code improvements.
Debug Logs: Set the environment variable RUST_LOG=debug to see detailed logging output.
Example:

bash
Copy
Edit
RUST_LOG=debug cargo run
Submission Guidelines
Ensure Your Repository is Complete:

All source code files and configuration files (e.g., Cargo.toml) are included.
A complete and detailed README.md is present.
Push Your Code to GitHub:

Initialize Git (if not already):
bash
Copy
Edit
git init
Add All Files:
bash
Copy
Edit
git add .
Commit Your Changes:
bash
Copy
Edit
git commit -m "Initial commit: Crypto trading bot implementation"
Add Remote Repository:
bash
Copy
Edit
git remote add origin 
Push to GitHub: https://github.com/harshu927/Crypto-Trader
bash
Copy
Edit
git push -u origin main
Submit the GitHub Repository Link as required by your assessment instructions.
[00:31, 23/3/2025] Harsh Choubey: ---

This README provides all the details needed to run, test, and submit your Rust-based crypto trading bot. It covers every aspect from installation, configuration, execution, design reasoning, and submission—ideal for a beginner to follow along. Enjoy building and best of luck with your assessment!
