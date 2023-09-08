# flexible-time

[![crates.io](https://img.shields.io/crates/v/flexible-time.svg)](https://crates.io/crates/flexible-time)
[![docs.rs](https://docs.rs/flexible-time/badge.svg)](https://docs.rs/flexible-time)
[![CI](https://github.com/ctron/flexible-time/workflows/CI/badge.svg)](https://github.com/ctron/flexible-time/actions?query=workflow%3A%22CI%22)


Time formats can be tricky. And parsing them even more so. However, in some cases you just don't want to
bother the user with that. Especially when it comes command line applications.

## The Problem

> I need a command line argument `--since`, which allows the user to provide a year, or specific minute of a day.
> Either one is fine.

This crate can do it.

## Filling the gaps

The timestamp will be parsed, but have gaps (like month, day). As can be different expectations on how to fill those
gaps. The primary motivation for this crate was a "since" timestamp, so the strategy for that would be to fill
missing components with the minimal value. That's implemented using the `StartTimestamp`.

Currently, there is no timezone handling, it defaults to UTC.

## Formats

For timestamps, the following formats can be used:

| **Input**             | **Start timestamp**       |
|-----------------------|---------------------------|
| `2023`                | `2023-01-01 00:00:00 UTC` |
| `2023-02`             | `2023-02-01 00:00:00 UTC` |
| `2023-02-03`          | `2023-02-03 00:00:00 UTC` |
| `2023-02-03 16`       | `2023-02-03 16:00:00 UTC` |
| `2023-02-03 16:05`    | `2023-02-03 16:05:00 UTC` |
| `2023-02-03 16:05:06` | `2023-02-03 16:05:06 UTC` |
