# How to Build Linkly RS :wrench:

To build **Linkly RS**, let's get familiar with the concept used.
Firstly, you can add a new url to the database using the POST request to the _/url_ and if the url has not been added already, the Redis database is updated and you should get a 200 status alongside the short version of the url.
When making a GET request to the _/url/{short}_ it checks if the url exists on redis and if it does, it redirects the client to the original url.

### Pre Requisite

I expect you to have some knowledge on

- [Rust](https://www.rust-lang.org/learn)
- [Redis](https://redis.io/university/)
- [PostgreSQL](https://www.postgresqltutorial.com)
- [Rocket](https://rocket.rs/guide/v0.5/) (A rust based web framework)
