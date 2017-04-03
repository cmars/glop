#!/bin/bash

set -ex

HERE=$(cd $(dirname $0); pwd)

CONTAINER=$1
if [ -z "$CONTAINER" ]; then
	CONTAINER=$(lxc launch ubuntu:16.04 | awk '/Starting/ {print $2}')
	echo "Created ${CONTAINER}"
fi
for i in {1..10}; do
	CONTAINER_IP4=$(lxc info ${CONTAINER} | awk '/eth0:\tinet\t/ {print $3}')
	if [ -n "${CONTAINER_IP4}" ]; then
		break
	fi
	sleep $i
done
if [ -z "${CONTAINER_IP4}" ]; then
	echo "failed to get container IP address"
	exit 1
fi

cd ${HERE}/../..
cargo build
lxc file push ./target/debug/glop ${CONTAINER}/usr/bin/glop
lxc exec ${CONTAINER} -- apt update
lxc exec ${CONTAINER} -- apt install libsodium18 -y
lxc exec --env RUST_LOG=info ${CONTAINER} -- /bin/bash -c '/usr/bin/glop server init'
TOKEN=$(lxc exec --env RUST_LOG=info ${CONTAINER} -- /bin/bash -c "/usr/bin/glop server token add ${CONTAINER}")
./target/debug/glop remote add ${CONTAINER} ${CONTAINER_IP4}:6709 ${TOKEN}
nohup lxc exec --env RUST_LOG=info ${CONTAINER} -- /bin/bash -c '/usr/bin/glop server run' &
