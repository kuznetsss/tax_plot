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
        const taxData = Plot.taxData.at(context.at(0).dataIndex);
        return [
            'Base salary: ' + taxData.base_salary.toLocaleString(),
            'Total income: ' + taxData.total_income.toLocaleString(),
            'Tax value: ' + taxData.tax_value.toLocaleString(),
        ];

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
                        title: { text: 'Total income after tax, £', display: true, color: plotColor, font: plotFont },
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
                data: [currentPoint],
                fill: true,
                borderColor: 'rgb(168, 52, 16)',
                borderWidth: 3,
                pointStyle: 'crossRot',
                radius: 7,
                tension: 0,
            },
            {
                animation: false,
                label: 'Income after tax',
                data: taxData.income_after_tax(),
                fill: false,
                borderColor: 'rgb(5, 162, 162)',
                tension: 0
            }
        ];

        this.chart.update();
    }
}

export { Plot };
