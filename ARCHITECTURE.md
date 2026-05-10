# Architecture of site2md

Each site live in a separate crate.

## Limitation

Currently, both `scraper` and `htmd` is used, thus each html is parsed and iterated twice. I'll soon work out and switch to a unified DOM implementation.
