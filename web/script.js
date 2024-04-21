import init, { BaseSalaryRange, InputData, calculate_for_range } from './wasm/tax_plot.js';
import { Plot } from './plot.js';

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
    plot.updateTheme();
})


const plotStart = document.getElementById('plot_start');
const plotEnd = document.getElementById('plot_end')
function checkStartEnd(start, end) {
    if (start >= end) {
        plotStart.setAttribute('aria-invalid', 'true');
        plotStart.setAttribute('aria-describedby', 'Start must be lower than end');
        plotEnd.setAttribute('aria-invalid', 'true');
        plotEnd.setAttribute('aria-describedby', 'End must be higher than start');
        return false;
    }
    plotStart.removeAttribute('aria-invalid');
    plotEnd.removeAttribute('aria-invalid');
    return true;
}

function getValue(elementId) { return parseFloat(document.getElementById(elementId).value); }

const form = document.getElementById('form');

form.addEventListener('submit', (e) => {
    e.preventDefault();

    const plotStartValue = getValue('plot_start');
    const plotEndValue = getValue('plot_end');
    if (!checkStartEnd(plotStartValue, plotEndValue)) { return; }

    const plotStepValue = getValue('plot_step');

    const baseSalaryRange = BaseSalaryRange.new(plotStartValue, plotEndValue, plotStepValue);

    const annualBonus = getValue('other_income') / 100.;
    const pension = getValue('pension') / 100.;
    const otherIncome = getValue('other_income');

    const inputData = InputData.new(pension, otherIncome, annualBonus);
    const taxData = calculate_for_range(baseSalaryRange, inputData);

    plot.update(taxData);
})


