<script lang="ts">
  import { Chart } from 'chart.js';
  import type { firstPlacesChart } from '$lib/types';

  let { dataset }: { dataset: firstPlacesChart } = $props();

  let chart: Chart<'pie', number[], string> | undefined = undefined;

  $effect(() => {
    const datasets = () => [
      {
        backgroundColor: Array.from(dataset.values()).map((elem) => elem[0]),
        borderWidth: 0,
        data: Array.from(dataset.values()).map((elem) => elem[1])
      }
    ];
    const labels = () => [...dataset.keys()];

    if (chart != undefined) {
      chart.data = { datasets: datasets(), labels: labels() };
      chart.update();
      return;
    }
    chart = new Chart('firstPlacesChart', {
      data: { datasets: datasets(), labels: labels() },
      type: 'pie'
    });
  });
</script>

<canvas id="firstPlacesChart"></canvas>
