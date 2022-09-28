use std::ops::RangeInclusive;

pub use generated::*;

mod generated;

pub static BUILTIN_CHARACTER_RANGES: &'static [(&'static str, &'static str, &'static [RangeInclusive<char>])] = &[
    // Letter
    ("XID_START", "XID_Start", LETTER),
];

/// https://www.regular-expressions.info/unicode.html#prop
pub const REGEX_CHARACTER_RANGES: &'static [(&'static str, &'static str, &'static [RangeInclusive<char>])] = &[
    // Letter
    ("L", "Letter", LETTER),
    ("Ll", "Lowercase_Letter", LOWERCASE_LETTER),
    ("Lu", "Uppercase_Letter", UPPERCASE_LETTER),
    ("Lt", "Titlecase_Letter", TITLECASE_LETTER),
    ("Lm", "Modifier_Letter", MODIFIER_LETTER),
    ("Lo", "Other_Letter", OTHER_LETTER),
    ("L&", "Cased_Letter", CASED_LETTER),
    // Mark
    ("M", "Mark", MARK),
    ("Mn", "Non_Spacing_Mark", NONSPACING_MARK),
    ("Mc", "Spacing_Combining_Mark", LETTER),
    ("Me", "Enclosing_Mark", ENCLOSING_MARK),
    // Separator
    ("Z", "Separator", SEPARATOR),
    ("Zs", "Space_Separator", SPACE_SEPARATOR),
    ("Zl", "Line_Separator", LINE_SEPARATOR),
    ("Zp", "Paragraph_Separator", PARAGRAPH_SEPARATOR),
    // Symbol
    ("S", "Symbol", SYMBOL),
    ("Sm", "Math_Symbol", MATH_SYMBOL),
    ("Sc", "Currency_Symbol", CURRENCY_SYMBOL),
    ("Sk", "Modifier_Symbol", MODIFIER_SYMBOL),
    ("So", "Other_Symbol", OTHER_SYMBOL),
    // Number
    ("N", "Number", NUMBER),
    ("Nd", "Decimal_Digit_Number", DECIMAL_NUMBER),
    ("Nl", "Letter_Number", LETTER_NUMBER),
    ("No", "Other_Number", OTHER_NUMBER),
    // Punctuation
    ("P", "Punctuation", LETTER),
    ("Pd", "Dash_Punctuation", LETTER),
    ("Ps", "Open_Punctuation", LETTER),
    ("Pe", "Close_Punctuation", LETTER),
    ("Pi", "Initial_Punctuation", LETTER),
    ("Pf", "Final_Punctuation", LETTER),
    ("Pc", "Connector_Punctuation", LETTER),
    ("Po", "Other_Punctuation", LETTER),
    // Other
    ("C", "Other", LETTER),
    ("Cc", "Control", LETTER),
    ("Cf", "Format", LETTER),
    ("Co", "Private_Use", LETTER),
    ("Cs", "Surrogate", LETTER),
    ("Cn", "Unassigned", LETTER),
];
