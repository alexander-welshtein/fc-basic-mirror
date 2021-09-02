import { CardProvider } from '../providers/Card'

/**
 * @typedef Card
 * @property {string} [id]
 * @property {string} [front]
 * @property {string} [back]
 * @property {string} [date]
 */

export const Card = {
  /**
   * @param {Card} entity 
   * @returns {Card}
   */
  from: entity => entity
}

export const Cards = {
  /**
   * @param {Card[]} entities 
   * @returns {Card[]}
   */
  from: entities => entities
}

export const CardModel = {
  state: {
    cards: []
  },
  actions: {
    loadCards: async () => ({
      cards: await CardProvider.getCards()
    }),
    createCard: async (_, input) => {
      const output = await CardProvider.createCard(input)

      return {
        cards: [
          ...CardModel.state.cards,
          output
        ]
      }
    }
  }
}