# AI shell helper

Calls ChatGPT to create a shell command to accomplish a task described
in natural language. Accepts the task description as the first argument
and reads bash history from standard input to supply more context to
ChatGPT.

Usage:

```bash
cargo install --path .

cat >> ~/.bashrc << EOF
export OPENAI_API_KEY="YOUR_API_KEY"

function l() { # Replace with abbreviation of choice
  COMMAND=$(H=$(history 20) echo "$H" | llmsh "$*")
  read -p "${COMMAND} ? " CONFIRM
  if [ -z "${CONFIRM}" ]; then
    eval ${COMMAND}
  fi
}
EOF
```

Then it can be run as follows, confirming the command by pressing enter:

```bash
$ l get version of ubuntu
lsb_release -a ?
No LSB modules are available.
Distributor ID:	Ubuntu
Description:	Ubuntu 22.04.2 LTS
Release:	22.04
Codename:	jammy
```
