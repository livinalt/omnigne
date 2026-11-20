# Infrastructure

See the [local development guide](../docs/LOCAL_DEVELOPMENT.md) for Docker prerequisites and optional journal persistence.

The root Dockerfile runs the one-shot paper scanner as an unprivileged user. It does not run a live trading service.

```bash
docker build -t omnigne:dev .
docker run --rm omnigne:dev
```

Add the following when the corresponding runtime behavior exists:

- RPC/indexer service configuration and backfill checkpoints.
- A persistent transaction database and recovery procedure.
- Prometheus scrape configuration once metrics are exposed.
- Grafana dashboards for latency, failed fees and reconciled PnL.
- Optional alert adapters with configured recipients.

Keep production credentials outside source control. No signing service or deployment automation is included in this starter.
