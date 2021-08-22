<script>
  import Button from './Button.svelte'

  export let opened = false
</script>

<div
  class="flex space-x-4"
>
  {#if opened}
    <input class="w-1/2 px-4 py-2 text-gray-600 outline-none">
    <input class="w-1/2 px-4 py-2 text-gray-600 outline-none">
  {/if}
  <Button
    icon={opened ? 'checkmark' : 'plus'}
    on:click={() => opened = !opened}
  />
</div>