# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain if needed
install:
	# Install if needed
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks

# Example: Extract data
extract: 
	cargo run extract

# Example: Transform and Load data
transform_load:
	cargo run transform_load

# Example: Create a database entry
create:
	cargo run query "INSERT INTO AlcoholDB (id,country,beer_servings,spirit_servings,wine_servings,total_pure_alcohol) VALUES (1000, 'A', 10, 10, 10, 11.1);"

# Example: Read from the database
read:
	cargo run query "SELECT * FROM AlcoholDB WHERE id = 10;"

# Example: Update a database entry
update:
	cargo run query "UPDATE AlcoholDB SET id=10, country='B', beer_servings=1, spirit_servings=1, wine_servings=1, total_pure_alcohol=1.1 WHERE id=10;"

# Example: Delete a database entry
delete:
	cargo run query "DELETE FROM AlcoholDB WHERE id=3;"

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi