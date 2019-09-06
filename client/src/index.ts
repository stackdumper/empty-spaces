import * as PIXI from 'pixi.js'

window.addEventListener('load', () => {
  const socket = new WebSocket('ws://localhost:8000')

  let app = new PIXI.Application({
    antialias: false, // default: false
    transparent: false, // default: false
    resolution: 1, // default: 1
  })
  document.body.appendChild(app.view)
  app.renderer.backgroundColor = 0x0a1215
  app.renderer.resize(document.body.clientWidth, document.body.clientHeight)
  window.addEventListener('resize', () => {
    app.renderer.resize(document.body.clientWidth, document.body.clientHeight)
  })

  // let canvas = new PIXI.Graphics()
  // app.stage.addChild(canvas)

  // socket.addEventListener('message', (e) => {
  //   let data = JSON.parse(e.data) as [any, any, any][]

  //   canvas.clear()
  //   canvas.beginFill(0xfff6f1)
  //   data.forEach(([position, _, mass], i) => {
  //     let radius = Math.cbrt(mass.data) / 2

  //     canvas.drawCircle(position.data.x * 500, position.data.y * 500, radius * 2)
  //   })
  //   canvas.endFill()

  //   // app.renderer.render(app.stage)
  // })

  // let entities = []

  // socket.addEventListener('message', (e) => {
  //   let data = JSON.parse(e.data) as [any, any, any][]

  //   data.forEach(([position, _, mass], i) => {
  //     let radius = Math.cbrt(mass.data) / 2

  //     if (!entities[i]) {
  //       let entity = new PIXI.Graphics()
  //       entity.beginFill(0xfff6f1)
  //       entity.drawCircle(0, 0, radius * 2)
  //       entity.endFill()

  //       app.stage.addChild(entity)

  //       entities[i] = entity
  //     }

  //     let entity = entities[i]

  //     entity.x = position.data.x * 500
  //     entity.y = position.data.y * 500
  //   })

  //   // app.renderer.render(app.stage)
  // })

  let entities: PIXI.Sprite[] = []
  let container = new PIXI.Container()

  document.addEventListener('keydown', (e) => {
    console.log('keydown')

    switch (e.key) {
      case 'w':
        container.position.y += 100
        break
      case 's':
        container.position.y -= 100
        break
      case 'a':
        container.position.x += 100
        break
      case 'd':
        container.position.x -= 100
        break
    }
  })
  // container.cacheAsBitmap = true
  app.stage.addChild(container)

  let entity = new PIXI.Graphics()

  entity.beginFill(0xfff6f1)
  entity.lineStyle(0)
  entity.drawRect(0, 0, 1, 1)
  entity.endFill()

  let texture = app.renderer.generateTexture(entity, PIXI.SCALE_MODES.LINEAR, 1)
  let multiplier = 50

  socket.addEventListener('message', (e) => {
    let data = JSON.parse(e.data) as [any, any, any][]

    data.forEach(([position, _], i) => {
      if (!entities[i]) {
        let sprite = new PIXI.Sprite(texture)

        sprite.width = 1 * multiplier
        sprite.height = 1 * multiplier
        sprite.interactive = false
        sprite.interactiveChildren = false

        container.addChild(sprite)

        entities[i] = sprite
      }

      let entity = entities[i]

      entity.position.x = position.data.x * multiplier
      entity.position.y = position.data.y * multiplier
    })

    app.renderer.render(app.stage)
  })
})
