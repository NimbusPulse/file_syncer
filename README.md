# File Syncer for [NimbusPulse](https://nimbuspulse.com/)

This is a simple-to-use file syncer across multiple servers hosted on [NimbusPulse](https://nimbuspulse.com/).

## Features
 - Sync files between multiple servers and regions
 - Lightweight on CPU and network; diffs files based on [BLAKE2](https://en.wikipedia.org/wiki/BLAKE_(hash_function)) hashes and resolves conflicts automatically based on last modified time.
 - Files are synced recursively from the directory specified in `SYNC_PATH` (the path on the server, e.g. `/Missions`).
 - Works with any number of servers; no limit.

## Usage
 - Download the latest release from [GitHub](https://github.com/NimbusPulse/file_syncer/releases).
 - Copy the `.env.example` file to `.env` and edit it. `NIMBUSPULSE_SYNC_INSTANCE_IDS` is a comma-separated list of instance IDs to sync files between.
 - Start the file syncer.

## TODO
 - Conflict resolution is non-destructive; more strategies can be added.
