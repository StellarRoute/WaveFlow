# Admin API routes

Protected by `x-api-key` header matching one of `API_ADMIN_KEYS` (comma-separated in env).

Router: `crates/api/src/routes/admin.rs`

## `POST /api/v1/admin/programs`

Create a program row linked to an on-chain program ID.

**Request body:**

```json
{
  "on_chain_program_id": 1,
  "github_repo": "StellarRoute/WaveFlow",
  "maintainer_address": "G...",
  "reward_per_point": 100,
  "milestone_cap": null
}
```

**Validation:**
- `github_repo` must pass `validate_github_repo()` (owner/repo slug)
- `reward_per_point` must be positive

**Response:** HTTP 201 with `{ "id": "<uuid>" }`

## `POST /api/v1/admin/contributors`

Register a contributor wallet for a program.

**Request body:**

```json
{
  "program_id": "<uuid from create program>",
  "github_username": "alice",
  "stellar_address": "G..."
}
```

**Response:** HTTP 201 with `{ "program_id": "...", "github_username": "alice" }`

## Example

```bash
curl -X POST http://localhost:8081/api/v1/admin/programs \
  -H "Content-Type: application/json" \
  -H "x-api-key: dev-admin-key-change-in-production" \
  -d '{"on_chain_program_id":1,"github_repo":"StellarRoute/WaveFlow","maintainer_address":"G...","reward_per_point":100}'
```
