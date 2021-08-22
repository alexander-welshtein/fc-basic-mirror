<script>
  export let icon
</script>

<div>
  <img
    class="w-12 h-12 p-2 bg-white cursor-pointer hover:bg-gray-100 hov transition-colors"
    src={`assets/${icon}.svg`}
    alt="Add flashcard"
    on:click
  >
</div>