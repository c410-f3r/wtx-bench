<script lang="ts">
  import { dateAndTime } from '$lib/Utils';
  import Csv from '$lib/Csv';
  import FirstPlacesChart from './FirstPlacesChart.svelte';
  import Header from './Header.svelte';
  import ManyDatesChart from './ManyDatesChart.svelte';

  let { data }: { data: { csv: Csv } } = $props();

  let environment: string = $state(data.csv.results.keys().next().value);
  let implementation: string = $state('');
  let lastDays: number = $state(1);
  let protocol: string = $state('');
  let test: string = $state('');

  let chartsData = $derived.by(() => {
    return data.csv.chartsData(environment, dates, protocol, implementation, test);
  });
  let dates = $derived.by(() => {
    return [...data.csv.allDates(environment)].reverse();
  });
  let datesStrings = $derived.by(() => {
    return dates.map((date) => dateAndTime(new Date(date)));
  });
  let firstPlacesTitle = $derived.by(() => {
    if (implementation === '' && test === '') {
      return 'First places (All tests)';
    } else {
      return 'First places';
    }
  });
  let maxDays = $derived(data.csv.oldestDayCountFromEnvironment(environment));
  let scoresTitle = $derived.by(() => {
    if (implementation === '' && test === '') {
      return 'Scores (Geometric mean of all tests)';
    } else {
      return 'Scores';
    }
  });

  $effect(() => {
    lastDays = Math.min(data.csv.oldestDayCountFromEnvironment(environment), 7);
    protocol = data.csv.results.get(environment)!.values().next().value.keys().next().value;
  });
</script>

<svelte:head>
  <title>Benchmarks</title>
  <meta
    name="description"
    content="Benchmarks focused on web technologies intended to measure the 'wtx' project"
  />
</svelte:head>

<Header
  csv={data.csv}
  {dates}
  bind:environment
  bind:implementation
  bind:lastDays
  {maxDays}
  bind:protocol
  bind:test
/>

<main class="my-5 p-3">
  <div class="columns is-variable">
    {#if chartsData[0] !== undefined}
      <div class="column is-4">
        <h5 class="title is-5">{firstPlacesTitle}</h5>
        <FirstPlacesChart dataset={chartsData[0]} />
      </div>
    {/if}
    <div class="column">
      <h5 class="title is-5">{scoresTitle}</h5>
      <ManyDatesChart dataset={chartsData[1]} labels={datesStrings} />
    </div>
  </div>
</main>

<footer class="footer has-text-centered">
  <h6 class="is-size-6 mb-2">
    <strong>Contribute</strong> on GitHub
  </h6>

  <div>
    <iframe
      frameborder="0"
      height="30px"
      title="wtx-bench"
      scrolling="0"
      src="https://ghbtns.com/github-btn.html?user=c410-f3r&repo=wtx-bench&type=star&count=true&size=large"
      width="120px"
    ></iframe>
  </div>
</footer>
