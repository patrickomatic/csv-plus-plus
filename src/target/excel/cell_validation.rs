use crate::modifier::DataValidation;
use umya_spreadsheet as u;

#[derive(Debug)]
pub(super) struct CellValidation(pub(super) a1_notation::Address, pub(super) DataValidation);

// TODO:
// * optimize this so there isn't a separate data validation for each cell - if a data validation
//     is placed on a fill, we can specify the range covered by that fill instead of each cell
// * finish the unimplemented ones
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
                validation
                    .set_formula1(c.clone())
                    .set_type(u::DataValidationValues::Custom)
                    .set_prompt(format!("Custom formula: {c}"));
            }
            DataValidation::DateAfter(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThan)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date after {d}"));
            }
            DataValidation::DateBefore(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThan)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date before {d}"));
            }
            DataValidation::DateBetween(d1, d2) => {
                validation
                    .set_formula1(d1.to_string())
                    .set_formula2(d2.to_string())
                    .set_operator(u::DataValidationOperatorValues::Between)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date between {d1} and {d2}"));
            }
            DataValidation::DateEqualTo(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date equal to {d}"));
            }
            DataValidation::DateIsValid => unimplemented!(),
            DataValidation::DateNotBetween(d1, d2) => {
                validation
                    .set_formula1(d1.to_string())
                    .set_formula2(d2.to_string())
                    .set_operator(u::DataValidationOperatorValues::NotBetween)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date not between {d1} and {d2}"));
            }
            DataValidation::DateOnOrAfter(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThanOrEqual)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date on or after {d}"));
            }
            DataValidation::DateOnOrBefore(d) => {
                validation
                    .set_formula1(d.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThanOrEqual)
                    .set_type(u::DataValidationValues::Date)
                    .set_prompt(format!("Date on or before {d}"));
            }
            DataValidation::NumberBetween(n1, n2) => {
                validation
                    .set_formula1(n1.to_string())
                    .set_formula2(n2.to_string())
                    .set_operator(u::DataValidationOperatorValues::Between)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number between {n1} and {n2}"));
            }
            DataValidation::NumberEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number equal to {n}"));
            }
            DataValidation::NumberGreaterThan(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThan)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number greater than {n}"));
            }
            DataValidation::NumberGreaterThanOrEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::GreaterThanOrEqual)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number greater than or equal to {n}"));
            }
            DataValidation::NumberLessThan(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThan)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number less than {n}"));
            }
            DataValidation::NumberLessThanOrEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::LessThanOrEqual)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number less than or equal to {n}"));
            }
            DataValidation::NumberNotBetween(n1, n2) => {
                validation
                    .set_formula1(n1.to_string())
                    .set_formula2(n2.to_string())
                    .set_operator(u::DataValidationOperatorValues::NotBetween)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number not between {n1} and {n2}"));
            }
            DataValidation::NumberNotEqualTo(n) => {
                validation
                    .set_formula1(n.to_string())
                    .set_operator(u::DataValidationOperatorValues::NotEqual)
                    .set_type(u::DataValidationValues::Decimal)
                    .set_prompt(format!("Number equal to {n}"));
            }
            DataValidation::TextContains(_) => todo!(),
            DataValidation::TextDoesNotContain(_) => todo!(),
            DataValidation::TextEqualTo(_) => todo!(),
            DataValidation::TextIsValidEmail => unimplemented!(),
            DataValidation::TextIsValidUrl => unimplemented!(),
            DataValidation::ValueInList(values) => {
                let list_as_string = values
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                validation
                    .set_formula1(&list_as_string)
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::List)
                    .set_prompt(format!("Number equal to {list_as_string}"));
            }
            DataValidation::ValueInRange(a1) => {
                validation
                    .set_formula1(a1.to_string())
                    .set_operator(u::DataValidationOperatorValues::Equal)
                    .set_type(u::DataValidationValues::List)
                    .set_prompt(format!("Number equal to {a1}"));
            }
        }
        validation
    }
}
