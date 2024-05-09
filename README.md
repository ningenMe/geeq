# geeq

## openapi generate
```shell
openapi-generator generate -i ./openapi.yaml -g typescript-axios -o ./geeq-frontend/components/generated
openapi-generator generate -i ./openapi.yaml -g rust-axum -p packageName=generated -o ./geeq-backend/generated
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