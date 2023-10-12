hello:
	echo "Hello, World"

getjson:
	curl \
	--header "Accept: application/json" \
	--request GET 'http://localhost:3000/json'

putjson:
	curl \
	--header "Content-Type: application/json" \
	--request PUT \
	--data '{"name":"John Doe","age":30}' \
	'http://localhost:3000/json'	

putbooks:
	curl \
	--request PUT 'http://localhost:3000/books' \
	--header "Content-Type: application/json" \
	--data '{"id":4,"title":"Decameron","author":"Giovanni Boccaccio"}'

tty:
	docker exec -it axum_crud_test_web bash

migration:
	sqlx migrate add "initial database setup"