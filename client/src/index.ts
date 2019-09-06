import * as PIXI from 'pixi.js'

window.addEventListener('load', () => {
  // setup pixi
  let app = new PIXI.Application({
    antialias: false,
    transparent: false,
    resolution: 1,
  })
  document.body.appendChild(app.view)
  app.renderer.backgroundColor = 0x0a1215
  app.renderer.resize(document.body.clientWidth, document.body.clientHeight)
  window.addEventListener('resize', () => {
    app.renderer.resize(document.body.clientWidth, document.body.clientHeight)
  })

  // setup socket
  const socket = new WebSocket('ws://localhost:8000')

  // prepare rectangle texture
  let texture = app.renderer.generateTexture(
    new PIXI.Graphics()
      .beginFill(0xfff6f1)
      .lineStyle(0)
      .drawRect(0, 0, 1, 1)
      .endFill(),
    PIXI.SCALE_MODES.LINEAR,
    1,
  )

  // update entities
  let scale = 10

  socket.addEventListener('message', (e) => {
    app.stage.children = []

    let entities = JSON.parse(e.data) as [
      { data: { x: number; y: number } },
      { sections: { x: number; y: number }[] },
    ][]

    entities.forEach(([position, structure], index) => {
      const entity = new PIXI.Container()

      structure.sections.forEach((section) => {
        let sprite = new PIXI.Sprite(texture)

        sprite.position.x = section.x * scale
        sprite.position.y = section.y * scale

        sprite.width = 1 * scale
        sprite.height = 1 * scale

        entity.addChild(sprite)
      })

      app.stage.addChild(entity)

      // adjust position
      entity.position.x = position.data.x * scale
      entity.position.y = position.data.y * scale
    })
  })
})
