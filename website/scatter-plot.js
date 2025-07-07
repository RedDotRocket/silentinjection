// Interactive Scatter Plot for HF Scanner Vulnerability Distribution
// Uses Chart.js for visualization

class VulnerabilityScatterPlot {
    constructor(containerId, dataUrl) {
        this.containerId = containerId;
        this.dataUrl = dataUrl;
        this.chart = null;
        this.data = null;
        this.init();
    }

    async init() {
        try {
            await this.loadData();
            this.createChart();
            this.setupInteractivity();
        } catch (error) {
            console.error('Error initializing scatter plot:', error);
            this.showError();
        }
    }

    async loadData() {
        const response = await fetch(this.dataUrl);
        if (!response.ok) {
            throw new Error(`Failed to load data: ${response.status}`);
        }

        const csvText = await response.text();
        this.data = this.parseCSV(csvText);
        this.processData();
    }

    parseCSV(csvText) {
        const lines = csvText.trim().split('\n');
        const headers = lines[0].split(',');
        const data = [];

        for (let i = 1; i < lines.length; i++) {
            const values = lines[i].split(',');
            const row = {};
            headers.forEach((header, index) => {
                row[header.trim()] = values[index] ? values[index].trim() : '';
            });
            data.push(row);
        }

        return data;
    }

    processData() {
        // Group by org and repo, similar to the Python analysis
        const repoData = {};

        this.data.forEach(row => {
            const key = `${row.org}/${row.repo}`;
            if (!repoData[key]) {
                repoData[key] = {
                    org: row.org,
                    repo: row.repo,
                    safe_usages: 0,
                    partial_usages: 0,
                    unsafe_usages: 0,
                    file_count: 0
                };
            }

            repoData[key].safe_usages += parseInt(row.safe_usages) || 0;
            repoData[key].partial_usages += parseInt(row.partial_usages) || 0;
            repoData[key].unsafe_usages += parseInt(row.unsafe_usages) || 0;
            repoData[key].file_count += 1;
        });

        // Calculate safety metrics
        this.processedData = Object.values(repoData).map(repo => {
            const totalUsages = repo.safe_usages + repo.partial_usages + repo.unsafe_usages;
            const safetyRatio = totalUsages > 0 ? repo.safe_usages / totalUsages : 0;

            return {
                ...repo,
                total_usages: totalUsages,
                safety_ratio: safetyRatio,
                label: `${repo.org}/${repo.repo}`
            };
        }).filter(repo => repo.total_usages >= 1); // Filter out repos with no usage
    }

