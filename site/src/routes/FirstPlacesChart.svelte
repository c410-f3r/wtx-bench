<script lang="ts">
  import Chart from 'chart.js/auto';
  import { randomColor } from '$lib/Utils';
  import type { firstPlacesChart } from '$lib/types';

  let { dataset }: { dataset: firstPlacesChart } = $props();

  let chart: any = undefined;

  $effect(() => {
    const datasets = () => [
      {
        backgroundColor: [...dataset.values()].map(() => randomColor()),
        borderWidth: 0,
        data: [...dataset.values()]
      }
    ];
    const labels = () => [...dataset.keys()];

    if (chart != undefined) {
      chart.data = { datasets: datasets(), labels: labels() };
      chart.update();
      return;
    }
    chart = new Chart('firstPlacesChart', {
      type: 'pie',
      data: { datasets: datasets(), labels: labels() }
    });
  });
</script>

<canvas id="firstPlacesChart"></canvas>
