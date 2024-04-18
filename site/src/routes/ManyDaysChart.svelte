<script lang="ts">
  import Chart from 'chart.js/auto';
  import { randomColor } from '$lib/Utils';
  import type { manyDaysChart } from '$lib/types';

  let { dataset, labels }: { dataset: manyDaysChart; labels: string[] } = $props();

  let chart: any = undefined;

  $effect(() => {
    const datasets = () => {
      return dataset.map(([label, data]) => {
        const color = randomColor();
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
    chart = new Chart('manyDaysChart', {
      type: 'line',
      data: { datasets: datasets(), labels }
    });
  });
</script>

<div>
  <canvas id="manyDaysChart"></canvas>
</div>
