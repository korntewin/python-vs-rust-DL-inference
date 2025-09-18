# === Migrate Data ===

migrate-data:
	@docker compose up --build -d migrate-postgresql

dump-feature-1:
	@docker compose up --build python-dump-feature-1

dump-feature-2:
	@docker compose up --build python-dump-feature-2

dump-data: dump-feature-1 dump-feature-2

# === API Server ===

run-python-pytorch-api-server:
	@docker compose up --build -d python-pytorch-api-server

run-python-onnx-api-server:
	@docker compose up --build -d python-onnx-api-server

run-rust-api-server:
	@docker compose up --build -d rust-api-server

run-python-pytorch-load-tester:
	@docker compose up --build -d python-pytorch-load-tester

run-python-onnx-load-tester:
	@docker compose up --build -d python-onnx-load-tester

run-rust-load-tester:
	@docker compose up --build -d rust-load-tester

run-load-tester-ui:
	@uv run --package python-load-tester locust -f python-load-tester/main.py

run-sanity-check:
	@docker compose up --build sanity-check