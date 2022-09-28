ucd-generate property-values tests/UCD > src/builtin/property_values.rs
ucd-generate property-bool tests/UCD --chars > src/builtin/property_bool.rs
ucd-generate general-category tests/UCD --chars > src/builtin/general_category.rs
#ucd-generate perl-word tests/UCD --chars > src/builtin/perl-word.rs
ucd-generate word-break tests/UCD --chars > src/builtin/word_break.rs
ucd-generate sentence-break tests/UCD --chars > src/builtin/sentence_break.rs
# ucd-generate script tests/UCD --chars > src/builtin/script.rs
ucd-generate script-extension tests/UCD --chars > src/builtin/script_extension.rs
cargo fmt

#    age                          Create Unicode Age tables.
#    bidi-class                   Create the Bidi_Class property tables.  ucd-generate bidi-class tests/UCD --chars
#    bidi-mirroring-glyph         Create Unicode Bidi Mirroring Glyph table.
#    canonical-combining-class    Create the Canonical_Combining_Class table.
#    case-folding-simple          Create a case folding table using the simple mapping.
#    case-mapping                 Create unconditional case mapping tables for upper, lower and
#                                 title case.
#    dfa                          Serialize a single DFAs
#    general-category             Create the General_Category property tables.
#    grapheme-cluster-break       Create a table for each Grapheme_Cluster_Break value.
#    help                         Prints this message or the help of the given subcommand(s)
#    jamo-short-name              Create the Jamo_Short_Name property table.  ucd-generate property-names  tests/UCD
#    joining-type                 Create the Joining_Type property tables. ucd-generate joining-type  tests/UCD
#    names                        Create a mapping from character name to codepoint.
#    perl-word                    Create a boolean property table for the \w character class.
#    property-bool                Create boolean property tables.
#    property-names               Create the canonical property name table.
#    property-values              Create the canonical property value table.
