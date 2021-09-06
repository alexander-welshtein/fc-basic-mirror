<script>
  import CardItem from "./CardItem.svelte";
  import { Card, CardModel } from "../models/Card";
  import { useModel } from "../models";

  let cards = [];

  useModel(CardModel, state => cards = state.cards);
</script>

<div class="flex">
  {#each cards as card}
    <CardItem card={Card.clone(card)} />
  {/each}
</div>
