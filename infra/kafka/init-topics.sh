#!/bin/bash
set -euo pipefail
B=/opt/kafka/bin/kafka-topics.sh
until "$B" --bootstrap-server kafka:29092 --list >/dev/null 2>&1; do sleep 2; done
create() { "$B" --bootstrap-server kafka:29092 --create --if-not-exists --topic "$1" --partitions "$2" --replication-factor 1 --config "retention.ms=$3"; }
create raw-events 3 604800000
create normalized-events 3 259200000
create detection-signals 3 604800000
create quarantine 1 1209600000
touch /tmp/topics-ready
