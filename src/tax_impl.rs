use crate::tax::InputData;

struct TaxLevel {
    income: f32,
    tax: f32,
}

const TAX_LEVELS: [TaxLevel; 3] = [
    TaxLevel {
        income: 125_140.,
        tax: 0.45,
    },
    TaxLevel {
        income: 50_271.,
        tax: 0.40,
    },
    TaxLevel {
        income: 12_570.,
        tax: 0.20,
    },
];

const NI_LEVELS: [TaxLevel; 2] = [
    TaxLevel {
        income: 4_189.,
        tax: 0.02,
    },
    TaxLevel {
        income: 1_048.,
        tax: 0.08,
    },
];

pub fn get_total_income(base_salary: f32, data: &InputData) -> f32 {
    base_salary * (1.0 + data.annual_bonus) + data.other_income
}

pub fn get_personal_allowance(total_income: f32) -> f32 {
    assert!(
        total_income >= 0.,
        "Total income can't be negative. Got {}",
        total_income
    );
    match total_income {
        t if (0.0..=100_000.).contains(&t) => 12_570.,
        t if t > 125_140. => 0.,
        _ => 12_570. - (total_income - 100_000.) / 2.,
    }
}

pub fn get_tax_value(
    total_income: f32,
    base_salary: f32,
    personal_allowance: f32,
    data: &InputData,
) -> f32 {
    let mut tax_value = base_salary * data.pension_contribution;
    let mut income_to_tax = total_income;
    for TaxLevel { income, tax } in TAX_LEVELS {
        if total_income > income {
            tax_value += (income_to_tax - income) * tax;
            income_to_tax = income;
        }
    }
    tax_value +=
        (TAX_LEVELS.last().unwrap().income - personal_allowance) * TAX_LEVELS.last().unwrap().tax;
    tax_value
}

pub fn get_national_insurance(total_income: f32) -> f32 {
    let mut monthly_income = total_income / 12.;
    let monthly_contribution: f32 = NI_LEVELS
        .iter()
        .map(|l| {
            let taxed_value = (monthly_income - l.income).max(0.) * l.tax;
            monthly_income = monthly_income.min(l.income);
            taxed_value
        })
        .sum();
    monthly_contribution * 12.
}

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    fn expect_near(a: f32, b: f32) {
        const PRECISION: f32 = 1e-3;
        assert!((a - b).abs() < PRECISION, "{} is different from {}", a, b);
    }

    struct TestCase {
        base_salary: f32,
        data: InputData,
        expected_result: f32,
    }

    #[test]
    fn test_get_total_income() {
        for case in [
            TestCase {
                base_salary: 1_000.,
                data: InputData {
                    annual_bonus: 0.,
                    pension_contribution: 0.,
                    other_income: 0.,
                },
                expected_result: 1_000.,
            },
            TestCase {
                base_salary: 1_000.,
                data: InputData {
                    annual_bonus: 0.,
                    pension_contribution: 0.,
                    other_income: 123.,
                },
                expected_result: 1_123.,
            },
            TestCase {
                base_salary: 1_000.,
                data: InputData {
                    annual_bonus: 0.1,
                    pension_contribution: 0.,
                    other_income: 0.,
                },
                expected_result: 1_100.,
            },
            TestCase {
                base_salary: 1_000.,
                data: InputData {
                    annual_bonus: 0.1,
                    pension_contribution: 987.,
                    other_income: 123.,
                },
                expected_result: 1_223.,
            },
        ] {
            expect_near(
                get_total_income(case.base_salary, &case.data),
                case.expected_result,
            );
        }
    }

    #[test]
    fn test_get_personal_allowance() {
        expect_near(get_personal_allowance(1.), 12_570.);
        expect_near(get_personal_allowance(100_000.), 12_570.);
        expect_near(get_personal_allowance(100_002.), 12_569.);
        expect_near(get_personal_allowance(106_000.), 9_570.);
        expect_near(get_personal_allowance(125_141.), 0.);
    }

    #[test]
    fn test_get_tax_value() {
        let data = InputData {
            pension_contribution: 0.,
            other_income: 0.,
            annual_bonus: 0.,
        };
        for case in [
            TestCase {
                base_salary: 3_000.,
                data: data.clone(),
                expected_result: 0.,
            },
            TestCase {
                base_salary: 22_570.,
                data: data.clone(),
                expected_result: 2_000.,
            },
            TestCase {
                base_salary: 60_271.,
                data: data.clone(),
                expected_result: 11_540.2,
            },
            TestCase {
                base_salary: 106_000.,
                data: data.clone(),
                expected_result: 30_431.8,
            },
            TestCase {
                base_salary: 135_140.,
                data,
                expected_result: 44_501.8,
            },
            TestCase {
                base_salary: 22_570.,
                data: InputData {
                    annual_bonus: 0.,
                    pension_contribution: 0.1,
                    other_income: 0.,
                },
                expected_result: 4_257.,
            },
        ] {
            let total_income = get_total_income(case.base_salary, &case.data);
            let personal_allowance = get_personal_allowance(total_income);
            expect_near(
                get_tax_value(
                    total_income,
                    case.base_salary,
                    personal_allowance,
                    &case.data,
                ),
                case.expected_result,
            )
        }
    }

    #[test]
    fn test_national_insurance() {
        expect_near(get_national_insurance(12_000.), 0.);
        expect_near(get_national_insurance(24_000.), 913.92);
        expect_near(get_national_insurance(72_000.), 3450.);
    }
}
