ucd-generate property-bool tests/UCD --chars > src/builtin/property_bool.rs
ucd-generate general-category tests/UCD --chars > src/builtin/general_category.rs
#ucd-generate perl-word tests/UCD --chars > src/builtin/perl-word.rs
# ucd-generate script tests/UCD --chars > src/builtin/script.rs
ucd-generate script-extension tests/UCD --chars > src/builtin/script_extension.rs
cargo fmt