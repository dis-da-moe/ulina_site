# ulina_site

This is the repository for the archived discord bot and website of the Ulina project: https://www.ulinaworld.com

It's open source to allow people from the community to contribute and to provide an example for others making a site/bot with rocket/yew/serenity, since I haven't found many other examples of this.

## Structure

This is a cargo workspace with three crates, server, tools and common.

### Server

The backend, this contains the Rocket server, the sqlite database (connected with sqlx) and the Serenity discord bot. The bot and server both share the database through a pool of connections, and so they will always be in-sync.


### Tools

The frontend, an SPA made with Yew, which makes up the [tools](https://www.ulinaworld.com/tools) section of the site (the rest of the site is static html). It regularly makes requests to the Rocket server, and so there's the final crate:

### Common

This is a library that contains code used both by the frontend and backend, such as the struct's for serialising/deserialising JSON and the functions for converting time.

## Building

Trunk is used to build the Tools crate and place it in `/server/public/tools/`.
Nodejs then runs tailwind to generate the css file.
Finally Cargo runs the server.
This can be seen in `run.ps1`

### Excluded files
In order to run this, you need to add the following files:

- All of `/server/data` which includes:   
    - `Ulina.db` (schema available in `schema.sql`)
    - `/maps`
    - `/backups`
<br></br>
- Some `/server/public` files:
    - all of `/flags` (includes flags)
    - `tools/tools.js` & `tools/tools_bg.wasm` (the yew build) 
<br></br>
- `/server/.env` which has the variables:

    - `ADMIN`: the admin password for the site
    - `CLIENT_ID`: the client id of the bot
    - `DISCORD_TOKEN`: the token of the bot
    - `CLIENT_SECRET`: discord client secret
    - `GUILD_ID`: id of the server that the bot operates in
    - `ADMIN_ID`: user id of the admin
    - `ADMIN_ROLE_ID`: role id of admins 
    - `SECRET_KEY`: key used for cookies
    - `REDIRECT`: Oauth redirect after discord login
    - `DATABASE_URL`: path to the database, usually "sqlite:server/data/Ulina.db"
    - `GOOGLE_KEY`: google api key for accessing google sheets
