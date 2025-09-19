#!/bin/bash
set -e


# Global variables
BROKER_PORT=""
IP="127.0.0.1"
LOG_PATH=""

# Parse command line arguments using getopts
while getopts "p:i:l:h" opt; do
    case $opt in
        p)
            BROKER_PORT="$OPTARG"
            ;;
        i)
            IP="$OPTARG"
            ;;
        l)
            LOG_PATH="$OPTARG"
            ;;
        h)
            usage
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            usage
            ;;
        :)
            echo "Option -$OPTARG requires an argument." >&2
            usage
            ;;
    esac
done

# Check if required parameters are provided
if [ -z "$BROKER_PORT" ]; then
    echo "Error: BROKER_PORT parameter is required"
    echo "Use -p to specify the broker port"
    usage
fi

if [ -z "$LOG_PATH" ]; then
    echo "Error: LOG_PATH parameter is required"
    echo "Use -l to specify the log path"
    usage
fi

echo "Starting bitvmx-job-dispatcher..."
echo "BROKER_PORT: $BROKER_PORT"
echo "IP: $IP"
echo "LOG_PATH: $LOG_PATH"

# create logs directory if it doesn't exist
mkdir -p "$LOG_PATH"

# remove the log file
rm -rf "$LOG_PATH/bitvmx-dispatcher.log"

# we go to the root of the project to avoid relative path issues
CURRENT_PATH=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$CURRENT_PATH/../";

# go to the bitvmx client folder
BITVMX_PATH="$CURRENT_PATH/../../../rust-bitvmx-workspace/rust-bitvmx-client"
cd "$BITVMX_PATH"

# go to the job dispatcher folder
RUST_BACKTRACE=1 "../rust-bitvmx-job-dispatcher/target/release/bitvmx-emulator-dispatcher" --ip $IP --port $BROKER_PORT 2>&1 | while IFS= read -r line; do echo "$line"; echo "$line" | sed -r 's/\x1B\[([0-9]{1,2}(;[0-9]{1,2})*)?[mGKHF]//g' >> "$LOG_PATH/bitvmx-dispatcher.log"; done


# =============================================================================
# FUNCTIONS
# =============================================================================

# Function to display usage
usage() {
    echo "Usage: $0 -i IP -p BROKER_PORT"
    echo "  -i IP           The IP address to use (default: 127.0.0.1) [OPTIONAL]"
    echo "  -p BROKER_PORT  The broker port to use (e.g., 22222) [REQUIRED]"
    echo ""
    echo "Examples:"
    echo "  $0 -p 22222                    # Uses default IP 127.0.0.1"
    echo "  $0 -i 192.168.1.100 -p 22222  # Uses custom IP"
    exit 1
}
