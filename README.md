# Personal browsing analyzer

A small project to get better at building Rust apps.

The goal of this project is to build a visualizer, similar to Apple's screen time visualizer, but
for the specific topics you spend time browsing. For example, if you spend 2 hours looking at different
pages related to a HW assignment, and 30 minutes searching for different recipes, you should see
a breakdown of those topics.

# Running locally

## Chrome Extension

1. `pnpm build` (this compiles the typescript file to a js one)
2. Add the extension as a local one in your browser, pointing to the `chrome-extension` directory

## Backend

1. Start the postgres db with
   `docker run -d --name browsing-analysis-db -p 5432:5432 -e POSTGRES_PASSWORD=password -e POSTGRES_DB=browsing-analysis pgvector/pgvector:pg16`
   - To enter the database, run
2. Update your `.env` file appropriately.
3. `cd server && cargo run`

## Frontend

1. `pnpm dev`
