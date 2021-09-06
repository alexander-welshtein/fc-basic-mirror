<script>
  import { actModel } from "../models";
  import { Card, CardModel } from "../models/Card";
  import IconButton from "./IconButton.svelte";

  export let opened = false;

  let front = "";
  let back = "";

  const { createOrUpdateCard } = actModel(CardModel);
</script>

<div class="flex space-x-4">
  {#if opened}
    <input
      class="w-1/2 px-4 py-2 text-gray-600 outline-none"
      on:input={event => (front = event.target.value)}
    />
    <input
      class="w-1/2 px-4 py-2 text-gray-600 outline-none"
      on:input={event => (back = event.target.value)}
    />
  {/if}
  <IconButton
    icon={opened ? "checkmark" : "plus"}
    alt={opened ? "Confirm new card" : "Add new card"}
    on:click={() => {
      opened &&
        front.length &&
        back.length &&
        createOrUpdateCard(Card.from({ front, back }));
      !opened && (front = "" || true) && (back = "" || true);
      opened = !opened;
    }}
  />
</div>
