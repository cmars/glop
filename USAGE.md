% USAGE

# Usage

_Some musings about glop in practice. None of this is implemented yet and it
might change drastically until glop is released as '1.0'._

## Starting the agent server

    glop server run

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

    glop agent introduce foo:fooer bar:barrister

Agents declare the roles in which they act, and the counterpart agent roles
they react with. The agents then coordinate amongst themselves what they do
from there. The colon syntax is of the form `agent-name:role`, used to designed
each agent and the role it should act in, in the conversation.

# Agent servers

An agent server is initialized explicitly with

    $ glop server init

or implicitly the first time it is started with

    $ glop server run

## Server access tokens

The glop server generates an initial access token, a secret key that encrypts
and authenticates client-server communication.

Tokens may be created for remote access to the agent server API. For example,

    $ glop server token add bob
    BGIPBeVLO5lcuWZDn2FrvC9VO6luPPrYK4CSm0iUpcY=

Active tokens may be listed.

    $ glop server token list
    SHi19sxsov2/6SJ7cPd3IAMieEJtP5zxcogeTXkHl2w= admin
    BGIPBeVLO5lcuWZDn2FrvC9VO6luPPrYK4CSm0iUpcY= bob

Tokens may be revoked by name.

    $ glop server token revoke bob

Tokens may be revoked by token contents.

    $ glop server token revoke BGIPBeVLO5lcuWZDn2FrvC9VO6luPPrYK4CSm0iUpcY=

## Client-Server access

Clients may add remote agents.

    $ glop remote add website 10.212.34.35 JhnFgFbqO+OA95y+DpPDEjeezvJguGLoosg93FG6yB8=

If the client has remote SSH access to the machine, this is easy to set up.

    $ glop remote add website 10.212.34.35:6709 $(ssh ubuntu@10.212.34.35 "sudo glop server token add cmars")

The glop agent server default, port 6709, is assumed if not specified.

Clients may list remotes.

    $ glop remote list
    website 10.212.34.35:6709

Clients may remove remotes.

    $ glop remote remove website

Remotes may be used with `glop agent` commands.

    $ glop agent webapp@website webapp.glop
    $ glop agent add db@db1 db.glop
    $ glop agent add db@db2 db.glop
    $ glop agent introduce webapp:db-client@website db:db-server@db1
    $ glop agent introduce db:db-primary@db1 db:db-replica@db2
    $ glop agent send webapp@website configure fqdn=foo.bar.com
    $ glop agent send webapp@website golive
    $ glop agent send db@db2 backup

## Server-Server access

As the above example illustrates, a client can introduce agents across remotes.

    glop agent introduce foo:fooer@remote1 bar:barrister@remote2

The client will request peer tokens from _remote1_ and _remote2_ as an admin,
and then send an introduction message to each server with these tokens. Agent
replies will then go to the introduced counterpart on the other agent server.

# Keeping track of contacts

_Still thinking about this section... Feels like contacts could be stored as
regular vars maybe?_

Once introduced, agents may need to keep track of their counterpart contacts.
From within a `match` on a topic from a role,

    match (message intro from foolike as barrister) #!/bin/bash
    set -e
    glop contact add foo-fighters intro
    !#

Where `foo` was the topic of a matched message, and `foo-fighters` is a contact list.

Contact lists can be the object of a `msg send`,

    glop msg send foo-fighters attack=true method=squirrels count=20

The members of contact list may be shown,

    glop contact list foo-fighters

A contact may also be removed by resolving the role of the current topic match,

    match (message bye from foolike as barrister) #!/bin/bash
    set -e
    glop contact remove foo-fighters bye
    !#

# TODO

- Common message pattern language. Well-defined message types?
- Cleaner distinction between agent "instance" and "template"
- Agent lifecycle issues. How do they die?
