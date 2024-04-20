<script lang="ts">
  import Chart from 'chart.js/auto';
  import type { manyDatesChart } from '$lib/types';

  let { dataset, labels }: { dataset: manyDatesChart; labels: string[] } = $props();

  let chart: any = undefined;

  $effect(() => {
    const datasets = () => {
      return dataset.map(([label, color, data]) => {
        return {
          backgroundColor: color,
          borderColor: color,
          data,
          fill: false,
          label,
          tension: 0.1
        };
      });
    };

    if (chart != undefined) {
      chart.data = { datasets: datasets(), labels };
      chart.update();
      return;
    }
    chart = new Chart('manyDatesChart', {
      data: { datasets: datasets(), labels },
      type: 'line'
    });
  });
</script>

<div>
  <canvas id="manyDatesChart"></canvas>
</div>
