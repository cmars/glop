# Usage

_Some musings about glop in practice. None of this is implemented yet and it
might change drastically._

## Loading agents

Load agents by specifying glop files or a directory tree containing glop files,
or a URL that refers to a glop file or directory tree.

    glop load {filespec...}

If glopd is not already running, it is started. glopd may also be started with
`glop daemon`. Loaded agents and their state persist across restarts of
the daemon.

## Listing agents

Agents may be listed with `glop list`.

## Stopping and starting agents

Specific agents may be stopped and started by name.

    glop stop rust-devel gnupg-ykneo
    glop start gnupg-ykneo

## Unloading agents

Agents may be unloaded. This stops the agent and forgets its state.

    glop unload go-devel

# Operating agents with messages

The general form of sending messages is:

    glop send {target} {topic} [ k=v [ k=v ... ] ]

Where:
- {target} is the local identity of the agent
- {topic} is the kind of message
- k=v is zero or more structured key-value pairs

Keys may use a flattened dot notation to indicate message structure.

## Flattened dot notation

### Nested objects

A key like foo.bar=2 results in a message structured as `{"foo": {"bar": 2}}`
when rendered as JSON.

### Arrays

A key like foo.0=2 results in a message structured as `{"foo": [2]}`. Sparse
arrays result in null values for the unspecified elements.

## Examples

### Changing configuration

Agents are operated by sending them messages. This message tells the go-devel
agent to set the Go release version to 1.9.0.

    glop send go-devel configure release=1.9.0

Response might be:

    status=ok

### Reading configuration

This message asks go-devel for its configuration.

    glop send go-devel configuration

The response might be:

    release=1.9.0
    downloader.url.base=https://storage.googleapis.com/golang/
    downloader.url.pattern=go${version}.${platform}-${arch}.tar.gz

# Coordinating agents with conversations

It's tedious to have to tell agents every single thing they need to do.
Instead, get them talking to each other!

    glop introduce my-dev-env rust-devel,go-devel,gnupg-ykneo,vim,tmux,bashrc

Once introduced, the agent 'my-dev-env' messages with the given agents on my
behalf, customizing them to my liking -- even introducing them to each other to
have further conversations. The clean separation of site-specific configuration
from general operations maximizes reuse of the general capabilities provided by
rust-devel.

## Coordinate across machines

Agents running on different machines can be introduced over SSH.

    glop introduce some-reporting ubuntu@my-cuda-beast:some-theano-based-app

# TODO

- Common message pattern language
- Cleaner distinction between agent "instance" and "template"
- Agent lifecycle issues
