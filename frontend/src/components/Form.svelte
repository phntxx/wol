<script lang="ts">
  import { wakeUp } from "$lib/wrappers";

  const handleSubmission = (event: SubmitEvent) => {
    const formElement = event.target as HTMLFormElement
    const formProps = Object.fromEntries(new FormData(formElement))

    const address = formProps.address as string
    wakeUp(address)
  }
</script>

<h2 class="header">Custom address</h2>
<form on:submit|preventDefault={event => handleSubmission(event)}>
  <input type="text" class="address-input" name="address" placeholder="MAC address" />
  <input type="submit" class="submit-button" value="Wake up" />
</form>

<style>

  form {
    width: 100%;

    display: grid;
    grid-gap: 0.25rem;
  }

  input {
    display: block;
  }

  .header {
    text-align: center;
    margin: 0.75rem 0;
  }

  .address-input {
    border: 1px solid black;
    border-radius: 5px;

    font-size: 1.25rem;
    font-family: monospace;
  }

  .submit-button {
    width: 100%;
    border: 1px solid black;
    border-radius: 5px;
    background: none;

    cursor: pointer;
  }
</style>