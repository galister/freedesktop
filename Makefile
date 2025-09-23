.PHONY: help check test publish-dry-run publish clean

# Default target
help:
	@echo "freedesktop workspace publishing targets:"
	@echo ""
	@echo "  check           - Check all crates compile"
	@echo "  test            - Run all tests"
	@echo "  publish-dry-run - Dry run publish for all crates"
	@echo "  publish         - Publish all crates in correct order"
	@echo "  clean           - Clean build artifacts"
	@echo ""
	@echo "For first-time publishing, run: make publish"

lint:
	cargo clippy

lint-fix:
	cargo clippy --fix

# Development targets
check:
	@echo "🔍 Checking all crates..."
	cargo check --workspace

test:
	@echo "🧪 Running tests..."
	cargo test --workspace

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Dry run - test publishing without actually doing it
publish-dry-run:
	@echo "🔍 Dry run publishing individual crates..."
	@echo "📦 Testing freedesktop-core (should succeed)"
	cargo publish --dry-run -p freedesktop-core
	@echo ""
	@echo "⚠️  Note: Cannot dry-run dependent crates until dependencies are published"
	@echo "   Other crates will fail until their dependencies are published"
	@echo ""
	@echo "✅ freedesktop-core dry run completed!"
	@echo "📋 Ready to publish! Run 'make publish' when ready."

# Define publish order (dependency order matters)
PUBLISH_ORDER := freedesktop-core freedesktop-apps freedesktop-icon freedesktop

# Publish all crates in correct dependency order
publish: check test
	@echo "🚀 Publishing all crates in dependency order..."
	@echo ""
	@echo "This will publish to crates.io:"
	@i=1; for crate in $(PUBLISH_ORDER); do \
		echo "  $$i. $$crate"; \
		i=$$((i+1)); \
	done
	@echo ""
	@read -p "Continue? [y/N] " confirm && [ "$$confirm" = "y" ] || exit 1
	@echo ""
	@total=$$(echo $(PUBLISH_ORDER) | wc -w); \
	i=1; \
	for crate in $(PUBLISH_ORDER); do \
		echo "📦 $$i/$$total Publishing $$crate..."; \
		cargo publish -p $$crate; \
		if [ $$i -lt $$total ]; then \
			echo "⏳ Waiting 60 seconds for crates.io to index $$crate..."; \
			sleep 60; \
		fi; \
		i=$$((i+1)); \
	done
	@echo ""
	@echo "🎉 All crates published successfully!"
	@echo "📋 Next steps:"
	@for crate in $(PUBLISH_ORDER); do \
		echo "  • Check https://crates.io/crates/$$crate"; \
	done