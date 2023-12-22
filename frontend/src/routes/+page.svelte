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
    margin: 0;

    font-family: "Helvetica 45 Light", sans-serif;
  }

  :global(body) {
    display: grid;
    place-items: center;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      color: #e3e5dc;
      background: #191f24;
    }
  }

  @media (prefers-color-scheme: light) {
    :global(body) {
      color: #021627;
      background: #f0eee5;
    }
  }

  @media (min-width: 1536px) {
    .container {
      max-width: 90%;
      padding: 1rem;
    }
  }

  @media screen and (max-width: 766px) {
    .container {
      max-width: 100%;
      padding: 0.5rem;
    }
  }

  .container {
    max-width: 80%;
    max-height: 50%;

    padding: 2rem;
  }
</style>