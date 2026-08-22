mkdir -p  certs/

mkcert -install
mkcert \
  -cert-file certs/todo.localhost.pem \
  -key-file certs/todo.localhost.key \
  todo.localhost
