# Production Tool: A System Design Example
## What Is This?
Production Tool is a simple system that tracks some generic users, products, and orders, and allows creation and deletion of the items. A MySQL database is accessed via a backend Rust REST API that is used by the Vue3 frontend. I wanted to design a system that simply demonstrated some of my skills

## Prerequisites
- `npm` to build the frontend
    - And some way to host the server (if not using npm vite)
- `rust` and `cargo` for backend
- `docker` for database

## Dev Build Instructions
> Note that the following build scripts are bash scripts. I don't know a whole lot about Windows Powershell or whatever MacOS uses but I don't imagine the scripts will work there
- Run `./scripts/build.sh` from the root of the project. This script does the following:
    - Docker composes database
    - Builds Rust backend
    - Installs npm packages and builds (even though run script doesn't use the frontend build)
- Running `./scripts/run.sh` will run everything, with the frontend being run in dev mode. 
    - For a production environment the frontend could be built `npm run build` and run with some other web server, but that's another dependency that I don't want to control here so do it yourself if you really wanna lol
- Additional things you will have to manually configure:
    - Add a `.env` to `database/` with your database password `MYSQL_ROOT_PASSWORD=PASSWORD`
    - (Optional) If you want to change the database port, do so in the `docker-compose.yaml`
    - Add a `.env` to `backend/` with your database url: `DATABASE_URL=mysql://root:PASSWORD@localhost:3306/pt`
        - Change the password to be the same as your database password
        - If you changed the database port, change `3306` accordingly
    - (Optional) If you want to change the address for the REST API, add `HOST` and `PORT` env variables to your `.env` or elsewhere
        - If you do change the backend address, you need to add a `.env` to `frontend/` and add the address as `VITE_BACKEND_URL`. Otherwise it defaults to `http://localhost:8001`
