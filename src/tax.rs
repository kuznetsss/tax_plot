use crate::tax_impl;
use std::iter::Iterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
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
            pension_contribution,
            other_income,
            annual_bonus,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TaxData {
    pub base_salary: f32,
    pub annual_bonus: f32,
    pub other_income: f32,
    pub tax_value: f32,
    pub national_insurance: f32,
    pub pension_contribution: f32,
    pub pension_tax_relief: f32,
}

#[wasm_bindgen]
impl TaxData {
    pub fn total_income(&self) -> f32 {
        self.base_salary + self.annual_bonus + self.other_income
    }

    pub fn take_home(&self) -> f32 {
        self.total_income() - self.tax_value - self.national_insurance - self.pension_contribution
            + self.pension_tax_relief
    }
}

#[wasm_bindgen]
pub fn calculate(base_salary: f32, data: &InputData) -> TaxData {
    let total_income = tax_impl::get_total_income(base_salary, data);
    let personal_allowance = tax_impl::get_personal_allowance(total_income);
    let tax_value = tax_impl::get_tax_value(total_income, personal_allowance);
    let national_insurance = tax_impl::get_national_insurance(total_income);
    let pension_contribution = tax_impl::get_pension_contribution(base_salary, data);
    let pension_tax_relief = tax_impl::get_pension_tax_relief(total_income, pension_contribution);
    TaxData {
        base_salary,
        annual_bonus: tax_impl::get_annual_bonus(base_salary, data),
        other_income: data.other_income,
        tax_value,
        national_insurance,
        pension_contribution,
        pension_tax_relief,
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
        self.extract_data(|d| d.total_income())
    }

    pub fn tax_value(&self) -> js_sys::Float32Array {
        self.extract_data(|d| d.tax_value)
    }

    pub fn income_after_tax(&self) -> js_sys::Float32Array {
        self.extract_data(|d| d.take_home())
    }

    pub fn at(&self, index: usize) -> TaxData {
        assert!(
            index < self.data.len(),
            "Index must be smaller than data size"
        );
        self.data[index].clone()
    }
}

#[wasm_bindgen]
pub fn calculate_for_range(base_salary_range: BaseSalaryRange, data: &InputData) -> OutputData {
    let data: Vec<_> = base_salary_range.map(|s| calculate(s, data)).collect();
    OutputData { data }
}
