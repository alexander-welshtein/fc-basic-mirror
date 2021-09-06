<script>
  import { Card, CardModel } from "../models/Card";
  import clsx from "clsx";
  import { actModel, useModel } from "../models";
  import { UIModel } from "../models/UI";
  import Button from "./Button.svelte";

  let opened = false;
  let card;
  let front = "";
  let back = "";

  useModel(UIModel, ({ selectedCard }) => {
    card = Card.from(selectedCard);
    front = card?.front
    back = card?.back
    opened = !!card;
  });

  const { unselectCard } = actModel(UIModel);
  const { createOrUpdateCard } = actModel(CardModel);
</script>

<div
  class={clsx(
    "fixed z-10 inset-0 overflow-y-auto",
    opened ? "block" : "hidden"
  )}
>
  <!-- Overlay -->
  <div
    class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
    on:click={() => unselectCard(card)}
  >
    <!-- Center -->
    <div class="w-full h-full flex items-center justify-center">
      <!-- Shape -->
      <div
        class="flex flex-col bg-white p-4 rounded-md space-y-4 w-full sm:w-1/3"
        on:click|stopPropagation
      >
        <!-- Front input -->
        <input
          class="w-full px-4 py-2 text-gray-600 bg-gray-50 outline-none"
          value={card?.front || ""}
          on:input={event => front = event.target.value}
        />
        <!-- Back input -->
        <input
          class="w-full px-4 py-2 text-gray-600 bg-gray-50 outline-none"
          value={card?.back || ""}
          on:input={event => back = event.target.value}
        />
        <!-- Footer -->
        <div class="flex justify-end space-x-4">
          <Button value="Cancel" on:click={() => unselectCard(card)} />
          <Button 
            value="Confirm" 
            primary on:click={() => {
              (front !== card.front || back !== card.back)
              && createOrUpdateCard(Card.from({ ...card, front, back }))
              unselectCard(card)
            }} 
            />
        </div>
      </div>
    </div>
  </div>
</div>
