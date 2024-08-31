# imperial-bot

A discord bot for verifying that users are actually from Imperial College London.

## Running

Run either via using the included Dockerfile, or by compiling and running the program using `cargo`.

## Configuration

Configuration is done via environment variables. Environment variables can be set in the environment, _or_ can be set in
a `.env` file in the _same directory_ that the binary lives in.

| Environment Variable | Description                                                                                  | Required, Default       |
| -------------------- | -------------------------------------------------------------------------------------------- | ----------------------- |
| `LOG_LEVEL`          | The logging level for the application. See https://docs.rs/log/latest/log/ for more details. | No, defaults to `error` |
| `DISCORD_TOKEN`      | The application token for the discord bot.                                                   | Yes                     |
| `DATABASE_URL`       | URL to the postgres database.                                                                | Yes                     |
| `SMTP_HOST`          | Host URL/domain for the SMTP mail server used to send verification messages.                 | Yes                     |
| `SMTP_USER`          | The username for the SMTP mail server.                                                       | Yes                     |
| `SMTP_PASS`          | The password for the SMTP mail server.                                                       | Yes                     |
| `SMTP_FROM`          | The email that the discord bot will send messages from (for example, `this@here.com`)        | Yes                     |
