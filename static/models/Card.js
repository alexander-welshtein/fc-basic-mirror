import { CardProvider } from "../providers/Card"

export const CardModel = {
  state: {
    cards: []
  },
  actions: {
    loadCards: async () => ({
      cards: await CardProvider.getCards()
    })
  }
}