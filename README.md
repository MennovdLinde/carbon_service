# carbon_service

Rust microservice for calculating the carbon footprint of web pages. Part of the [Green Web Audit](https://greenwebaudit-frontend.vercel.app) platform.

**Live:** [greenwebaudit-carbon-8d141f0eb525.herokuapp.com](https://greenwebaudit-carbon-8d141f0eb525.herokuapp.com)

---

## API

### `POST /calculate`

Calculate CO2 emissions for a given number of bytes transferred.

**Request:**
```json
{ "bytes": 500000 }
```

**Response:**
```json
{
  "bytes": 500000,
  "carbon_grams": 1.0
}
```

**Methodology:** ~2g CO2 per MB transferred (industry standard).

### `GET /health`

Returns `{ "status": "ok" }`.

---

## Tech Stack

- **Rust** (edition 2024)
- **actix-web 4** — HTTP server
- **serde / serde_json** — JSON serialization
- **tokio** — async runtime
- Deployed on **Heroku**

---

## Local Development

```bash
# Run locally on port 8001
cargo run

# Or set a custom port
PORT=8080 cargo run
```

Test it:
```bash
curl -X POST http://localhost:8001/calculate \
  -H "Content-Type: application/json" \
  -d '{"bytes": 500000}'
```

---

## Deployment

Deployed on Heroku using the [Rust buildpack](https://github.com/emk/heroku-buildpack-rust). The `Procfile` runs the compiled binary.

```
web: ./target/release/carbon_service
```

---

## Related Projects

| Project | Description | Link |
|---------|-------------|------|
| greenwebaudit-frontend | React dashboard | [Vercel](https://greenwebaudit-frontend.vercel.app) |
| greenwebaudit-backend | Node.js audit API | [Heroku](https://greenwebaudit-backend-11644999869f.herokuapp.com) |
| green-audit-cli | CLI tool for terminal-based audits | [GitHub](https://github.com/MennovdLinde/green-audit-cli) |

---

## License

MIT
