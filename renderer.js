function Renderer() {
  this.runtime = 0.0
  this.MASS_RADIUS = 3.0
  this.MASS_SELECT_RADIUS = 7.0
  this.SPRING_SELECT_RADIUS = 2.0

  this.MASS_COLOR = '#222222'
  this.SELECTED_MASS_COLOR = '#111111'
  this.ACTIVE_MASS_COLOR = '#000000'

  // transform from physics to screen
  this.p2s = function(crabs, vec) {
    return new Vec(vec.x, crabs.height - vec.y)
  }

  // transform from screen to physics
  this.s2p = function(crabs, vec) {
    return new Vec(vec.x, crabs.height - vec.y)
  }

  this.run = function(crabs) {
    var canvas = crabs.canvas.getContext('2d')
    var ui = crabs.uiCanvas.getContext('2d')
    var me = this
    time(me.runtime, function() {
      canvas.clearRect(0, 0, crabs.width, crabs.height)
      ui.clearRect(0, 0, crabs.width, crabs.height)

      var physics = crabs.physics
      crabs.ids.forEach(function(id) {
        if (physics.masses[id]) {
          var mass = physics.masses[id]
          var screenPos = me.p2s(crabs, mass.pos)
          // log(log.DEBUG, 'mass ' + id + ' at ' + screenPos.x + ', ' + screenPos.y)

          if (crabs.ui.selected == id) {
            if (crabs.ui.itemSelected) {
              canvas.strokeStyle = me.ACTIVE_MASS_COLOR
              canvas.fillStyle = me.ACTIVE_MASS_COLOR
            } else {
              canvas.strokeStyle = me.SELECTED_MASS_COLOR
              canvas.fillStyle = me.SELECTED_MASS_COLOR
            }

            canvas.beginPath()
            canvas.arc(screenPos.x, screenPos.y, me.MASS_SELECT_RADIUS, 0, Math.PI*2)
            canvas.stroke()
          } else {
            canvas.fillStyle = me.MASS_COLOR
          }

          canvas.beginPath()
          canvas.arc(screenPos.x, screenPos.y, me.MASS_RADIUS, 0, Math.PI*2)
          canvas.fill()
        }
      })
      crabs.ids.forEach(function(id) {
        if (physics.springs[id]) {
          var spring = physics.springs[id]
          var a = physics.masses[spring.ida],
              b = physics.masses[spring.idb]
          var aPos = me.p2s(crabs, a.pos),
              bPos = me.p2s(crabs, b.pos)
          if (physics.muscles[id]) {
            canvas.strokeStyle = '#000000'
            canvas.beginPath()
            canvas.moveTo(aPos.x, aPos.y)
            canvas.lineTo(bPos.x, bPos.y)
            canvas.stroke()
          } else {
            canvas.strokeStyle = '#222222'
            canvas.beginPath()
            canvas.moveTo(aPos.x, aPos.y)
            canvas.lineTo(bPos.x, bPos.y)
            canvas.stroke()
          }

          if (crabs.ui.selected == id) {
            var center = Vec.scale(0.5, Vec.add(aPos, bPos))
            canvas.beginPath()
            canvas.arc(center.x, center.y, me.SPRING_SELECT_RADIUS, 0, Math.PI*2)
            canvas.stroke()
          }
        }
      })

      // ui.clearRect(0, 0, 40, 200)
      // TODO: draw UI
    })

    ui.font = '10px sans-serif'
    ui.fillStyle = '#808080'
    ui.fillText(crabs.physics.runtime + '/' + crabs.renderer.runtime,
        0, 10)
    // TODO: draw timings/frame rate
    // log(log.DEBUG, 'timings: ' + crabs.physics.runtime + ' ms, ' +
    //                              crabs.renderer.runtime + ' ms')
  }

  this.resize = function(crabs) {
  }
}


