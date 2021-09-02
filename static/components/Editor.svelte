<script>
  import { actModel } from "../models";
  import { Card, CardModel } from "../models/Card";
  import Button from "./Button.svelte";

  export let opened = false;

  let front = "";
  let back = "";

  const { createCard } = actModel(CardModel);
</script>

<div class="flex space-x-4">
  {#if opened}
    <input
      class="w-1/2 px-4 py-2 text-gray-600 outline-none"
      on:input={(event) => (front = event.target.value)}
    />
    <input
      class="w-1/2 px-4 py-2 text-gray-600 outline-none"
      on:input={(event) => (back = event.target.value)}
    />
  {/if}
  <Button
    icon={opened ? "checkmark" : "plus"}
    on:click={() => {
      opened &&
        front.length &&
        back.length &&
        createCard(Card.from({ front, back }));
      !opened && (front = "" || true) && (back = "" || true);
      opened = !opened;
    }}
  />
</div>
