<script lang="ts">

export let name: string;
export let address: string;

import { wakeUp } from "$lib/wrappers"

const handleClick = () => wakeUp(address, name)

</script>

<button on:click={handleClick}>
  <p class="item-name">{name}</p>
  <p class="item-address">{address}</p>
</button>

<style>
  button {
    width: 100%;

    display: block;
    text-align: start;

    cursor: pointer;
    
    border: 1px solid black;
    border-radius: 5px;
    background: none;
  }

  .item-name {
    width: 100%;
    font-weight: bold;
    font-size: 1.25rem;
    margin: 0;
  }

  .item-address {
    font-style: italic;
    font-size: small;
    margin: 0;
  }
</style>