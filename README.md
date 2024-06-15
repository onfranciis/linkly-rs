# Linkly RS

### Bitly, but on rust

Linkly RS is a url shortner service built on Rust using the [Rocket](https://rocket.rs/) web server framework, Redis and PostgreSQL database. It's easy to host, just have the [rust tool chain](https://www.rust-lang.org/tools/install) installed on your system and an env file with the configuration below.

```bash
DATABASE_URL = "postgres://<username>:<password>@<url>/linkly_rs"
```

<br>

Now, in your directory, run

```bash
$ cargo run
```

## API

This service offers 4 end points

- **GET** /
  <br>
  This is a health check end point.

  <br>

  **_Response_**

  ```json
  // OK - 200
  {
    "connected": true
  }
  ```

  <br>

- **POST** /url
  <br>
  Add new url

  <br>

  **_Body_**

  ```json
  // x-www-form-urlencoded
  {
    "url": "onfranciis.dev"
  }
  ```

  <br>

  **_Response_**

  ```json
  // OK - 200
  {
    "result": {
      "id": 5,
      "long": "http://onfranciis.dev",
      "short": "a4f8",
      "date": "05 Jun-2024 06:58:00am +0100"
    },
    "err": null
  }
  ```

  ```json
  // Conflict - 409
  {
    "result": null,
    "err": "Seems like this url has already been shortened! Is it 'a4f8' ?"
  }
  ```

  <br>

- **GET** /url/{id}
  <br>
  Redirect if successful

  <br>

  **_Response_**

  ```json
  // Not Found - 404
  {
    "result": null,
    "err": "This url is invalid! Kindly confirm"
  }
  ```

- **GET** /url
  <br>
  Return all urls

  <br>

  **_Response_**

  ```json
  // OK - 200
  {
    "result": [
      {
        "id": 3,
        "long": "http://google.com",
        "short": "00a5",
        "date": "02 Jun-2024 13:40:56pm +0100"
      },
      {
        "id": 5,
        "long": "http://onfranciis.dev",
        "short": "a4f8",
        "date": "05 Jun-2024 06:58:00am +0100"
      },
      {
        "id": 6,
        "long": "http://test.com",
        "short": "e423",
        "date": "05 Jun-2024 07:06:43am +0100"
      }
    ],
    "err": null
  }
  ```

<br>

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white) ![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black) ![VS Code Insiders](https://img.shields.io/badge/VS%20Code%20Insiders-35b393.svg?style=for-the-badge&logo=visual-studio-code&logoColor=white) ![Redis](https://img.shields.io/badge/redis-%23DD0031.svg?style=for-the-badge&logo=redis&logoColor=white)

For support and enquiries, reach out via [hello@onfranciis.dev](mailto:hello@onfranciis.dev)
