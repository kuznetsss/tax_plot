struct TaxLevel {
    income: u64,
    tax: f32,
}

const TAX_LEVELS: [TaxLevel; 3] = [
    TaxLevel {
        income: 125_140,
        tax: 0.45,
    },
    TaxLevel {
        income: 50_271,
        tax: 0.40,
    },
    TaxLevel {
        income: 12_570,
        tax: 0.20,
    },
];

pub struct InputData {
    /// part of base salary goes to pension
    pub pension_contribution: f32,
    /// absolute value of other income
    pub other_income: u64,
    /// part of base salary
    pub annual_bonus: f32,
}

pub struct TaxData {
    pub total_income: u64,
    pub tax_value: u64
}

pub fn calculate(base_salary: u64, data: InputData) -> TaxData {
    let total_income = (base_salary as f32 * (1.0 + data.annual_bonus)) as u64 + data.other_income;
    let mut tax_value = 0_u64;
    for TaxLevel{income, tax} in TAX_LEVELS {
        if total_income > income {
            tax_value += ((total_income - income) as f32 * tax) as u64;
        }
    }
    let personal_allowance = get_personal_allowance(total_income);
    tax_value += ((TAX_LEVELS.last().unwrap().income - personal_allowance) as f32 * TAX_LEVELS.last().unwrap().tax) as u64;
    TaxData{total_income, tax_value}
}

fn get_personal_allowance(total_income: u64) -> u64 {
    match total_income {
        0..=100_000 => 12_570,
        125_140..=u64::MAX => 0,
        _ => 12_570 - (total_income - 100_000) / 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_personal_allowance() {
        assert_eq!(get_personal_allowance(1), 12_570);
        assert_eq!(get_personal_allowance(100_000), 12_570);
        assert_eq!(get_personal_allowance(100_002), 12_569);
        assert_eq!(get_personal_allowance(125_141), 0);
    }
}
