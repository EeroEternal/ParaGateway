#!/bin/bash

# ParaGateway One-Click Startup Script

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting ParaGateway Setup...${NC}"

# 1. Check for .env file
if [ ! -f .env ]; then
    echo -e "${YELLOW}Warning: .env file not found. Creating a default one...${NC}"
    echo "ADMIN_TOKEN=admin123" > .env
    echo "ADDR=127.0.0.1:18765" >> .env
    echo "DATABASE_URL=sqlite://paragateway.db?mode=rwc" >> .env
    echo -e "${GREEN}Created .env with default values. Port set to 18765.${NC}"
fi

# 2. Start Backend
echo -e "${BLUE}Starting Backend (Rust)...${NC}"
cargo run &
BACKEND_PID=$!

# 3. Start Frontend
echo -e "${BLUE}Preparing Frontend...${NC}"
if [ -d "web" ]; then
    (
        cd web
        if [ ! -d "node_modules" ]; then
            echo -e "${YELLOW}Frontend: node_modules not found. Running npm install (this may take a minute)...${NC}"
            npm install > /dev/null 2>&1
        fi
        echo -e "${BLUE}Starting Frontend (Vite) on http://localhost:18764...${NC}"
        npm run dev
    ) &
    FRONTEND_PID=$!
else
    echo -e "${YELLOW}Warning: web directory not found. Skipping frontend.${NC}"
fi

# Function to handle shutdown
cleanup() {
    echo -e "\n${BLUE}Shutting down ParaGateway...${NC}"
    kill $BACKEND_PID 2>/dev/null
    if [ ! -z "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null
    fi
    echo -e "${GREEN}Done.${NC}"
    exit
}

# Trap SIGINT (Ctrl+C) and SIGTERM
trap cleanup SIGINT SIGTERM

echo -e "${GREEN}ParaGateway is running!${NC}"
echo -e "Backend: http://localhost:18765"
echo -e "Frontend: http://localhost:18764"
echo -e "Press Ctrl+C to stop both services."

# Wait for background processes
wait
