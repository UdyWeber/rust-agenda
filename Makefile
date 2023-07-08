update_schema:
	diesel print-schema > ./back-end/src/database/schema.rs
backend:
	cd ./back-end && cargo run
frontend:
	cd ./front-end && trunk serve
migrate:
	cd ./back-end && diesel migration run