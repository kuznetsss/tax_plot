class Plot {
    constructor() {
        this.chartElement = document.getElementById('chart')
        this.chart = null
    }

    updateTheme() {
        if (this.chart == null) { return }

        const plotColor = getComputedStyle(document.body).getPropertyValue('color')

        this.chart.options.scales.x.grid.color = plotColor
        this.chart.options.scales.x.grid.tickColor = plotColor
        this.chart.options.scales.x.ticks.backDropColor = plotColor
        this.chart.options.scales.x.ticks.color = plotColor
        this.chart.options.scales.x.title.display = true
        this.chart.options.scales.x.title.color = plotColor

        this.chart.options.scales.y.grid.color = plotColor
        this.chart.options.scales.y.grid.tickColor = plotColor
        this.chart.options.scales.y.ticks.backDropColor = plotColor
        this.chart.options.scales.y.ticks.color = plotColor
        this.chart.options.scales.y.title.display = true
        this.chart.options.scales.y.title.color = plotColor

        this.chart.update()
    }

    update(taxData) {
        if (this.chart != null) { this.chart.destroy() }

        this.chart = new Chart(this.chartElement, {
            type: 'line',
            data: {
                labels: taxData.base_salary(),
                datasets: [{
                    label: 'Income after tax',
                    data: taxData.income_after_tax(),
                    fill: false,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0
                }]
            },
            options: {
                scales: {
                    x: {
                        type: 'linear',
                        title: { text: 'Base salary, £' },
                    },
                    y: {
                        type: 'linear',
                        title: { text: 'Total income after tax, £' },
                    }

                },
                plugins: {
                    legend: { display: false }
                }
            },
        })

        this.updateTheme()
    }
}

export { Plot };
