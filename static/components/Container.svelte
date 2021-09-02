<script>
  import Flashcard from './Flashcard.svelte'
  import { CardModel } from '../models/Card';
  import { onModel } from '../models';

  let cards = []

  onModel(CardModel, state => cards = state.cards)
</script>

<div
  class="flex"
>
  {#each cards as card}
    <Flashcard data={card}/>
  {/each}
</div>