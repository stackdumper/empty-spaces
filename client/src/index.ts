import * as PIXI from 'pixi.js'

window.addEventListener('load', () => {
  const socket = new WebSocket('ws://localhost:8000')

  let app = new PIXI.Application({
    antialias: true, // default: false
    transparent: false, // default: false
    resolution: 0.5, // default: 1
  })
  document.body.appendChild(app.view)
  app.renderer.backgroundColor = 0x0a1215
  app.renderer.resize(document.body.clientWidth * 2, document.body.clientHeight * 2)
  window.addEventListener('resize', () => {
    app.renderer.resize(document.body.clientWidth * 2, document.body.clientHeight * 2)
  })

  let entities = []

  socket.addEventListener('message', (e) => {
    let data = JSON.parse(e.data) as [any, any, any][]

    data.forEach(([position, _, mass], i) => {
      let radius = Math.cbrt(mass.data) / 2

      if (!entities[i]) {
        let entity = new PIXI.Graphics()
        entity.beginFill(0xfff6f1)
        entity.drawCircle(0, 0, radius * 2)
        entity.endFill()

        app.stage.addChild(entity)

        entities[i] = entity
      }

      let entity = entities[i]

      entity.x = position.data.x * 500
      entity.y = position.data.y * 500
    })

    app.renderer.render(app.stage)
  })
})
