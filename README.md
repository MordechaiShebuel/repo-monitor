# repo-monitor
Monitor podman Void repo, report available versions to user through GraphQL API

# Start PostgreSQL locally if you have it installed as a system service.
# Then create a database:
`createdb managed_apps`

# Set the connection string:
`export DATABASE_URL='postgres://localhost/managed_apps`

# Apply the SQL schema from the previous answer:
`psql "$DATABASE_URL" -f schema.sql`

# Start the API on port 8000 (assuming your app uses PORT):
```
export PORT=8000
cargo build
```

# Running in dev mode
```
sudo -u repo-monitor env \
  DATABASE_URL='postgres://%2Ftmp/managed_apps?user=repo-monitor&port=5432' \
  ./target/debug/repo-monitor
```
