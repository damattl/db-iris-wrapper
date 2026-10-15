# DB IRIS Wrapper
Wrapper around the [IRIS Timetable API](https://iris.noncd.db.de) from Deutsche Bahn. \
The data is requested periodically and stored in a database.

[API Documentation](https://db-iris.it-solutions-mayer.de/v1/swagger/index.html)

## Limitations:
- Can only supply with data that is already stored
- Is currently limited to stations that are capable of handling intercity trains

## Deployment:
The project can be deployed using Docker Compose. `docker-compose.yaml` holds the shared base, \
`docker-compose.dev.yaml` and `docker-compose.prod.yaml` the environment specific overrides. \
Run `make dev` locally or `make prod` on the server (requires `DOMAIN` and `POSTGRES_PASSWORD`). \
All required environment variables are documented in the .env.example file. \
Currently it is available under [https://db-iris.it-solutions-mayer.de](https://db-iris.it-solutions-mayer.de)
(Deployed via Dokku)

## Timezones:
Timestamps are stored in display time.
If the train arrived at the stop at 11.09.2025 19:22 CEST this is stored in the database as 2025-09-11 19:22:00.000000

## ToDo:
- [ ] Add more tests
- [ ] Add Documentation
- [ ] Handle errors more explicitly
- [x] Add UI
- [x] Document deployment
- [x] Make querying the api more efficient (always query 6 hours in advance)
- [x] Merge trains that go over the date border (train 422-250912 that travels from 22:00 - 02:00 is the same as 422-250913)
- [x] Include current arrival times (trains that are late)

This is my first rust project, so feedback on the code is always appreciated.
