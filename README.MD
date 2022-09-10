# ulina_map

Excluded files:

- All of /data which includes:
- Ulina.db
- /maps
- /backups

as well as public/flags (which includes the flags)

and .env which has the variables:

- ADMIN: the admin password for the site
- CLIENT_ID: the client id of the bot
- DISCORD_TOKEN: the token of the bot
- CLIENT_SECRET: discord client secret
- GUILD_ID: id of the server that the bot operates in
- ADMIN_ID: user id of the admin
- ADMIN_ROLE_ID: role id of admins 
- SECRET_KEY: key used for cookies
- REDIRECT: Oauth redirect after discord login
- DATABASE_URL: path to the database, usually "sqlite:server/data/Ulina.db"
- GOOGLE_KEY: google api key for accessing google sheets