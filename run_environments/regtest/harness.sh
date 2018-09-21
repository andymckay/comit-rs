#!/usr/bin/env bash

set -e;
export PROJECT_ROOT=$(git rev-parse --show-toplevel)

if test "$LOG_NODES"; then
    exec 4>&1
else
    exec 4>/dev/null
fi

if test "$LOG_SERVICES"; then
    if [ "$LOG_SERVICES" = "1" ]; then
        exec 3>&1;
    else
        exec 3>"$LOG_SERVICES";
    fi
else
    exec 3>/dev/null;
fi

function log {
    echo "$@" >&2;
}

END(){
    log "KILLING docker containers";
    (
        cd $PROJECT_ROOT/run_environments/regtest;
        docker-compose rm -sfv;
    );

    for pid in "$BOB_COMIT_NODE_PID" "$ALICE_COMIT_NODE_PID"; do
        if test "$pid" && ps "$pid" >/dev/null; then
            echo "KILLING $pid";
            kill "$pid" 2>/dev/null;
        fi
    done
}

trap 'END' EXIT;


function start_target() {
    name=$1;
    log_prefixed=$name-$2
    log "Starting $log_prefixed";
    # Logs prefixes the service name in front of its logs
    "${PROJECT_ROOT}/target/debug/$name" 2>&1 | sed  "s/^/$log_prefixed: / " >&3 &
    # returns the PID of the process
    jobs -p
}

function activate_segwit() {
    log "Generating enough blocks to activate segwit";
    curl -s --user $BITCOIN_RPC_USERNAME:$BITCOIN_RPC_PASSWORD --data-binary \
          "{\"jsonrpc\": \"1.0\",\"id\":\"curltest\",\"method\":\"generate\", \"params\": [ 432 ]}" -H 'content-type: text/plain;' $BITCOIN_RPC_URL  > /dev/null
}


function setup() {
    log "Starting up ...";

    cargo build

    #### Env variable to run all services
    set -a;
    source ${PROJECT_ROOT}/run_environments/common.env
    source ${PROJECT_ROOT}/run_environments/regtest/network.env
    source ${PROJECT_ROOT}/run_environments/regtest/regtest.env
    set +a;

    #### Start all services
    (
        cd $PROJECT_ROOT/run_environments/regtest;
        docker-compose up -d bitcoin ethereum >&4 2>&4;
    );

    sleep 5;

    activate_segwit;

    BOB_ROCKET_PORT=8010
    ALICE_ROCKET_PORT=8000

    BOB_COMIT_PORT=8011
    ALICE_COMIT_PORT=8001

    BOB_COMIT_LISTEN="0.0.0.0:$BOB_COMIT_PORT";
    ALICE_COMIT_LISTEN="0.0.0.0:$ALICE_COMIT_PORT";

    BOB_COMIT_NODE_PID=$(
        export BITCOIN_RPC_URL=http://localhost:18443 \
               ETHEREUM_NODE_ENDPOINT=http://localhost:8545 \
               ROCKET_ADDRESS=0.0.0.0 \
               REMOTE_COMIT_NODE_SOCKET_ADDR=127.0.0.1:$ALICE_COMIT_PORT \
               ROCKET_PORT=$BOB_ROCKET_PORT \
               RUST_LOG=comit_node=debug,bitcoin_htlc=debug \
               RUST_BACKTRACE=1 \
               COMIT_LISTEN=$BOB_COMIT_LISTEN
               BITCOIN_SATOSHI_PER_KB=50;

        start_target "comit_node" "Bob  ";
    );



    ALICE_COMIT_NODE_PID=$(
        export  ROCKET_ADDRESS=0.0.0.0 \
                RUST_LOG=comit_node=debug,bitcoin_htlc=debug \
                REMOTE_COMIT_NODE_SOCKET_ADDR=127.0.0.1:$BOB_COMIT_PORT \
                ROCKET_PORT=$ALICE_ROCKET_PORT \
                COMIT_LISTEN=$ALICE_COMIT_LISTEN
                RATE=0.1;

        start_target "comit_node" "Alice";
    );
}

test "$@" || { log "ERROR: The harness requires a file to run!"; exit 1; }

setup;

sleep 2;

"$@"