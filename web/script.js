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
const plotStartText = document.getElementById('plot_start_text');

plotStart.addEventListener('input', (e) => {
    e.preventDefault();
    plotStartText.innerText = plotStart.value;
    plotEnd.setAttribute('min', plotStart.value);
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

function updatePlot(e) {
    e.preventDefault();

    const plotStartValue = parseInt(plotStart.value);
    const plotEndValue = parseInt(plotEnd.value);
    if (!checkStartEnd(plotStartValue, plotEndValue)) { return; }

    const plotStepValue = parseInt(plotStep.value);

    const baseSalaryRange = BaseSalaryRange.new(plotStartValue, plotEndValue, plotStepValue);

    const annualBonus = getValue('other_income') / 100.;
    const pension = getValue('pension') / 100.;
    const otherIncome = getValue('other_income');

    const inputData = InputData.new(pension, otherIncome, annualBonus);
    const taxData = calculate_for_range(baseSalaryRange, inputData);

    plot.update(taxData);
    document.getElementById('plot_settings').removeAttribute('hidden')
    plotStart.value = 0
    plotEnd.value = 200000
    plotStep.value = 500
}

form.addEventListener('submit', (e) => { updatePlot(e) })
plotStart.addEventListener('change', (e) => { updatePlot(e) })
plotEnd.addEventListener('change', (e) => { updatePlot(e) })
plotStep.addEventListener('change', (e) => { updatePlot(e) })

