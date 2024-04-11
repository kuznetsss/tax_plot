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

pub struct InputData {
    /// part of base salary goes to pension
    pub pension_contribution: f32,
    /// absolute value of other income
    pub other_income: f32,
    /// part of base salary
    pub annual_bonus: f32,
}

pub struct TaxData {
    pub total_income: f32,
    pub tax_value: f32,
}

pub fn calculate(base_salary: f32, data: &InputData) -> TaxData {
    let total_income = base_salary * (1.0 + data.annual_bonus) + data.other_income;
    let mut tax_value = base_salary * data.pension_contribution;
    let mut income_to_tax = total_income;
    for TaxLevel { income, tax } in TAX_LEVELS {
        if total_income > income {
            tax_value += (income_to_tax - income) * tax;
            income_to_tax = income;
        }
    }
    let personal_allowance = get_personal_allowance(total_income);
    tax_value +=
        (TAX_LEVELS.last().unwrap().income - personal_allowance) * TAX_LEVELS.last().unwrap().tax;
    TaxData {
        total_income,
        tax_value,
    }
}

fn get_personal_allowance(total_income: f32) -> f32 {
    assert!(
        total_income >= 0.,
        "Total income can't be negative. Got {total_income}"
    );
    match total_income {
        t if (0.0..=100_000.).contains(&t) => 12_570.,
        t if t > 125_140. => 0.,
        _ => 12_570. - (total_income - 100_000.) / 2.,
    }
}

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    fn expect_near(a: f32, b: f32) {
        const PRECISION: f32 = 1e-3;
        assert!((a - b).abs() < PRECISION, "{a} is different from {b}");
    }

    #[test]
    fn test_get_personal_allowance() {
        expect_near(get_personal_allowance(1.), 12_570.);
        expect_near(get_personal_allowance(100_000.), 12_570.);
        expect_near(get_personal_allowance(100_002.), 12_569.);
        expect_near(get_personal_allowance(125_141.), 0.);
    }

    #[test]
    fn test_calculate_only_salary() {
        let data = InputData {
            pension_contribution: 0.,
            other_income: 0.,
            annual_bonus: 0.,
        };
        for (base_salary, expected_tax_value) in [
            (3_000., 0.),
            (22_570., 2_000.),
            (60_271., 11_540.2),
            (106_000., 30_431.8),
            (135_140., 44_501.8),
        ] {
            let TaxData {
                total_income,
                tax_value,
            } = calculate(base_salary, &data);
            expect_near(total_income, base_salary);
            expect_near(tax_value, expected_tax_value);
        }
    }

    #[test]
    fn test_calculate_pension_contribution() {
        let data = InputData {
            pension_contribution: 0.1,
            other_income: 0.,
            annual_bonus: 0.,
        };
        let base_salary = 22_570.;
        let expected_tax_value = 4_257.;
        let TaxData {
            total_income,
            tax_value,
        } = calculate(base_salary, &data);
        expect_near(total_income, base_salary);
        expect_near(tax_value, expected_tax_value);
    }

    #[test]
    fn test_calculate_other_income() {
        let data = InputData {
            pension_contribution: 0.0,
            other_income: 1_000.,
            annual_bonus: 0.,
        };
        let base_salary = 22_570.;
        let expected_tax_value = 2_200.;
        let TaxData {
            total_income,
            tax_value,
        } = calculate(base_salary, &data);
        expect_near(total_income, base_salary + data.other_income);
        expect_near(tax_value, expected_tax_value);
    }

    #[test]
    fn test_calculate_annual_bonus() {
        let data = InputData {
            pension_contribution: 0.0,
            other_income: 0.,
            annual_bonus: 0.1,
        };
        let base_salary = 22_570.;
        let expected_tax_value = 2_451.4;
        let expected_total_income = 24_827.;
        let TaxData {
            total_income,
            tax_value,
        } = calculate(base_salary, &data);
        expect_near(total_income, expected_total_income);
        expect_near(tax_value, expected_tax_value);
    }
}