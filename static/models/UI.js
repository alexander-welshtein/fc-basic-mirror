export const UIModel = {
  state: {
    selectedCard: undefined
  },
  actions: {
    selectCard: card => ({ selectedCard: card }),

    unselectCard: () => ({ selectedCard: undefined })
  }
}