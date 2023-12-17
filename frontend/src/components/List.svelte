<script lang="ts">
import { getData } from "$lib/api";

import ListItem from "$components/ListItem.svelte";
</script>

<h2 class="header">Saved devices</h2>
{#await getData()}
  <p>Loading, please wait...</p>
{:then data}
  <div class="list-container">
    {#each data.devices as item}
      <ListItem name={item.name} address={item.mac} />
    {/each}
  </div>
{/await}

<style>
  .header {
    text-align: center;
    margin: 0.75rem 0;
  }

  .list-container {
    width: 100%;

    display: grid;
    grid-gap: 0.25rem;
  }
</style>