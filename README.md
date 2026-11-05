# repo-monitor
## Version 0.1.0
## Known Bugs:

## Feature Request
- Add/Delete items to list through GraphQL

## Program Description
Monitor podman Void repo, report available versions to user through GraphQL API

## Start PostgreSQL locally if you have it installed as a system service.
## Then create a database:
`createdb managed_apps`

## Set the connection string:
`export DATABASE_URL='postgres://localhost/managed_apps`

## Apply the SQL schema from the previous answer:
`psql "$DATABASE_URL" -f schema.sql`

## Start the API on port 8000 (assuming your app uses PORT):
```
export PORT=8000
cargo build
```

## Running in dev mode
```
sudo -u repo-monitor env \
  DATABASE_URL='postgres://%2Ftmp/managed_apps?user=repo-monitor&port=5432' \
  ./target/debug/repo-monitor
```

## Query the API

The examples below assume the GraphQL endpoint is `http://localhost:8000/graphql`. These use `POST`, which is suitable for queries and mutations.

```bash
# List managed apps
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ managedApps { id name endpoint version } }"}'
```

```bash
# Check the Podman repository service
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ podmanRepoStatus { state checkedAt message } }"}'
```

```bash
# Check the build server
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"{ buildServerStatus { state checkedAt currentJobId } }"}'
```

### Run mutations

```bash
# Set application 42's version to 2.4.1
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"mutation { changeAppVersion(appId: \"42\", version: \"2.4.1\") { id name version } }"}'
```

```bash
# Request a repository restart
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"mutation { restartPodmanRepo { accepted message } }"}'
```

```bash
# Queue a build
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"mutation { runBuild { id state startedAt message } }"}'
```

If `runBuild` returns a job ID, poll it with:

```bash
curl -s http://localhost:8000/graphql \
  -H 'Content-Type: application/json' \
  -d '{"query":"query { buildJob(id: \"JOB_ID\") { id state startedAt finishedAt message } }"}'
```

## Running unit tests:
