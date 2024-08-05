<script lang="ts">
  import { dateAndTime, firstDateOfLastDays, environmentStateParams } from '$lib/Utils';
  import FirstPlacesChart from './FirstPlacesChart.svelte';
  import Header from './Header.svelte';
  import ManyDatesChart from './ManyDatesChart.svelte';
  import {
    ArcElement,
    CategoryScale,
    Chart,
    Legend,
    LinearScale,
    LineController,
    LineElement,
    PieController,
    PointElement,
    Tooltip
  } from 'chart.js';
  import type { PageData } from './$types';

  Chart.register(
    ArcElement,
    CategoryScale,
    Legend,
    LinearScale,
    LineController,
    LineElement,
    PieController,
    PointElement,
    Tooltip
  );

  let { data }: { data: PageData } = $props();

  const firstEnvironment = data.csv.results.keys().next().value;
  const firstParams = environmentStateParams(data.csv, firstEnvironment);

  let environment = $state(firstEnvironment);
  let implementation = $state('');
  let lastDays = $state(firstParams.lastDays);
  let protocol = $state(firstParams.protocol);
  let test = $state('');

  let chartsData = $derived.by(() => {
    return data.csv.chartsData(environment, dates, protocol, implementation, test);
  });
  let dates = $derived.by(() => {
    return [...data.csv.allDates(environment, firstDateOfLastDays(lastDays))].reverse();
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
  let scoresTitle = $derived.by(() => {
    if (implementation === '' && test === '') {
      return 'Completion time in milliseconds (Geometric mean of all tests)';
    } else {
      return 'Completion time in milliseconds';
    }
  });
</script>

<svelte:head>
  <title>WEB Benchmarks</title>
  <meta
    name="description"
    content="Different benchmarks of different WEB technologies measured mainly to evaluate the performance of the 'wtx' project"
  />
</svelte:head>

<Header
  csv={data.csv}
  {dates}
  bind:environment
  bind:implementation
  bind:lastDays
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
