#!/bin/bash

# Function to stop a service by PID file
stop_service() {
    SERVICE_NAME=$1
    PID_FILE=$2

    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        kill $PID && echo "$SERVICE_NAME (PID: $PID) has been stopped."
        rm -f "$PID_FILE"
    else
        echo "No PID file found for $SERVICE_NAME. Is it running?"
    fi
}

# Stop quote_api
stop_service "quote_api" "quote_api.pid"

# Stop quote_sinker
stop_service "quote_sinker" "quote_sinker.pid"
