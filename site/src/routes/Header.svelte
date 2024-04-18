<script lang="ts">
  let {
    csv,
    days = $bindable(),
    environment = $bindable(),
    implementation = $bindable(),
    protocol = $bindable(),
    test = $bindable()
  }: any = $props();

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
        href="/"
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
            <select bind:value={environment} id="environment">
              {#each csv.environments as environment}
                <option value={environment}>{environment}</option>
              {/each}
            </select>
          </div>
        </div>
      </div>

      <div class="navbar-item">
        <div>
          <label class="label" for="days">Days</label>
          <div class="control">
            <input
              bind:value={days}
              class="input"
              id="days"
              min="1"
              style="width:80px;"
              type="number"
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
                {#each csv.protocols as protocol}
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
                {#each csv.implementations as implementation}
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
                {#each csv.tests as test}
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
