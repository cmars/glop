#!/bin/bash

set -ex

HERE=$(cd $(dirname $0); pwd)

GLOPSRC=$1
if [ -z "$GLOPSRC" ]; then
	echo "usage: $0 <glopfile> [<lxd container>]"
	exit 1
fi
if [ ! -e "$GLOPSRC" ]; then
	echo "$GLOPFILE not found"
	exit 1
fi
GLOPNAME=$(basename $GLOPSRC | sed 's/\.glop$//')

CONTAINER=$2
if [ -z "$CONTAINER" ]; then
	CONTAINER=$(lxc launch ubuntu:16.04 | grep Starting | awk '{print $2}')
	echo "Created ${CONTAINER}"
fi

cd ${HERE}/../..
cargo build
lxc file push ./target/debug/glop ${CONTAINER}/usr/bin/glop
lxc exec ${CONTAINER} -- mkdir -p /etc/glop
lxc file push ${GLOPSRC} ${CONTAINER}/etc/glop/${GLOPNAME}.glop
nohup lxc exec --env RUST_LOG=debug ${CONTAINER} -- /bin/bash -c '/usr/bin/glop server' &
lxc exec ${CONTAINER} -- /bin/bash -c 'for i in {1..10}; do [ -e ~/.glop.agent ] && break || sleep $i; done'
lxc exec ${CONTAINER} -- /usr/bin/glop agent add ${GLOPNAME} /etc/glop/${GLOPNAME}.glop
