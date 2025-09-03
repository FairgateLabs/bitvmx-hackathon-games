#!/bin/bash

# Script to run multiple frontend instances on different ports
# This allows testing the backend configuration system

echo "Starting multiple frontend instances..."

# Function to run frontend on specific port
run_frontend() {
    local port=$1
    local backend_name=$2
    
    echo "Starting frontend on port $port (connects to $backend_name)..."
    
    # Create a new terminal window/tab for each instance
    if command -v osascript &> /dev/null; then
        # macOS - open new terminal tab
        osascript -e "tell application \"Terminal\" to do script \"cd $(pwd) && PORT=$port yarn dev\""
    elif command -v gnome-terminal &> /dev/null; then
        # Linux - open new terminal tab
        gnome-terminal --tab --title="Frontend $port" -- bash -c "cd $(pwd) && PORT=$port yarn dev; exec bash"
    elif command -v wt &> /dev/null; then
        # Windows Terminal
        wt new-tab --title "Frontend $port" -- bash -c "cd $(pwd) && PORT=$port yarn dev"
    else
        echo "Please run the following commands in separate terminals:"
        echo "Terminal 1: PORT=3000 yarn dev"
        echo "Terminal 2: PORT=3001 yarn dev"
    fi
}

# Start frontend instances
run_frontend 3000 "Backend 1 (localhost:8080)"
run_frontend 3001 "Backend 2 (localhost:8081)"

echo ""
echo "Frontend instances started!"
echo "Frontend 1: http://localhost:3000 (connects to Backend 1)"
echo "Frontend 2: http://localhost:3001 (connects to Backend 2)"
echo ""
echo "Make sure your backends are running on:"
echo "Backend 1: http://localhost:8080"
echo "Backend 2: http://localhost:8081"
