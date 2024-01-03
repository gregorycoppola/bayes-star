# Bayes-Star Usage Instructions

This document provides instructions on how to set up and run the Bayes-Star project.

# Warning if You Already Have Redis
This software is currently set to clear the Redis database on localhost when it starts.
I will work on a better UI.
In the meantime:
* **Do not run this software if you already have a Redis database on 'localhost' because it will get cleared.**


# Reminder to "Use a Chat Bot"
**At any time if you get stuck, just ask your favorite "chat bot"**. 

Keeping docs up to date has always been impossible, and I'm only testing on my own context, but you can just get your favorite chat bot to explain things to you if you know how to ask. I used [ChatGPT](https://chat.openai.com/) in the creation of this project.

# System Overview
The dependencies are:
* **Rust**
    * This runs the **Rust** code that does inference and training [Rust](https://www.rust-lang.org/).
* **REDIS**
    * This is an in-memory data store (similar to **MEMCACHE**) where the data and theories are stored as **strings**. 
    * You can use any store and any serialization method.
* **python3**
    * This is **optional** because I wrote my "eval" in python3.
    * But, it is trivial and you can start over in any framework. I'm not that current on the latest data analysis tools.

# Redis for the Data Store
See [REDIS.md](REDIS.md).

# Rust for the Model

The main program is written in [Rust](https://www.rust-lang.org/).

There is right now some analysis code written in [python3](https://www.python.org/). But, if you want to use a different language for analysis, you don't have to use python.

### Installing Rust on Your System

See [Rust](https://www.rust-lang.org/) or ask your favorite chat bot.

## Install the BAYES STAR Package
You have to `cd` into the `rust` directory and do a `cargo build` to install the dependencies:


```
cd rust
cargo build
```

You can find the dependencies in the file [Cargo.toml](rust/Cargo.toml).

# Python for the Analysis
Note: The analysis currently uses python. But, you can rewrite your own eval. I only putted the simplest eval to make the graph of the training loss.

## Checking Python Installation

Before you begin, you need to ensure that Python 3 is installed on your system. To check if Python is installed and determine its version, open your terminal or command prompt and type:

To see if you have python (before version 3), type:
```bash
python --version
```

To see if you have python3, type
```
python3 --version
```

## Install Python Dependencies
Right now, the only dependency is `matplotlib`. Run the following command in your terminal or command prompt to install `matplotlib`:

```bash
pip install matplotlib
```

or if your system differentiates between Python 2 and Python 3:

```bash
pip3 install matplotlib
```

# Running

In order to run training and the eval I used to make the graph in [training loss graph](docs/images/training_loss.png), run:

```
./trainandanalyze.sh
```

In order to just run training, run:

```
./train.sh
```