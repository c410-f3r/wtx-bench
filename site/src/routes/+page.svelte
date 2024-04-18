<script lang="ts">
  import Header from './Header.svelte';
  import ManyDaysChart from './ManyDaysChart.svelte';
  import FirstPlacesChart from './FirstPlacesChart.svelte';
  import { short8601String } from '$lib/Utils';

  let { data }: any = $props();

  let days: number | undefined = $state(7);
  let environment: string = $state('fooo');
  let implementation: string = $state('');
  let protocol: string = $state('web-socket');
  let test: string = $state('64 text messages of 2MiB composed by 64 frames');

  let daysDates = $derived.by(() => {
    if (days === undefined) {
      return [];
    }
    let array = Array.from({ length: days }, (_, idx) => {
      let fromDate = new Date();
      fromDate.setDate(fromDate.getDate() - idx);
      return short8601String(fromDate);
    });
    return array.reverse();
  });
  let chartsData = $derived.by(() => {
    return data.csv.chartsData(environment, daysDates, protocol, implementation, test);
  });
</script>

<svelte:head>
  <title>Benchmarks</title>
  <meta
    name="description"
    content="Benchmarks focused on web technologies intended to measure the 'wtx' project"
  />
</svelte:head>

<Header csv={data.csv} bind:days bind:environment bind:implementation bind:protocol bind:test />

<main class="p-3 mt-5">
  <div class="columns is-variable">
    {#if chartsData[0] !== undefined}
      <div class="column is-4">
        <h5 class="title is-5">First places</h5>
        <FirstPlacesChart dataset={chartsData[0]} />
      </div>
    {/if}
    {#if chartsData[1] !== undefined}
      <div class="column">
        <h5 class="title is-5">Scores</h5>
        <ManyDaysChart dataset={chartsData[1]} labels={daysDates} />
      </div>
    {/if}
  </div>
</main>
