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
  
  createCard: async ({ front, back }) => (await MasterProvider.sendGraphQLRequest(`
    mutation {
      createCard(front: "${front}", back: "${back}") { 
        id
        front
        back
        date
      }
    }
  `))['createCard']
}