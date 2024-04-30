# roman-api

# openapi generate
```shell
openapi-generator generate -i ./openapi.yaml -g typescript-axios -o ./geeq-frontend/components/generated
openapi-generator generate -i ./openapi.yaml -g rust-axum -p packageName=generated -o ./geeq-backend/generated
```