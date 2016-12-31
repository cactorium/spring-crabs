'use strict'

var log = function(level, msg) {
  var levelNames = ['DEBUG', 'INFO', 'WARN', 'ERR']
  if (levelNames[level]) {
    console.log(levelNames[level] + ': ' + msg)
  } else {
    console.log('UNKNOWN: ' + msg)
  }
}
log.DEBUG = 0
log.INFO  = 1
log.WARN  = 2
log.ERR   = 3

function Vec(x, y) {
  this.x = x
  this.y = y
}
Vec.add = function(a, b) {
  return new Vec(a.x + b.x, a.y + b.y)
}
Vec.sub = function(a, b) {
  return new Vec(a.x - b.x, a.y - b.y)
}
Vec.dot = function(a, b) {
  return a.x * b.x + a.y * b.y
}
Vec.scale = function(r, a) {
  return new Vec(r*a.x, r*a.y)
}
Vec.mag = function(r) {
  return Math.sqrt(r.x*r.x + r.y*r.y)
}
Vec.norm = function(r) {
  return Vec.scale(1.0/Vec.mag(r), r)
}
Vec.zero = function() {
  return new Vec(0.0, 0.0)
}

var mass = function(x, y, vx, vy, m) {
  return {
    pos: new Vec(x, y),
    vel: new Vec(vx, vy),
    forc: new Vec(0.0, 0.0),
    m: m
  }
}
var spring = function(ida, idb, l, k) {
  return {
    ida: ida, idb: idb, l: l, k: k
  }
}

var time = function(t, f) {
  var start = performance.now()
  f()
  var end = performance.now()
  t = end - start
}

function Physics() {
  this.DEFAULT_MASS = 1.0

  this.masses = {}
  this.springs = {}

  this.f = 0.0005
  this.g = new Vec(0.0, -2.0)
  this.slip = 0.2
  this.bounce = 0.998
  this.dt = 1

  this.height = 520
  this.width = 1024

  this.runtime = 0.0

  this.run = function(crabs) {
    var me = this
    time(me.runtime, function() {
      crabs.ids.forEach(function(id) {
        if (me.springs[id]) {
          var spring = me.springs[id]
          var a = me.masses[spring.ida]
          var b = me.masses[spring.idb]
          var r = Vec.sub(a.pos, b.pos)
          var f = Vec.scale(spring.k * (Vec.mag(r) - spring.l), Vec.norm(r))
        }
        if (me.masses[id]) {
          var mass = me.masses[id]
          mass.forc = Vec.add(mass.forc, Vec.scale(-me.f, mass.vel))
          mass.forc = Vec.add(mass.forc, Vec.scale(mass.m, me.g))
        }
      })
      crabs.ids.forEach(function(id) {
        if (me.masses[id]) {
          var mass = me.masses[id]
          mass.vel = Vec.add(mass.vel, Vec.scale(1.0/mass.m, mass.forc))
          mass.pos = Vec.add(mass.pos, Vec.scale(me.dt, mass.vel))
          mass.forc = Vec.zero()
        }
      })
      crabs.ids.forEach(function(id) {
        if (me.masses[id]) {
          var mass = me.masses[id]
          var collisionTime = null
          if (mass.pos.x < 0.0) { 
            collisionTime = -mass.pos.x / mass.vel.x
          }
          if (mass.pos.x > me.width) {
            collisionTime = -(mass.pos.x - me.width) / mass.vel.x
          }
          if (collisionTime != null) {
            var collisionPos = Vec.add(mass.pos, Vec.scale(collisionTime, mass.vel))
            mass.vel = new Vec(-me.bounce*mass.vel.x,
                              (1.0 - me.slip*Math.abs(mass.vel.x/Vec.mag(mass.vel)))*mass.vel.y)
            mass.pos = Vec.add(collisionPos, Vec.scale(-collisionTime, mass.vel))
          }

          collisionTime = null
          if (mass.pos.y < 0.0) {
            collisionTime = -mass.pos.y / mass.vel.y
          }
          if (mass.pos.y > me.height) {
            collisionTime = -(mass.pos.y - me.height) / mass.vel.y
          }
          if (collisionTime != null) {
            var collisionPos = Vec.add(mass.pos, Vec.scale(collisionTime, mass.vel))
            mass.vel = new Vec((1.0 - me.slip*Math.abs(mass.vel.y/Vec.mag(mass.vel)))*mass.vel.x,
                              -me.bounce*mass.vel.y)
            mass.pos = Vec.add(collisionPos, Vec.scale(-collisionTime, mass.vel))
          }
        }
      })
    })
  }

  this.resize = function(crabs) {
    // do nothing
    return
  }

  this.addMass = function(id, pos, vel, m) {
    this.masses[id] = mass(pos.x, pos.y, vel.x, vel.y, m)
  }
}

