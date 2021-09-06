<script>
  import clsx from "clsx";
import { createEventDispatcher } from "svelte";

  export let value
  export let primary

  const dispatch = createEventDispatcher()
</script>

<div class={clsx(
  "px-4 py-2 bg-white cursor-pointer hover:bg-gray-100 active:bg-gray-200 transition-colors",
  primary && "bg-accent-default text-white hover:bg-accent-darken-a active:bg-accent-darken-b"
)}
  on:click={event => dispatch("click", event)}
>
  <span>{value}</span>
</div>