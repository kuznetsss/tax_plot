use std::iter::Iterator;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub struct InputData {
    /// part of base salary goes to pension
    pub pension_contribution: f32,
    /// absolute value of other income
    pub other_income: f32,
    /// part of base salary
    pub annual_bonus: f32,
}

#[wasm_bindgen]
impl InputData {
    pub fn new(pension_contribution: f32, other_income: f32, annual_bonus: f32) -> Self {
        InputData {
            pension_contribution, other_income, annual_bonus
        }
    }
}


#[wasm_bindgen]
pub struct TaxData {
    pub base_salary: f32,
    pub total_income: f32,
    pub tax_value: f32,
}

#[wasm_bindgen]
fn calculate(base_salary: f32, data: &InputData) -> TaxData {
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
        base_salary,
        total_income,
        tax_value,
    }
}

#[wasm_bindgen]
pub struct BaseSalaryRange {
    start: f32,
    end: f32,
    step: f32,
    current: usize,
}

#[wasm_bindgen]
impl BaseSalaryRange {
    pub fn new(start: f32, end: f32, step: f32) -> Self {
        BaseSalaryRange {
            start,
            end,
            step,
            current: 0,
        }
    }
}

impl Iterator for BaseSalaryRange {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.start + self.step * (self.current as f32);
        if value < self.end {
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

#[wasm_bindgen]
pub struct OutputData {
    data: Vec<TaxData>,
}

#[wasm_bindgen]
impl OutputData {
    fn extract_data<F>(&self, extractor: F) -> js_sys::Float32Array
    where
        F: Fn(&TaxData) -> f32,
    {
        let js_data = js_sys::Float32Array::new_with_length(self.data.len() as u32);
        self.data.iter().enumerate().for_each(|(i, d)| {
            js_data.set_index(i as u32, extractor(d));
        });
        js_data
    }

    pub fn base_salary(&self) -> js_sys::Float32Array {
        self.extract_data(|d| d.base_salary)
    }

    pub fn total_income(&self) -> js_sys::Float32Array {
        self.extract_data(|d| d.total_income)
    }

    pub fn tax_value(&self) -> js_sys::Float32Array {
        self.extract_data(|d| d.tax_value)
    }

    pub fn income_after_tax(&self) -> js_sys::Float32Array {
        self.extract_data(|d| { d.total_income - d.tax_value })
    }
}

#[wasm_bindgen]
pub fn calculate_for_range(base_salary_range: BaseSalaryRange, data: &InputData) -> OutputData {
    let data: Vec<_> = base_salary_range.map(|s| calculate(s, data)).collect();
    OutputData{ data }
}

fn get_personal_allowance(total_income: f32) -> f32 {
    assert!(
        total_income >= 0.,
        "Total income can't be negative. Got {}", total_income
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
            let tax_data = calculate(base_salary, &data);
            expect_near(tax_data.total_income, base_salary);
            expect_near(tax_data.tax_value, expected_tax_value);
            expect_near(tax_data.base_salary, base_salary);
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
        let tax_data = calculate(base_salary, &data);
        expect_near(tax_data.total_income, base_salary);
        expect_near(tax_data.tax_value, expected_tax_value);
        expect_near(tax_data.base_salary, base_salary);
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
        let tax_data = calculate(base_salary, &data);
        expect_near(tax_data.total_income, base_salary + data.other_income);
        expect_near(tax_data.tax_value, expected_tax_value);
        expect_near(tax_data.base_salary, base_salary);
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
        let tax_data = calculate(base_salary, &data);
        expect_near(tax_data.total_income, expected_total_income);
        expect_near(tax_data.tax_value, expected_tax_value);
        expect_near(tax_data.base_salary, base_salary);
    }
}
