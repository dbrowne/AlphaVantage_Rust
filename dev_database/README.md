Test database for development use these commands to start the database and connect to it.
Default password is devuser_p

NOTE. THIS IS THE ONLY SUPPLIED .env file. 
An example will be provided at a later date

```
docker volume create --name d_alpha
docker-compose up -d

psql -U devuser -h localhost -p 6999 -d d_alpha
```