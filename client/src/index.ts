import * as PIXI from 'pixi.js'
import * as d3 from 'd3'

window.addEventListener('load', () => {
  // setup pixi
  let app = new PIXI.Application({
    antialias: false,
    transparent: false,
    resolution: 1,
  })
  document.body.appendChild(app.view)
  app.renderer.backgroundColor = 0x0a1215
  app.renderer.resize(document.body.clientWidth, document.body.clientHeight * 2)
  window.addEventListener('resize', () => {
    app.renderer.resize(document.body.clientWidth, document.body.clientHeight * 2)
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
  let scale = 5

  let count = 0

  setInterval(() => {
    console.log(count)

    count = 0
  }, 1000)

  socket.addEventListener('message', (e) => {
    count += 1

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

  d3.select('#root').call(
    d3
      .zoom()
      // .scaleExtent([1, 8])
      .on('zoom', zoom),
  )

  function zoom(e) {
    app.stage.position.x = d3.event.transform.x
    app.stage.position.y = d3.event.transform.y
    app.stage.scale.x = d3.event.transform.k
    app.stage.scale.y = d3.event.transform.k
  }
})
