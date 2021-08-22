<script>
  import AddButton from './Button.svelte'
  import Flashcard from './Flashcard.svelte'

  export let data = [
    {
      front: 'This does not tickle my fancy',
      back: 'Это не щекочет мою фантазию'
    },
    {
      front: 'You\'ve made your country proud',
      back: 'Вы заставили свою страну гордиться'
    }
  ]
</script>

<div
  class="flex"
>
  {#each data as item}
    <Flashcard data={item}/>
  {/each}
</div>