<script>
  import Header from "$components/Header.svelte"
  import Form from "$components/Form.svelte";
  import List from "$components/List.svelte"
</script>

<div class="container">
  <Header />
  <Form />
  <List />
</div>


<style>
  :global(html), :global(body) {
    width: 100%;
    height: 100%;
  }

  :global(body) {
    display: grid;
    place-items: center;
  }

  .container {
    max-width: 50%;
    max-height: 50%;

    padding: 2rem;
  }
</style>