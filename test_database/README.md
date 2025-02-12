
# Testing Database Setup

This is for unit and integration testing of the database. 
It is different from the development database defined in dev_database.

## Prerequisites

Ensure you have Docker and Docker Compose installed on your machine.

## Setup Instructions

1. **Create a Docker Volume** for persistent data storage:

   ```bash
   docker volume create --name test_alpha
   ```
2. **Launch the Database** with Docker Compose
   ```bash
   docker-compose up -d
   ```
   
3. **Connect to your Database**
use the command below with **'psql'** to connect to the database. The default password is **'devuser_p'**
   ```bash 
      psql -U testuser -h localhost -p 8999 -d test_alpha
   ```








