# Personal browsing analyzer

A small project to get better at building Rust apps.

The goal of this project is to build a visualizer, similar to Apple's screen time visualizer, but
for the specific topics you spend time browsing. For example, if you spend 2 hours looking at different
pages related to a HW assignment, and 30 minutes searching for different recipes, you should see
a breakdown of those topics.

## Running locally

1. Start the postgres db with
   `docker run -d --name browsing-analysis-db -p 5432:5432 -e POSTGRES_PASSWORD={your_password} -e POSTGRES_DB=browsing-analysis postgres`
2. Update your `.env` file appropriately.
3. `cd server && cargo run`
