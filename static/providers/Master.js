export const MasterProvider = {
  sendGraphQLRequest: async query => {
    const response = await fetch('/graphql', {
      method: 'post',
      headers: { 'Content-Type': 'application/json'},
      body: JSON.stringify({ query })
    })

    const json = await response.json()

    return json['data']
  }
}