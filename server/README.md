# Github pilot server

Make your Github CI processes fly.

Github pilot server enables granular and complex workflows for Github projects that goes beyond what is 
possible in Github Actions.

The server process is straightforward and extensible:
1. Github's webhook service makes a POST call to the server `github/webhook` endpoint.
2. The data from the Github event is compared against the set of Github Pilot rules (defined in `rules.yml`).
3. For any rule that _matches_, the corresponding Actions are executed.

## Configuration

The server configuration is carried out completely by environment variables.

|------------------------------------:-------------------------------------------|-----------------|
| Environment variable               |  Description                              | Default         |
|------------------------------------|-------------------------------------------|-----------------|
| GH_PILOT_HOST                      | The address the server listens at         | 127.0.0.1       |
| GH_PILOT_PORT                      | The port the server listens at            | 8330            |
| GH_PILOT_RULESET_PATH              | The file path for the rule set            | rules.yaml      |
|------------------------------------|-------------------------------------------|-----------------|

If a file called `.env` exists in the current directory, the environment variables will be configured from that.