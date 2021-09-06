<script>
  export let icon
  export let alt
</script>

<div>
  <img
    class="w-12 h-12 p-2 bg-white cursor-pointer hover:bg-gray-100 transition-colors"
    src={`assets/${icon}.svg`}
    alt={alt}
    on:click
  >
</div>