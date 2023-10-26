use crate::modifier::DataValidation;
use umya_spreadsheet as u;

#[derive(Debug)]
pub(super) struct CellValidation(pub(super) a1_notation::Address, pub(super) DataValidation);

macro_rules! custom_validation {
    ($v:ident, $type:ident, $formula1:tt, $prompt:expr) => {
        $v.set_formula1($formula1.to_string())
            .set_type(u::DataValidationValues::$type)
            .set_prompt($prompt)
    };

    ($v:ident, $type:ident, $formula1:tt, $formula2:tt, $prompt:expr) => {
        custom_validation!($v, $type, $formula1, $prompt).set_formula2($formula2.to_string())
    };
}

macro_rules! validation {
    ($v:ident, $type:ident, $op:ident, $formula1:tt, $prompt:expr) => {
        custom_validation!($v, $type, $formula1, $prompt)
            .set_operator(u::DataValidationOperatorValues::$op)
    };

    ($v:ident, $type:ident, $op:ident, $formula1:tt, $formula2:tt, $prompt:expr) => {
        custom_validation!($v, $type, $formula1, $formula2, $prompt)
            .set_operator(u::DataValidationOperatorValues::$op)
    };
}

// TODO:
// * optimize this so there isn't a separate data validation for each cell - if a data validation
//     is placed on a fill, we can specify the range covered by that fill instead of each cell
// * .set_allow_blank()? does GS allow that? I think we'd need additional syntax for required vs
//     not required validations
// * .set_prompt_title() too?
// * does it matter that I use Decimal rather than Whole?
impl From<CellValidation> for u::DataValidation {
    fn from(CellValidation(position, dv): CellValidation) -> Self {
        let mut sqref = u::SequenceOfReferences::default();
        sqref.set_sqref(position.to_string());

        let mut validation = u::DataValidation::default();
        validation.set_sequence_of_references(sqref);

        match dv {
            DataValidation::Custom(c) => {
                custom_validation!(validation, Custom, c, format!("Custom formula: {c}"));
            }

            DataValidation::DateAfter(d) => {
                validation!(validation, Date, GreaterThan, d, format!("Date after {d}"));
            }

            DataValidation::DateBefore(d) => {
                validation!(validation, Date, LessThan, d, format!("Date before {d}"));
            }

            DataValidation::DateBetween(d1, d2) => {
                validation!(
                    validation,
                    Date,
                    Between,
                    d1,
                    d2,
                    format!("Date between {d1} and {d2}")
                );
            }

            DataValidation::DateEqualTo(d) => {
                validation!(validation, Date, Equal, d, format!("Date equal to {d}"));
            }

            DataValidation::DateIsValid => {
                let f = format!("=ISNUMBER(DAY({position}))");
                custom_validation!(validation, Custom, f, "Date is valid");
            }

            DataValidation::DateNotBetween(d1, d2) => {
                validation!(
                    validation,
                    Date,
                    NotBetween,
                    d1,
                    d2,
                    format!("Date not between {d1} and {d2}")
                );
            }

            DataValidation::DateOnOrAfter(d) => {
                validation!(
                    validation,
                    Date,
                    GreaterThanOrEqual,
                    d,
                    format!("Date on or after {d}")
                );
            }

            DataValidation::DateOnOrBefore(d) => {
                validation!(
                    validation,
                    Date,
                    LessThanOrEqual,
                    d,
                    format!("Date on or before {d}")
                );
            }

            DataValidation::NumberBetween(n1, n2) => {
                validation!(
                    validation,
                    Decimal,
                    Between,
                    n1,
                    n2,
                    format!("Number between {n1} and {n2}")
                );
            }

            DataValidation::NumberEqualTo(n) => {
                validation!(
                    validation,
                    Decimal,
                    Equal,
                    n,
                    format!("Number equal to {n}")
                );
            }

            DataValidation::NumberGreaterThan(n) => {
                validation!(
                    validation,
                    Decimal,
                    GreaterThan,
                    n,
                    format!("Number greater than {n}")
                );
            }

            DataValidation::NumberGreaterThanOrEqualTo(n) => {
                validation!(
                    validation,
                    Decimal,
                    GreaterThanOrEqual,
                    n,
                    format!("Number greater than or equal to {n}")
                );
            }

            DataValidation::NumberLessThan(n) => {
                validation!(
                    validation,
                    Decimal,
                    LessThan,
                    n,
                    format!("Number less than {n}")
                );
            }

            DataValidation::NumberLessThanOrEqualTo(n) => {
                validation!(
                    validation,
                    Decimal,
                    LessThanOrEqual,
                    n,
                    format!("Number less than or equal to {n}")
                );
            }

            DataValidation::NumberNotBetween(n1, n2) => {
                validation!(
                    validation,
                    Decimal,
                    NotBetween,
                    n1,
                    n2,
                    format!("Number not between {n1} and {n2}")
                );
            }

            DataValidation::NumberNotEqualTo(n) => {
                validation!(
                    validation,
                    Decimal,
                    NotEqual,
                    n,
                    format!("Number not equal to {n}")
                );
            }

            DataValidation::TextContains(t) => {
                let f = format!("=ISNUMBER(SEARCH(\"{t}\", {position}))");
                custom_validation!(validation, Custom, f, format!("Text contains \"{t}\""));
            }

            DataValidation::TextDoesNotContain(t) => {
                let f = format!("=NOT(ISNUMBER(SEARCH(\"{t}\", {position})))");
                custom_validation!(
                    validation,
                    Custom,
                    f,
                    format!("Text does not contain \"{t}\"")
                );
            }

            DataValidation::TextEqualTo(t) => {
                let f = format!("{position} = \"{t}\"");
                custom_validation!(validation, Custom, f, format!("Text equal to \"{t}\""));
            }

            DataValidation::TextIsValidEmail => {
                // TODO: can probably do better, make a stdlib of useful functions?
                let f = format!("=ISNUMBER(MATCH(\"*@*.?*\", {position}, 0))");
                custom_validation!(validation, Custom, f, "Text is valid email");
            }

            DataValidation::TextIsValidUrl => {
                // TODO
                let f = format!("=RegExpMatch({position}, \"^http\")");
                custom_validation!(validation, Custom, f, "Text is valid URL");
            }

            DataValidation::ValueInList(values) => {
                let list_as_string = &values
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                validation!(
                    validation,
                    List,
                    Equal,
                    list_as_string,
                    format!("Value in list {list_as_string}")
                );
            }

            DataValidation::ValueInRange(a1) => {
                validation!(validation, List, Equal, a1, format!("Value in range {a1}"));
            }
        }
        validation
    }
}
