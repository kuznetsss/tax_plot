import { toString } from "./utils.js";

function isMobile() {
    if ("maxTouchPoints" in navigator) return navigator.maxTouchPoints > 0;
    return false;
}

class Plot {
    static taxData = null;

    constructor() {
        this.chartElement = document.getElementById('chart');
        this.chart = null;
    }

    tooltipData(context) {
        if (context.length == 0) return [];
        const taxData = Plot.taxData.at(context.at(0).dataIndex);
        return [
            'Base salary: ' + toString(taxData.base_salary),
            'Total income: ' + toString(taxData.total_income()),
            'Total deductions: ' + toString(taxData.total_deductions()),
        ];

    }

    updateColors() {
        const plotColor = getComputedStyle(document.body).getPropertyValue('color');

        this.chart.options.plugins.tooltip.bodyColor = plotColor;
        this.chart.options.plugins.tooltip.footerColor = plotColor;

        this.chart.options.scales.x.title.color = plotColor;
        this.chart.options.scales.x.grid.color = plotColor;
        this.chart.options.scales.x.grid.tickColor = plotColor;
        this.chart.options.scales.x.ticks.backDropColor = plotColor;
        this.chart.options.scales.x.ticks.color = plotColor;

        this.chart.options.scales.y.title.color = plotColor;
        this.chart.options.scales.y.grid.color = plotColor;
        this.chart.options.scales.y.grid.tickColor = plotColor;
        this.chart.options.scales.y.ticks.backDropColor = plotColor;
        this.chart.options.scales.y.ticks.color = plotColor;

        this.chart.update();
    }

    createChart() {
        if (this.chart != null) { return; }

        const plotColor = getComputedStyle(document.body).getPropertyValue('color');
        const plotFont = {
            family: getComputedStyle(document.body).getPropertyValue('font-family'),
            size: getComputedStyle(document.body).getPropertyValue('font-size').replace('px', '')
        }

        this.chart = new Chart(this.chartElement, {
            type: 'line',
            options: {
                aspectRatio: isMobile() ? 0.5 : 2,
                interaction: { mode: 'nearest' },
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        bodyColor: plotColor,
                        bodyFont: { ...plotFont, weight: 'bold' },
                        filter: function(tooltipItem) {
                            return tooltipItem.datasetIndex === 1;
                        },
                        footerColor: plotColor,
                        footerFont: { ...plotFont, weight: 'normal' },
                        displayColors: false,
                        callbacks: {
                            title: () => { return '' },
                            beforeFooter: this.tooltipData
                        }
                    }
                },
                scales: {
                    x: {
                        type: 'linear',
                        title: { text: 'Base salary, £', display: true, color: plotColor, font: plotFont },
                        grid: { color: plotColor, tickColor: plotColor },
                        ticks: { backDropColor: plotColor, color: plotColor, font: plotFont }
                    },
                    y: {
                        type: 'linear',
                        title: { text: 'Take home, £', display: true, color: plotColor, font: plotFont },
                        grid: { color: plotColor, tickColor: plotColor },
                        ticks: { backDropColor: plotColor, color: plotColor, font: plotFont }
                    }

                },
            },
        });
    }

    update(taxData, currentPoint) {
        if (this.chart == null) { this.createChart(); }

        Plot.taxData = taxData;

        this.chart.data.labels = taxData.base_salary();
        this.chart.data.datasets = [
            {
                animation: false,
                // label: 'Current base salary',
                data: [{
                    x: currentPoint.base_salary,
                    y: currentPoint.take_home()
                }],
                fill: true,
                borderColor: 'rgb(168, 52, 16)',
                borderWidth: 3,
                pointStyle: 'crossRot',
                radius: 7,
                tension: 0,
            },
            {
                animation: false,
                label: 'Take home',
                data: taxData.take_home(),
                fill: false,
                borderColor: 'rgb(5, 162, 162)',
                tension: 0
            }
        ];

        this.chart.update();
    }
}

export { Plot };
