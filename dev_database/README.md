
# Development Test Database Setup

Easily set up a development test database with Docker. Follow the steps below to initialize and connect to the database.

## Prerequisites

Ensure you have Docker and Docker Compose installed on your machine.

## Setup Instructions

1. **Create a Docker Volume** for persistent data storage:

   ```bash
   docker volume create --name d_alpha
   ```
2. **Launch the Database** with Docker Compose
   ```bash
   docker-compose up -d
   ```
   
3. **Connect to your Database**
use the command below with **'psql'** to connect to the database. The default password is **'devuser_p'**
   ```bash 
      psql -U devuser -h localhost -p 6999 -d d_alpha
   ```

## IMPORTANT NOTE!
 This is the only supplied **.env** file


** To regenerate the database documentation:
```bash
   rm -rf ../db_relations/*;
   java -jar ~/local/bin/schemaspy-6.2.4.jar \
    -t pgsql11 \
    -dp ~/local/bin/postgresql-42.6.0.jar \
    -db d_alpha \
    -host localhost \
    -port 6999 \
    -u devuser \
    -p devuser_p \
    -o ../db_relations;
   ```

```bash
google-chrome ../db_relations/index.html 
```



