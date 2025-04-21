docker run -p 8080:8080 \
  --name debug-backend \
  --mount type=bind,source="$(pwd)/db/jwt_token",target=/run/secrets/jwt-token,readonly \
  --mount type=bind,source="$(pwd)/db/password.txt",target=/run/secrets/db-password,readonly \
  -e JWT_TOKEN=/run/secrets/jwt-token \
  -e DB_PASSWORD=/run/secrets/db-password \
  -e DB_HOST=host.docker.internal \
  -it backend