# DB IRIS Wrapper
Wrapper around the [IRIS Timetable API](https://iris.noncd.db.de) from Deutsche Bahn.
The data is requested periodically and stored in a database.

[API Documentation](https://db-iris.it-solutions-mayer.de/v1/swagger/index.html)

## Limitations:
- Can only supply with data that is already stored
- Is currently limited to stations that are capable of handling intercity trains

## Deployment:
The project can be deployed using the docker-compose.yaml
Currently it is available under [https://db-iris.it-solutions-mayer.de](https://db-iris.it-solutions-mayer.de)

## Timezones:
Timestamps are stored in display time.
If the train arrived at the stop at 11.09.2025 19:22 CEST this is stored in the database as 2025-09-11 19:22:00.000000

## ToDo:
- [ ] Add more tests
- [ ] Add Documentation
- [ ] Add UI
- [ ] Document deployment
- [ ] Make querying the api more efficient (always query 6 hours in advance)
- [ ] Merge trains that go over the date border (train 422-250912 that travels from 22:00 - 02:00 is the same as 422-250913)

This is my first rust project, it took me about 40 hours so far to write it
