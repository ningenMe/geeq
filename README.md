# geeq

## openapi generate
```shell
openapi-generator generate -i ./openapi.yaml -g typescript-axios -o ./geeq-frontend/components/generated
openapi-generator generate -i ./openapi.yaml -g rust-axum -p packageName=generated -o ./geeq-backend/generated
```

## sqlx prepare
```
cargo sqlx prepare --workspace --database-url "mysql://${NINGENME_MYSQL_GEEQ_USER_USERNAME}:${NINGENME_MYSQL_GEEQ_USER_PASSWORD}@${NINGENME_MYSQL_HOST}:${NINGENME_MYSQL_PORT}/geeq"
```

## backend start
```shell
# local
ENV=local cargo run -p api
```

## fronend start
```shell
# local
npm run dev
```