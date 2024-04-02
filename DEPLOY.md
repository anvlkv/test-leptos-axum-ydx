## Deploy

### prepare queries

```
cargo sqlx prepare --workspace -- --features ssr
```

### push img

```
  docker build . -t cr.yandex/crptgaq4h1ds45ulihpq/test-task-ydx:latest && docker push cr.yandex/crptgaq4h1ds45ulihpq/test-task-ydx:latest
```
