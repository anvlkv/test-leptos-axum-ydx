## Deploy

### prepare queries

```
cd common
cargo sqlx prepare -- --features ssr
```

### push img

```
  docker build . -t cr.yandex/crptgaq4h1ds45ulihpq/test-task-ydx:latest && docker push cr.yandex/crptgaq4h1ds45ulihpq/test-task-ydx:latest
```
