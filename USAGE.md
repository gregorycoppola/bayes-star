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
**NOTE**: Training will *wipe out* your **REDIS** store on *localhost*, so **STOP** right now if you have **REDIS** on *localhost*.

See [REDIS.md](REDIS.md).

# Rust for the Model

The main program is written in [Rust](https://www.rust-lang.org/).

There is right now some analysis code written in [python3](https://www.python.org/). But, if you want to use a different language for analysis, you don't have to use python.

### Installing Rust on Your System

See [Rust](https://www.rust-lang.org/) or ask your favorite chat bot.

## Run
### Training

**NOTE**: Training will *wipe out* your **REDIS** store on *localhost*, so **STOP** right now if you have **REDIS** on *localhost*.

From the `rust` directory:

```
./train.sh dating_simple
```

### Plotting Convergence
Plot convergence for an observation of a variable using the string-valued test scenario key defined in `rust/src/bin/plot.rs`.

From the `rust` directory:

```
./plot.sh ../../bayes-data/feb11 dating_simple they_date 10
```