# Backend (Rust + Axum)

Initial MVP backend implementation for the Marketing Tools platform.

## Implemented endpoints

- `GET /health`
- `POST /auth/register`
- `POST /auth/login`
- `POST /content/create`
- `POST /content/schedule`
- `GET /content/list`
- `POST /ads/create`
- `GET /ads/performance`
- `GET /messages`
- `POST /messages/respond`
- `GET /analytics/dashboard`

## Run locally

```bash
cd /home/runner/work/marketing-tools/marketing-tools/backend
cargo run
```

Server starts on `http://127.0.0.1:3000`.

## Test

```bash
cd /home/runner/work/marketing-tools/marketing-tools/backend
cargo test
```

## Notes

- Current persistence is in-memory (`RwLock<Vec<_>>`) for MVP scaffolding.
- Next step is replacing in-memory storage with PostgreSQL/Supabase repositories.
