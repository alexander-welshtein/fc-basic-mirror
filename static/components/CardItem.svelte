<script>
  import { actModel } from "../models";
  import { UIModel } from "../models/UI";

  export let card;

  const { selectCard } = actModel(UIModel);
</script>

<div
  class="flex flex-col rounded-md shadow-md items-center p-4 bg-white mr-4 cursor-pointer"
  on:click={() => selectCard(card)}
>
  <div>
    {card.front}
  </div>
  <div class="text-gray-500">
    {card.back}
  </div>
</div>
