import { MasterProvider } from "./Master"

export const CardProvider = {
  getCards: async () => (await MasterProvider.sendGraphQLRequest(`{ cards { id front back date } }`))['cards']
}