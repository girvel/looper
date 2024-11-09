`looper` is a configuration file–driven cron-style manager for reoccuring tasks. You can even accomplish your one-time tasks with it: you create an entry "do your one time tasks from your todoist" for "0 0 4 * * Sun *" and all your todos magically get done!

## TL;DR usage

Displaying the schedule:

```bash
looper show
```

```
[5] Today is 09-Nov-2024:
#1d  clean the room
#0e  clean the toilet
#13  clean the windows
#0d  clean reflective surfaces
#0c  water the plants

Upcoming:
#0a  dust cleanup  @10-Nov-2024
#01  journal  @10-Nov-2024
#23  wash sweats  @10-Nov-2024
#1e  touch grass  @10-Nov-2024
#02  change of bedsheets  @15-Nov-2024
#15  change of towels in kitchen, bathroom  @15-Nov-2024
#1a  maintenance workout  @18-Nov-2024
#04  change lenses  @18-Nov-2024
#21  buy groceries  @01-Dec-2024
#18  clean the water heater  @01-Dec-2024
...16 more
```

Completing tasks:

```bash
looper done 1d 0e 0c
```

## Configuration file example

Use `looper path` to get the path.

```toml
# next ID is 03

[daily]  # any text
period = "0 0 3 * * * *"  # cron-like syntax
00 = "make the bed"  # in format <id> = <full text>
01 = "take vitamins"

[two_days]
period = "0 0 3 * * * *"
02 = "water the plants"
```

## Installation

Clone the repository, cd into it, run

```bash
cargo install --path logic
```

## Recommended practices

- alias "looper" as "lp" for frequent use
- keep the ID for your next task in the configuration file header comment
- stow + git for configuration management

