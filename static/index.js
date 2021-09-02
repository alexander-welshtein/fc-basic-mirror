import Application from './components/Application.svelte'
import "tailwindcss/tailwind.css"
import { actModel } from './models'
import { CardModel } from './models/Card'

new Application({
  target: document.querySelector('#root')
})

const { loadCards } = actModel(CardModel)

loadCards()
