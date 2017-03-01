# Usage

_Some musings about glop in practice. None of this is implemented yet and it
might change drastically._

## Starting the agent server

    glop server

## Adding agents

Add agents defined in .glop files.

    glop agent add hello helloworld.glop

Agents and their state persist across restarts of the glop server.

## Listing agents

Agents on the server may be listed with `glop agent list`.

    glop agent list

    hello

## Removing agents

Agents may be removed.

    glop agent remove 

# Interact with agents via messages

The general form of sending messages is:

    glop agent send {target} {topic} [ k=v [ k=v ... ] ]

Where:
- {target} is the agent name
- {topic} is the kind of message
- k=v is zero or more structured key-value pairs constituing a message object.

Keys may use a flattened dot notation to indicate message structure.

## Flattened dot notation

### Nested objects

A key like foo.bar=2 results in a message object structured as `{"foo": {"bar": 2}}`
when rendered as JSON.

### Arrays

A key like foo.0=2 results in a message structured as `{"foo": [2]}`. Sparse
arrays result in null values for the unspecified elements.

## Examples

### Changing configuration

Agents are operated by sending them messages. This message tells the go-devel
agent to set the Go release version to 1.9.0.

    glop agent send go-devel configure release=1.9.0

Response might be:

    status=ok

### Reading configuration

This message asks go-devel for its configuration.

    glop agent send go-devel configuration

The response might be:

    release=1.9.0
    downloader.url.base=https://storage.googleapis.com/golang/
    downloader.url.pattern=go${version}.${platform}-${arch}.tar.gz

# Coordinating agents with conversations

It's tedious to have to tell agents every single thing they need to do.
Instead, get them talking to each other!

    glop agent introduce foo bar

Agents declare the roles in which they act, and the counterpart agent roles
they react with. The agents then coordinate amongst themselves what they do
from there.



## Coordinate across machines

Agents running on different machines can be introduced over SSH.

    glop introduce some-reporting ubuntu@my-cuda-beast:some-theano-based-app

# TODO

- Common message pattern language
- Cleaner distinction between agent "instance" and "template"
- Agent lifecycle issues