    createChart() {
        const ctx = document.getElementById(this.containerId);
        if (!ctx) {
            throw new Error(`Container with id '${this.containerId}' not found`);
        }

        // Remove loading state
        const chartWrapper = ctx.closest('.chart-wrapper');
        if (chartWrapper) {
            chartWrapper.classList.add('chart-loaded');
        }

        // Prepare data for Chart.js
        const chartData = {
            datasets: [{
                label: 'Repositories',
                data: this.processedData.map(repo => ({
                    x: repo.total_usages,
                    y: repo.safety_ratio,
                    org: repo.org,
                    repo: repo.repo,
                    safe: repo.safe_usages,
                    partial: repo.partial_usages,
                    unsafe: repo.unsafe_usages,
                    total: repo.total_usages,
                    files: repo.file_count
                })),
                backgroundColor: this.getPointColors(),
                borderColor: 'rgba(0, 0, 0, 0.3)',
                borderWidth: 1,
                pointRadius: this.getPointSizes(),
                pointHoverRadius: 8,
                pointHoverBorderWidth: 2,
                pointHoverBorderColor: '#000'
            }]
        };

        // Configuration
        const config = {
            type: 'scatter',
            data: chartData,
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    title: {
                        display: true,
                        text: 'Safety Ratio x Total Usage',
                        font: {
                            size: 16,
                            weight: 'bold'
                        }
                    },
                    tooltip: {
                        mode: 'nearest',
                        intersect: false,
                        position: 'nearest',
                        callbacks: {
                            title: function(context) {
                                const data = context[0].raw;
                                return `${data.org}/${data.repo}`;
                            },
                            label: function(context) {
                                const data = context.raw;
                                const safetyPercent = (data.y * 100).toFixed(1);
                                return [
                                    `Safety Ratio: ${safetyPercent}%`,
                                    `Total Usage: ${data.total}`,
                                    `Safe: ${data.safe}`,
                                    `Partial: ${data.partial}`,
                                    `Unsafe: ${data.unsafe}`,
                                    `Files: ${data.files}`
                                ];
                            }
                        }
                    },
                    legend: {
                        display: false
                    }
                },
                scales: {
                    x: {
                        type: 'linear',
                        position: 'bottom',
                        title: {
                            display: true,
                            text: 'Total Hugging Face Usages',
                            font: {
                                size: 12,
                                weight: 'bold'
                            }
                        },
                        grid: {
                            color: 'rgba(0, 0, 0, 0.1)'
                        }
                    },
                    y: {
                        type: 'linear',
                        position: 'left',
                        title: {
                            display: true,
                            text: 'Safety Ratio (Safe/Total)',
                            font: {
                                size: 12,
                                weight: 'bold'
                            }
                        },
                        min: 0,
                        max: 1,
                        ticks: {
                            callback: function(value) {
                                return (value * 100).toFixed(0) + '%';
                            }
                        },
                        grid: {
                            color: 'rgba(0, 0, 0, 0.1)'
                        }
                    }
                },
                interaction: {
                    intersect: false,
                    mode: 'nearest',
                    axis: 'xy'
                },
                animation: {
                    duration: 1000
                },
                clip: false
            }
        };

        this.chart = new Chart(ctx, config);
        this.addReferenceLines();
        this.addQuadrantLabels();
    }

    getPointColors() {
        return this.processedData.map(repo => {
            const safetyRatio = repo.safety_ratio;

            // Use website's color palette: red -> amber -> green
            let red, green, blue;

            if (safetyRatio <= 0.5) {
                // Red to Amber (0.0 to 0.5)
                // Red-600: #dc2626, Amber-500: #f59e0b
                const t = safetyRatio * 2; // 0 to 1
                red = Math.round(220 - (t * 25)); // 220 -> 195
                green = Math.round(38 + (t * 97)); // 38 -> 135
                blue = Math.round(38 + (t * 11)); // 38 -> 49
            } else {
                // Amber to Green (0.5 to 1.0)
                // Amber-500: #f59e0b, Green-600: #059669
                const t = (safetyRatio - 0.5) * 2; // 0 to 1
                red = Math.round(195 - (t * 154)); // 195 -> 41
                green = Math.round(135 + (t * 30)); // 135 -> 165
                blue = Math.round(49 + (t * 20)); // 49 -> 69
            }

            return `rgba(${red}, ${green}, ${blue}, 0.7)`;
        });
    }

    getPointSizes() {
        return this.processedData.map(repo => {
            const total = repo.total_usages;
            if (total > 1000) return 8;
            if (total > 500) return 6;
            if (total > 100) return 5;
            if (total > 50) return 4;
            return 3;
        });
    }

    addReferenceLines() {
        // Add horizontal reference lines
        const yAxis = this.chart.scales.y;
        const xAxis = this.chart.scales.x;

        // 50% safety threshold
        this.chart.ctx.save();
        this.chart.ctx.strokeStyle = 'rgba(34, 197, 94, 0.7)';
        this.chart.ctx.setLineDash([5, 5]);
        this.chart.ctx.beginPath();
        this.chart.ctx.moveTo(xAxis.left, yAxis.getPixelForValue(0.5));
        this.chart.ctx.lineTo(xAxis.right, yAxis.getPixelForValue(0.5));
        this.chart.ctx.stroke();
        this.chart.ctx.restore();

        // 25% safety threshold
        this.chart.ctx.save();
        this.chart.ctx.strokeStyle = 'rgba(251, 191, 36, 0.7)';
        this.chart.ctx.setLineDash([5, 5]);
        this.chart.ctx.beginPath();
        this.chart.ctx.moveTo(xAxis.left, yAxis.getPixelForValue(0.25));
        this.chart.ctx.lineTo(xAxis.right, yAxis.getPixelForValue(0.25));
        this.chart.ctx.stroke();
        this.chart.ctx.restore();

        // 10% safety threshold
        this.chart.ctx.save();
        this.chart.ctx.strokeStyle = 'rgba(239, 68, 68, 0.7)';
        this.chart.ctx.setLineDash([5, 5]);
        this.chart.ctx.beginPath();
        this.chart.ctx.moveTo(xAxis.left, yAxis.getPixelForValue(0.1));
        this.chart.ctx.lineTo(xAxis.right, yAxis.getPixelForValue(0.1));
        this.chart.ctx.stroke();
        this.chart.ctx.restore();
    }

    addQuadrantLabels() {
        const canvas = this.chart.canvas;
        const ctx = canvas.getContext('2d');
        const xAxis = this.chart.scales.x;
        const yAxis = this.chart.scales.y;

        const maxX = Math.max(...this.processedData.map(d => d.total_usages));
        const midX = maxX / 2;
        const midY = 0.5;

        // Quadrant labels
        const labels = [
            { text: 'High Usage\nHigh Safety', x: 0.75, y: 0.75, color: 'rgba(34, 197, 94, 0.2)' },
            { text: 'High Usage\nLow Safety', x: 0.75, y: 0.25, color: 'rgba(239, 68, 68, 0.2)' },
            { text: 'Low Usage\nHigh Safety', x: 0.25, y: 0.75, color: 'rgba(59, 130, 246, 0.2)' },
            { text: 'Low Usage\nLow Safety', x: 0.25, y: 0.25, color: 'rgba(251, 191, 36, 0.2)' }
        ];

        labels.forEach(label => {
            const x = xAxis.left + (xAxis.right - xAxis.left) * label.x;
            const y = yAxis.top + (yAxis.bottom - yAxis.top) * (1 - label.y);

            ctx.save();
            ctx.fillStyle = label.color;
            ctx.fillRect(x - 60, y - 30, 120, 60);
            ctx.fillStyle = '#000';
            ctx.font = '12px Arial';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText(label.text.split('\n')[0], x, y - 10);
            ctx.fillText(label.text.split('\n')[1], x, y + 10);
            ctx.restore();
        });
    }

    setupInteractivity() {
        // Add click handler for detailed view
        this.chart.canvas.addEventListener('click', (event) => {
            const points = this.chart.getElementsAtEventForMode(event, 'nearest', { intersect: true }, true);
            if (points.length > 0) {
                const data = points[0].raw;
                this.showRepositoryDetails(data);
            }
        });

        // Add keyboard navigation
        document.addEventListener('keydown', (event) => {
            if (event.key === 'Escape') {
                this.hideRepositoryDetails();
            }
        });
    }

    showRepositoryDetails(data) {
        const modal = document.createElement('div');
        modal.className = 'repo-modal';
        modal.innerHTML = `
            <div class="modal-content">
                <span class="close">&times;</span>
                <h3>${data.org}/${data.repo}</h3>
                <div class="repo-stats">
                    <div class="stat">
                        <span class="label">Safety Ratio:</span>
                        <span class="value ${data.y >= 0.5 ? 'safe' : data.y >= 0.25 ? 'warning' : 'danger'}">
                            ${(data.y * 100).toFixed(1)}%
                        </span>
                    </div>
                    <div class="stat">
                        <span class="label">Total Usage:</span>
                        <span class="value">${data.total}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Safe Calls:</span>
                        <span class="value safe">${data.safe}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Partial Calls:</span>
                        <span class="value warning">${data.partial}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Unsafe Calls:</span>
                        <span class="value danger">${data.unsafe}</span>
                    </div>
                    <div class="stat">
                        <span class="label">Files Analyzed:</span>
                        <span class="value">${data.files}</span>
                    </div>
                </div>
            </div>
        `;

        // Add styles
        const style = document.createElement('style');
        style.textContent = `
            .repo-modal {
                position: fixed;
                z-index: 1000;
                left: 0;
                top: 0;
                width: 100%;
                height: 100%;
                background-color: rgba(0,0,0,0.5);
                display: flex;
                align-items: center;
                justify-content: center;
            }
            .modal-content {
                background-color: white;
                padding: 20px;
                border-radius: 8px;
                max-width: 400px;
                width: 90%;
                position: relative;
            }
            .close {
                position: absolute;
                right: 15px;
                top: 10px;
                font-size: 24px;
                cursor: pointer;
                color: #666;
            }
            .close:hover {
                color: #000;
            }
            .repo-stats {
                margin-top: 15px;
            }
            .stat {
                display: flex;
                justify-content: space-between;
                margin: 8px 0;
                padding: 5px 0;
                border-bottom: 1px solid #eee;
            }
            .stat:last-child {
                border-bottom: none;
            }
            .label {
                font-weight: 500;
            }
            .value {
                font-weight: bold;
            }
            .value.safe { color: #22c55e; }
            .value.warning { color: #fbbf24; }
            .value.danger { color: #ef4444; }
        `;

        document.head.appendChild(style);
        document.body.appendChild(modal);

        // Close functionality
        const closeBtn = modal.querySelector('.close');
        const closeModal = () => {
            document.body.removeChild(modal);
            document.head.removeChild(style);
        };

        closeBtn.onclick = closeModal;
        modal.onclick = (event) => {
            if (event.target === modal) closeModal();
        };
    }

    hideRepositoryDetails() {
        const modal = document.querySelector('.repo-modal');
        if (modal) {
            document.body.removeChild(modal);
        }
    }

    showError() {
        const container = document.getElementById(this.containerId);
        if (container) {
            container.innerHTML = `
                <div style="text-align: center; padding: 40px; color: #666;">
                    <h3>Error Loading Data</h3>
                    <p>Unable to load vulnerability distribution data.</p>
                    <p>Please check the data file path and try again.</p>
                </div>
            `;
        }
    }

    // Public method to update chart
    updateChart() {
        if (this.chart) {
            this.chart.update();
        }
    }

    // Public method to destroy chart
    destroy() {
        if (this.chart) {
            this.chart.destroy();
        }
    }
}

// Initialize the chart when the page loads
document.addEventListener('DOMContentLoaded', function() {
    const scatterPlot = new VulnerabilityScatterPlot(
        'vulnerability-scatter-plot',
        './results/hfscanner.csv'
    );
});
