import init, { BaseSalaryRange, InputData, calculate_for_range, calculate } from './wasm/tax_plot.js';
import { Plot } from './plot.js';
import { toString } from './utils.js';

await init();

const html = document.documentElement;

document.addEventListener('DOMContentLoaded', () => {
    html.setAttribute('data-theme', 'dark');
})

const themeSwitcher = document.getElementById('theme_switcher');
function updateTheme() { html.setAttribute('data-theme', themeSwitcher.checked ? 'light' : 'dark'); }

updateTheme();

const plot = new Plot();

themeSwitcher.addEventListener('change', (e) => {
    e.preventDefault();
    updateTheme();
    plot.updateColors();
})


const plotStart = document.getElementById('plot_start');
const plotStartText = document.getElementById('plot_start_text');

plotStart.addEventListener('input', (e) => {
    e.preventDefault();
    plotStartText.innerText = plotStart.value;
    plotEnd.setAttribute('min', parseInt(plotStart.value) + parseInt(plotEnd.step));
})

const plotEnd = document.getElementById('plot_end');
const plotEndText = document.getElementById('plot_end_text');

plotEnd.addEventListener('input', (e) => {
    e.preventDefault();
    plotEndText.innerText = plotEnd.value;
    plotStart.setAttribute('max', plotEnd.value);
})

const plotStep = document.getElementById('plot_step');
const plotStepText = document.getElementById('plot_step_text');

plotStep.addEventListener('input', (e) => {
    e.preventDefault()
    plotStepText.innerText = plotStep.value;
})

function checkStartEnd(start, end) {
    if (start >= end) {
        plotStart.setAttribute('aria-invalid', 'true');
        plotStart.setAttribute('aria-describedby', 'Start must be lower than end');
        plotEnd.setAttribute('aria-invalid', 'true');
        plotEnd.setAttribute('aria-describedby', 'End must be higher than start');
        console.log('start >= end: ', start, end)
        return false;
    }
    plotStart.removeAttribute('aria-invalid');
    plotEnd.removeAttribute('aria-invalid');
    return true;
}

function getValue(elementId) { return parseFloat(document.getElementById(elementId).value); }

const form = document.getElementById('form');

const plotSettings = document.getElementById('plot_settings');

function updateTable(taxData) {
    document.getElementById('table_base_salary').innerText = '+ £ ' + toString(taxData.base_salary);
    document.getElementById('table_annual_bonus').innerText = '+ £ ' + toString(taxData.annual_bonus);
    document.getElementById('table_total_income').innerText = '+ £ ' + toString(taxData.total_income());
    document.getElementById('table_other_income').innerText = '+ £ ' + toString(taxData.other_income);
    document.getElementById('table_tax').innerText = '- £ ' + toString(taxData.tax_value);
    document.getElementById('table_total_deductions').innerText = '- £ ' + toString(taxData.total_deductions());
    document.getElementById('table_national_insurance').innerText = '- £ ' + toString(taxData.national_insurance);
    document.getElementById('table_pension_contribution').innerText = '- £ ' + toString(taxData.pension_contribution);
    document.getElementById('table_pension_tax_relief').innerText = '+ £ ' + toString(taxData.pension_tax_relief);
    document.getElementById('table_take_home').innerText = '£ ' + toString(taxData.take_home());
}

function update(e) {
    e.preventDefault()

    const plotStartValue = parseInt(plotStart.value);
    const plotEndValue = parseInt(plotEnd.value);
    if (!checkStartEnd(plotStartValue, plotEndValue)) { return; }

    const plotStepValue = parseInt(plotStep.value);

    const baseSalaryRange = BaseSalaryRange.new(plotStartValue, plotEndValue, plotStepValue);

    const annualBonus = getValue('annual_bonus') / 100.;
    const pension = getValue('pension') / 100.;
    const otherIncome = getValue('other_income');

    const inputData = InputData.new(pension, otherIncome, annualBonus);
    const taxData = calculate_for_range(baseSalaryRange, inputData);

    const baseSalary = getValue('base_salary');
    const taxDataAtBaseSalary = calculate(baseSalary, inputData);
    plot.update(taxData, taxDataAtBaseSalary);
    updateTable(taxDataAtBaseSalary);
}

form.addEventListener('submit', (e) => { update(e) })
plotStart.addEventListener('change', (e) => { update(e) })
plotEnd.addEventListener('change', (e) => { update(e) })
plotStep.addEventListener('change', (e) => { update(e) })

form.requestSubmit()
