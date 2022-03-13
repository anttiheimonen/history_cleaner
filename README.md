# Bash history cleaner

Bash history cleaner is made to clean bash history of duplicate line and lines, which start with an unwanted word. At the moment these unwanted words are "cd" and "code". These are hard-coded in and cannot be changed. This might change in the future.

## Usage

There is no released executable, but you can clone the repository and build the application with Rust tools.

Run executable:

```bash
    history_cleaner ~/.bash_history
```

The old bash history file is not overwritten automatically. The cleaned bash history file will be in `~/.bash_history_cleaned`. To take the new file in use,
replace the `~/.bash_history` with the newly created `~/.bash_history_cleaned`.
