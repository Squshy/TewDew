<h2 align="center">
    TewDew
</h2>

<p align="center">
    A GraphQL To-Do tracker made with Rust, AsyncGraphQL, React and Urql.
</p>

![image](https://github.com/Squshy/TewDew/assets/36893334/df005e58-ff5d-4ecf-a2af-7da557a83be7)

## How to run it

### Running the Backend

To run this, first go to the `/apps/squid` directory and run the `./scripts/init_db.sh` script. This will run PostgreSQL in a Docker container fully migrated and ready to go. Once the DB is up and running, run the backend by running `cargo run`.

### Running the Frontend

To run the website, at any directory run `pnpm dev`. This will run the React application in dev mode (most likely on `localhost:5173`).

## What's Wrong

There's some CSS oopsies and some optimistic updates for tewdews and tasks, better code to write and GraphQL dataloader but I'm a little done with this so for now it's chilling like this : )
