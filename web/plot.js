class Plot {
    constructor() {
        this.chartElement = document.getElementById('chart');
        this.chart = null;
    }

    createChart() {
        if (this.chart != null) { return; }

        const plotColor = getComputedStyle(document.body).getPropertyValue('color');

        this.chart = new Chart(this.chartElement, {
            type: 'line',
            options: {
                plugins: {
                    legend: { display: false }
                },
                scales: {
                    x: {
                        type: 'linear',
                        title: { text: 'Base salary, £', display: true, color: plotColor },
                        grid: { color: plotColor, tickColor: plotColor },
                        ticks: { backDropColor: plotColor, color: plotColor }
                    },
                    y: {
                        type: 'linear',
                        title: { text: 'Total income after tax, £', display: true, color: plotColor },
                        grid: { color: plotColor, tickColor: plotColor },
                        ticks: { backDropColor: plotColor, color: plotColor }
                    }

                },
            },
        });
    }

    update(taxData) {
        if (this.chart == null) { this.createChart(); }

        this.chart.data.labels = taxData.base_salary();
        this.chart.data.datasets = [{
            animation: false,
            label: 'Income after tax',
            data: taxData.income_after_tax(),
            fill: false,
            borderColor: 'rgb(75, 192, 192)',
            tension: 0
        }];

        this.chart.update();
    }
}

export { Plot };
