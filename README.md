# Go Skate API: Trade Data Analyzer

**Go Skate API** is a robust Rust application meticulously crafted to **parse and analyze trade data** from the "Go Skate" Roblox game. Developed as a powerful **command-line interface (CLI) tool**, its primary purpose is to provide players with deep insights into item markets, including price estimations, supply and demand dynamics, and trade probabilities.

## Features

* **CSV Data Parsing**: Efficiently reads and processes trade data directly from CSV files. **This application expects the input CSV to be a Discord chat output log, typically exported using tools like [DiscordChatExporter](https://github.com/Tyrrrz/DiscordChatExporter).**

* **Intelligent Item Identification**: Recognizes a wide array of in-game items based on a comprehensive, pre-defined list of keywords.

* **Dynamic Price Estimation**: Calculates **median, minimum, and maximum** estimated prices for each item based on observed trade data.

* **Supply & Demand Analysis**: Quantifies market activity by counting **"sell" (supply)** and **"buy" (demand)** posts for each item.

* **Estimated Trade Chances**: Provides an estimated chance of successfully **buying** or **selling** an item, derived from supply and demand ratios.

* **Rough Selling Frequency**: Offers insights into how frequently items are traded, categorized by "times/day," "times/week," or "times/month."

* **Configurable Verbosity**: Includes an **optional verbose logging system** (`-v` or `--verbose` flag) to print detailed actions and error information during runtime.

* **Dynamic File Input**: Allows users to specify the input CSV data file path at runtime using the **`-d` or `--data` flag**, enabling flexible analysis of different datasets.

* **Structured YAML Output**: Presents the comprehensive trade analysis in a **clean, human-readable YAML format**, complete with metadata about the parsing process and data span.

---

## Current Status

The core functionality for parsing trade data, estimating prices, analyzing supply/demand, and generating detailed YAML output is fully implemented and operational.

---

## Implemented Features

* ✅ **CSV Data Parsing**

* ✅ **Item Identification**

* ✅ **Price Estimation**

* ✅ **Supply & Demand Analysis**

* ✅ **Trade Chance Calculation**

* ✅ **Selling Frequency**

* ✅ **Configurable Verbosity**

* ✅ **Dynamic File Input**

* ✅ **YAML Output**

---

## Why CLI?

Using a **Command-Line Interface (CLI)** for the Go Skate API provides a **lightweight, high-performance, and efficient** solution for data analysis. Unlike Graphical User Interfaces (GUIs) or Text-based User Interfaces (TUIs), a CLI application like this:

* **Optimizes Performance**: Rust's performance capabilities shine in CLI tools, allowing for rapid processing of large datasets with minimal overhead.

* **Conserves Resources**: CLIs typically consume significantly less CPU and memory compared to GUI applications, making them ideal for running in resource-constrained environments or as background tasks.

* **Enhances Scriptability**: CLI tools are inherently easy to integrate into automated scripts and workflows, allowing users to automate data analysis tasks effortlessly.

* **Facilitates Remote Access**: Can be easily run on remote servers or within Docker containers without needing a graphical environment.

This approach empowers users who value efficiency and automation, providing direct access to powerful data analysis functionalities directly from their terminal. The `goskateapi` brings robust trade data insights directly to your command line, helping you stay informed and productive.

---

## Installation & Usage

### Prerequisites

* **Rust**: Ensure you have the Rust toolchain installed. You can install it via [rustup](https://rustup.rs/).

* **Docker** (Optional, for containerized deployment): Install Docker Desktop (Windows/macOS) or Docker Engine (Linux).

* **Discord Chat Export**: You will need a Discord chat output log in CSV format. A recommended tool for this is [DiscordChatExporter](https://github.com/Tyrrrz/DiscordChatExporter).

### Local Setup

1.  **Clone the Repository**:
    ```bash
    git clone [https://github.com/Zephira58/goskateapi.git](https://github.com/Zephira58/goskateapi.git)
    cd goskateapi
    ```

2.  **Place Your CSV Data**:
    Ensure your trade data CSV file, named `tradexport_1755362248.csv`, is located in the `src/data/` directory within your project.

3.  **Build the Project**:
    ```bash
    cargo build --release
    ```
    This command compiles your Rust application and creates an executable in `target/release/`.

4.  **Run the Application**:

    * **Default file path (src/data/tradexport_1755362248.csv), no verbose output:**
        ```bash
        ./target/release/goskateapi
        ```
        (On Windows: `.\target\release\goskateapi.exe`)

    * **Default file path, with verbose output:**
        ```bash
        ./target/release/goskateapi -v
        # or
        ./target/release/goskateapi --verbose
        ```

    * **Specify a custom file path, no verbose output:**
        ```bash
        ./target/release/goskateapi -d "path/to/your/custom/data.csv"
        # or
        ./target/release/goskateapi --data "path/to/your/custom/data.csv"
        ```
        (Use quotes for paths containing spaces)

    * **Specify a custom file path, with verbose output:**
        ```bash
        ./target/release/goskateapi -v -d "path/to/your/custom/data.csv"
        # or
        ./target/release/goskateapi --verbose --data "path/to/your/custom/data.csv"
        ```

### Docker Deployment

1.  **Build the Docker Image**:
    Navigate to the root directory of your project (where `Dockerfile` is located).
    ```bash
    docker build -t goskateapi-image .
    ```

2.  **Run the Docker Container**:
    ```bash
    docker run goskateapi-image
    ```
    To pass flags to the application inside Docker:
    ```bash
    docker run goskateapi-image -v
    docker run goskateapi-image -d "src/data/tradexport_1755362248.csv" # Path inside container
    ```

---

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->

<!-- prettier-ignore-start -->

<!-- markdownlint-disable -->

<table>
  <tbody>
    <tr>
<!-- Zephi -->
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Zephira58"><img src="https://avatars.githubusercontent.com/u/66909997?v=4?s=100" width="100px;" alt="Zephira58"/><br /><sub><b>Zephira58</b></sub></a><br /><a href="https://github.com/Zephira58/goskateapi/commits?author=Zephira58" title="Code">💻</a> <a href="" title="Design">🎨</a> <a href="" title="Maintenance">🚧</a></td>
  </tbody>
</table>

<!-- markdownlint-restore -->

<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->