function Renderer() {
  this.runtime = 0.0
  this.MASS_RADIUS = 3.0
  this.MASS_SELECT_RADIUS = 7.0

  this.transform = function(crabs, vec) {
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
          // TODO: draw mass
          var screenPos = me.transform(crabs, mass.pos)
          canvas.fillStyle = '#222222'
          canvas.beginPath()
          canvas.arc(screenPos.x, screenPos.y, me.MASS_RADIUS, 0, Math.PI*2)
          canvas.fill()
          // log(log.DEBUG, 'mass ' + id + ' at ' + screenPos.x + ', ' + screenPos.y)

          if (crabs.ui.selected == id) {
            // TOOD: draw selection symbol
            canvas.strokeStyle = '#000000'
            canvas.beginPath()
            canvas.arc(screenPos.x, screenPos.y, 0, Math.PI*2, me.MASS_SELECT_RADIUS)
            canvas.stroke()
          }
        }
        if (physics.springs[id]) {
          var spring = physics.springs[id]
          var a = physics.masses[spring.ida],
              b = physics.masses[spring.idb]
          if (physics.muscles[id]) {
            // TODO: draw muscle
          } else {
            // TODO: draw spring
          }

          if (crabs.ui.selected == id) {
            // TODO: draw selection symbol
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

function UI() {
  this.ui = null
}

function TestUI() {
  this.mousemove = function(c, e) {
    // log(log.DEBUG, 'mouse move')
    // console.log(e)
  }
  this.click = function(c, e) {
    log(log.DEBUG, 'click')
    console.log(e)
  }
  this.keypress = function(c, e) {
    log(log.DEBUG, 'keypress')
    console.log(e)
  }
}

function Crabs(canvas, ui) {
  this.canvas = canvas
  this.uiCanvas = ui
  this.ids = []
  this.lastId = -1
  this.physics = new Physics()
  this.renderer = new Renderer()
  this.ui = new UI()
  this.components = [this.physics, this.renderer]
  this.hooks = [this.ui, new TestUI()]
  this.paused = false
  this.width = canvas.width
  this.height = canvas.height

  this.init = function() {
    var me = this

    var events = ['mousemove', 'click', 'keypress']
    events.forEach(function(name) {
      var listeners = me.hooks.filter(function(l) {
        return typeof(l[name]) != 'undefined'
      })
      log(log.INFO, 'adding hooks for ' + name + ': ' + listeners)
      ui.addEventListener(name, function(e) {
        listeners.forEach(function(l) {
          l[name](me, e)
        })
      })
    })

    this.resize()
  }
  this.go = function() {
    if (!this.paused) {
      this.components.forEach(function(c) {
        c.run(crabs)
      })

      var me = this
      window.requestAnimationFrame(function() {
        me.go()
      })
    }
    // log(log.INFO, 'frame')
  }
  this.resize = function() {
    var w = window.innerWidth, h = window.innerHeight
    log(log.DEBUG, 'resize ' + w + ', ' + h)

    this.width = w
    this.height = h

    canvas.width = w
    canvas.height = h
    ui.width = w
    ui.height = h

    this.components.forEach(function(c) {
      c.resize(this)
    })
  }

  this.addMass = function(pos, vel) {
    var newIdNum = this.lastId + 1
    var newId = newIdNum.toString()
    this.ids.push(newId)
    this.physics.addMass(newId, pos, vel, this.physics.DEFAULT_MASS)
    this.lastId = newIdNum
  }

  this.addSpring = function(m0, m1, r, k) {
    // TODO
  }

  this.addMuscle = function(m0, m1, r, k, l, p) {
    // TODO
  }
}
