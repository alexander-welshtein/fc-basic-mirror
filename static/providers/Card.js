import { MasterProvider } from './Master'

export const CardProvider = {
  getCards: async () => (await MasterProvider.sendGraphQLRequest(`
    {
      cards { 
        id
        front
        back
        date
      }
    }
  `))['cards'],
  
  createOrUpdateCard: async ({ id, front, back }) => (await MasterProvider.sendGraphQLRequest(`
    mutation {
      createOrUpdateCard(id: ${id || 0}, front: "${front}", back: "${back}") { 
        id
        front
        back
        date
      }
    }
  `))['createOrUpdateCard']
}