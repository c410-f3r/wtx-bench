<script lang="ts">
  /* eslint-disable @typescript-eslint/no-explicit-any */

  import { environmentStateParams, protocolState } from '$lib/Utils';

  let {
    csv,
    dates,
    environment = $bindable(),
    implementation = $bindable(),
    lastDays = $bindable(),
    protocol = $bindable(),
    test = $bindable()
  }: any = $props();

  const environmentInput = (elem: any) => {
    const localEnvironment = elem.target.value;
    const params = environmentStateParams(csv, localEnvironment);
    environment = localEnvironment;
    lastDays = params.lastDays;
    maxLastDays = Math.max(maxLastDays, lastDays);
    protocol = params.protocol;
  };

  const lastDaysInput = (elem: any) => {
    lastDays = elem.target.value;
    maxLastDays = Math.max(maxLastDays, lastDays);
    protocol = protocolState();
  };

  let [protocols, implementations, tests] = $derived.by(() => {
    let implementations = new Set();
    let protocols = new Set();
    let tests = new Set();

    for (const [localDate, localProtocols] of csv.results.get(environment)!) {
      if (!dates.includes(localDate)) {
        continue;
      }
      for (const [localProtocol] of localProtocols) {
        protocols.add(localProtocol);
      }
      const localProtocol = localProtocols.get(protocol);
      if (localProtocol === undefined) {
        continue;
      }
      for (const [localImplementation, localTests] of localProtocol) {
        implementations.add(localImplementation);
        for (const [localTest] of localTests.tests) {
          tests.add(localTest);
        }
      }
    }
    return [protocols, implementations, tests];
  });

  let maxLastDays = $state(lastDays);

  $effect(() => {
    const navbarBurgers = Array.prototype.slice.call(
      document.querySelectorAll('.navbar-burger'),
      0
    );
    navbarBurgers.forEach((el) => {
      el.addEventListener('click', () => {
        const target = document.getElementById(el.dataset.target);
        el.classList.toggle('is-active');
        target?.classList.toggle('is-active');
      });
    });
  });
</script>

<header>
  <nav class="navbar" aria-label="main navigation">
    <div class="navbar-brand">
      <a
        aria-expanded="false"
        aria-label="menu"
        class="navbar-burger"
        data-target="wtx-navbar"
        href={'#'}
        role="button"
      >
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
        <span aria-hidden="true"></span>
      </a>
    </div>

    <div class="navbar-menu" id="wtx-navbar">
      <div class="navbar-item">
        <div>
          <label class="label" for="environment">Environment</label>
          <div class="control select">
            <select id="environment" oninput={environmentInput}>
              {#each csv.environments() as environment}
                <option value={environment}>{environment}</option>
              {/each}
            </select>
          </div>
        </div>
      </div>

      <div class="navbar-item">
        <div>
          <label class="label" for="lastDays">Last days</label>
          <div class="control">
            <input
              class="input"
              id="lastDays"
              max={maxLastDays}
              min="1"
              oninput={lastDaysInput}
              style="width:80px;"
              type="number"
              value={lastDays}
            />
          </div>
        </div>
      </div>

      <div class="navbar-item">
        <div>
          <label class="label" for="protocol">Protocol</label>
          <div class="control">
            <div class="select">
              <select bind:value={protocol} id="protocol">
                {#each protocols as protocol}
                  <option value={protocol}>{protocol}</option>
                {/each}
              </select>
            </div>
          </div>
        </div>
      </div>

      <div class="navbar-item">
        <div>
          <label class="label" for="implementation">Implementation</label>
          <div class="control">
            <div class="select">
              <select bind:value={implementation} id="implementation">
                <option value=""></option>
                {#each implementations as implementation}
                  <option value={implementation}>{implementation}</option>
                {/each}
              </select>
            </div>
          </div>
        </div>
      </div>

      <div class="navbar-item">
        <div>
          <label class="label" for="test">Test</label>
          <div class="control">
            <div class="select">
              <select bind:value={test} id="test">
                <option value=""></option>
                {#each tests as test}
                  <option value={test}>{test}</option>
                {/each}
              </select>
            </div>
          </div>
        </div>
      </div>
    </div>
  </nav>
</header>
