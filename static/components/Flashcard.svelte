<script>
  export let data

  const { front, back } = data
</script>

<div
  class="flex flex-col rounded-md shadow-md items-center p-4 bg-white mr-4"
>
  <div>
    {front}
  </div>
  <div
    class="text-gray-500"
  >
    {back}
  </div>
</div>