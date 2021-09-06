export const useModel = (model, on) => {
  model.on = [
    ...model.on || [],
    on
  ]
}

export const actModel = model => {
  const apply = patch => {
    model.state = { ...model.state, ...patch }
    model.on?.forEach(subscription => subscription(model.state))
  }

  return Object
    .entries(model.actions)
    .reduce((result, [key, action]) => ({
      ...result,
      [key]: payload => {
        const patch = action(payload)
        patch['then'] ? patch.then(apply) : apply(patch)
      }
    }), {})
}