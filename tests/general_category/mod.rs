#[allow(dead_code, unused_variables)]
mod generated;

use character_set::{CharacterSet, DumpAction};
use generated::*;
use std::{fs::OpenOptions, io::Write};

#[test]
fn build_generated() -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open("src/builtin/generated.rs")?;
    writeln!(file, "use std::ops::RangeInclusive;\n",)?;
    // Letter
    writeln!(file, "{}\n", build_item_range("LETTER", LETTER))?;
    writeln!(file, "{}\n", build_item_range("LOWERCASE_LETTER", LOWERCASE_LETTER))?;
    writeln!(file, "{}\n", build_item_range("UPPERCASE_LETTER", UPPERCASE_LETTER))?;
    writeln!(file, "{}\n", build_item_range("TITLECASE_LETTER", TITLECASE_LETTER))?;
    writeln!(file, "{}\n", build_item_range("CASED_LETTER", CASED_LETTER))?;
    writeln!(file, "{}\n", build_item_range("MODIFIER_LETTER", MODIFIER_LETTER))?;
    writeln!(file, "{}\n", build_item_range("OTHER_LETTER", OTHER_LETTER))?;
    // Mark
    writeln!(file, "{}\n", build_item_range("MARK", MARK))?;
    writeln!(file, "{}\n", build_item_range("NONSPACING_MARK", NONSPACING_MARK))?;
    // writeln!(file, "{}\n", build_item_range("Spacing_Combining_Mark", Space))?;
    writeln!(file, "{}\n", build_item_range("ENCLOSING_MARK", ENCLOSING_MARK))?;
    // Separator
    writeln!(file, "{}\n", build_item_range("SEPARATOR", SEPARATOR))?;
    writeln!(file, "{}\n", build_item_range("SPACE_SEPARATOR", SPACE_SEPARATOR))?;
    writeln!(file, "{}\n", build_item_range("LINE_SEPARATOR", LINE_SEPARATOR))?;
    writeln!(file, "{}\n", build_item_range("PARAGRAPH_SEPARATOR", PARAGRAPH_SEPARATOR))?;
    // Symbol
    writeln!(file, "{}\n", build_item_range("SYMBOL", SYMBOL))?;
    writeln!(file, "{}\n", build_item_range("MATH_SYMBOL", MATH_SYMBOL))?;
    writeln!(file, "{}\n", build_item_range("CURRENCY_SYMBOL", CURRENCY_SYMBOL))?;
    writeln!(file, "{}\n", build_item_range("MODIFIER_SYMBOL", MODIFIER_SYMBOL))?;
    writeln!(file, "{}\n", build_item_range("OTHER_SYMBOL", OTHER_SYMBOL))?;
    // Number
    writeln!(file, "{}\n", build_item_range("NUMBER", NUMBER))?;
    writeln!(file, "{}\n", build_item_range("DECIMAL_NUMBER", DECIMAL_NUMBER))?;
    writeln!(file, "{}\n", build_item_range("LETTER_NUMBER", LETTER_NUMBER))?;
    writeln!(file, "{}\n", build_item_range("OTHER_NUMBER", OTHER_NUMBER))?;
    //     // Punctuation
    //     ("P", "Punctuation", LETTER),
    //     ("Pd", "Dash_Punctuation", LETTER),
    //     ("Ps", "Open_Punctuation", LETTER),
    //     ("Pe", "Close_Punctuation", LETTER),
    //     ("Pi", "Initial_Punctuation", LETTER),
    //     ("Pf", "Final_Punctuation", LETTER),
    //     ("Pc", "Connector_Punctuation", LETTER),
    //     ("Po", "Other_Punctuation", LETTER),
    //     // Other
    //     ("C", "Other", LETTER),
    //     ("Cc", "Control", LETTER),
    //     ("Cf", "Format", LETTER),
    //     ("Co", "Private_Use", LETTER),
    //     ("Cs", "Surrogate", LETTER),
    //     ("Cn", "Unassigned", LETTER),
    Ok(())
}

fn build_item_range(name: &str, item: &[(u32, u32)]) -> String {
    let dump = DumpAction {
        name: name.to_string(),
        public: "pub".to_string(),
        skip_fmt: true,
        dump_tree: true,
        dump_check: true,
        dump_range: true,
        trie_set: "".to_string(),
    };
    let mut set = CharacterSet::nil();
    for i in item {
        set.include(*i).unwrap_or_default()
    }
    return dump.dump(&set).unwrap_or_default();
}
