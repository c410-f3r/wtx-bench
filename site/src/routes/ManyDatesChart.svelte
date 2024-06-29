<script lang="ts">
  import { Chart } from 'chart.js';
  import type { manyDatesChart } from '$lib/types';

  let { dataset, labels }: { dataset: manyDatesChart; labels: string[] } = $props();

  let chart: Chart<'line', number[], string> | undefined = undefined;

  $effect(() => {
    const datasets = () => {
      return Array.from(dataset.entries()).map(([label, [color, data]]) => {
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
