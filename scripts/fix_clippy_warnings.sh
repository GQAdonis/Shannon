#!/bin/bash
# Automated Clippy Warning Fixes for Shannon Project
# This script addresses common patterns found in clippy output

set -euo pipefail

echo "🔧 Shannon Clippy Warning Auto-Fix Script"
echo "=========================================="

# Function to fix format strings (uninlined_format_args)
fix_format_strings() {
    echo "📝 Fixing format strings..."
    
    # Fix common format!() patterns in Rust files
    find rust/ -name "*.rs" -type f -exec sed -i '' \
        -e 's/format!("Engine init: {}", e)/format!("Engine init: {e}")/g' \
        -e 's/format!("Module load: {}", e)/format!("Module load: {e}")/g' \
        -e 's/format!("fs open: {}", e)/format!("fs open: {e}")/g' \
        -e 's/format!("WASI command linker: {}", e)/format!("WASI command linker: {e}")/g' \
        -e 's/format!("instantiate: {}", e)/format!("instantiate: {e}")/g' \
        -e 's/format!("alloc: {}", e)/format!("alloc: {e}")/g' \
        -e 's/format!("alloc call: {}", e)/format!("alloc call: {e}")/g' \
        -e 's/format!("memory write: {}", e)/format!("memory write: {e}")/g' \
        -e 's/format!("memory read: {}", e)/format!("memory read: {e}")/g' \
        -e 's/format!("utf8: {}", e)/format!("utf8: {e}")/g' \
        -e 's/format!("json encode: {}", e)/format!("json encode: {e}")/g' \
        -e 's/format!("json parse: {}", e)/format!("json parse: {e}")/g' \
        -e 's/format!("{}.wasm", /format!("{/g' \
        -e 's/format!("{}:{}", name, version)/format!("{name}:{version}")/g' \
        -e 's/format!("{}-{}.wasm", name, version)/format!("{name}-{version}.wasm")/g' \
        -e 's/format!("Failed to compile {}: {}", name, e)/format!("Failed to compile {name}: {e}")/g' \
        -e 's/format!("Network denied to {}", host)/format!("Network denied to {host}")/g' \
        -e 's/format!("Filesystem access denied: {}", path)/format!("Filesystem access denied: {path}")/g' \
        -e 's/format!("Write access denied: {}", path)/format!("Write access denied: {path}")/g' \
        -e 's/format!("Read access denied: {}", path)/format!("Read access denied: {path}")/g' \
        -e 's/format!("FS path not in allowlist: {}", path)/format!("FS path not in allowlist: {path}")/g' \
        -e 's/format!("Host {} not in allowlist", host)/format!("Host {host} not in allowlist")/g' \
        {} \;
}

# Function to fix documentation backticks
fix_doc_backticks() {
    echo "📚 Fixing documentation backticks..."
    
    find rust/ -name "*.rs" -type f -exec sed -i '' \
        -e 's|//! - SurrealDB |//! - `SurrealDB` |g' \
        -e 's|instead of PostgreSQL|instead of `PostgreSQL`|g' \
        -e 's|//! MicroSandbox:|//! `MicroSandbox`:|g' \
        -e 's|Secure MicroVM |Secure `MicroVM` |g' \
        -e 's|the MicroSandbox|the `MicroSandbox`|g' \
        -e 's|their MicroVmProfile|their `MicroVmProfile`|g' \
        -e 's|EventLog to be|`EventLog` to be|g' \
        -e 's|HybridBackend with|`HybridBackend` with|g' \
        -e 's|using MicroSandbox|using `MicroSandbox`|g' \
        {} \;
}

# Function to replace #[allow] with #[expect]
fix_allow_attributes() {
    echo "🔍 Replacing #[allow] with #[expect]..."
    
    find rust/ -name "*.rs" -type f -exec sed -i '' \
        -e 's/#\[allow(dead_code)\]/#[expect(dead_code, reason = "Planned for future use")]/g' \
        -e 's/#\[allow(unused_variables)\]/#[expect(unused_variables, reason = "Parameter reserved for future implementation")]/g' \
        {} \;
}

# Function to fix wildcard imports
fix_wildcard_imports() {
    echo "🌟 Fixing wildcard imports..."
    
    # This is tricky to automate - report them instead
    echo "   Note: Wildcard imports require manual review"
    grep -r "use.*::\\*;" rust/ --include="*.rs" | head -20 || true
}

# Function to fix ignored unit patterns
fix_unit_patterns() {
    echo "🎯 Fixing ignored unit patterns..."
    
    find rust/ -name "*.rs" -type f -exec sed -i '' \
        -e 's/_ = tokio::time::sleep/() = tokio::time::sleep/g' \
        -e 's/let _ = rx\.close()/rx.close()/g' \
        {} \;
}

# Function to fix eprintln format strings
fix_eprintln_formats() {
    echo "🖨️  Fixing eprintln! format strings..."
    
    find rust/ -name "*.rs" -type f -exec sed -i '' \
        -e 's/eprintln!("\[MicroSandbox\] PID {} timed out", pid)/eprintln!("[MicroSandbox] PID {pid} timed out")/g' \
        {} \;
}

# Main execution
echo ""
echo "Starting automated fixes..."
echo ""

fix_format_strings
fix_doc_backticks
fix_allow_attributes
fix_unit_patterns
fix_eprintln_formats

echo ""
echo "✅ Automated fixes complete!"
echo ""
echo "⚠️  Manual review still required for:"
echo "   - Wildcard imports (use specific imports)"
echo "   - Manual Debug implementations (add .finish_non_exhaustive())"
echo "   - Unused async functions (remove async keyword)"
echo "   - Type casts (use try_from instead)"
echo "   - Multiple crate versions (update Cargo.lock)"
echo ""
echo "Run 'cargo clippy --all-features' to see remaining issues"
