'use strict'

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
var muscle = function(r, a, p) {
  return {
    r: r, a: a, p: p
  }
}

function Physics() {
  this.DEFAULT_MASS = 1.0

  this.masses = {}
  this.springs = {}
  this.muscles = {}

  this.f = 0.0005
  this.g = new Vec(0.0, -0.0)
  this.slip = 0.2
  this.bounce = 0.998
  this.dt = 1
  this.wavetime = 0.0
  this.wavestep = 0.05

  this.height = 520
  this.width = 1024

  this.runtime = 0.0

  this.run = function(crabs) {
    var me = this
    time(me.runtime, function() {
      crabs.ids.forEach(function(id) {
        if (me.muscles[id]) {
          var muscle = me.muscles[id]
          var spring = me.springs[id]
          if (spring) {
            spring.l = muscle.r*(1.0+muscle.a*Math.sin(me.wavetime + muscle.p))
          }
        }
        if (me.springs[id]) {
          var spring = me.springs[id]
          var a = me.masses[spring.ida]
          var b = me.masses[spring.idb]
          var r = Vec.sub(a.pos, b.pos)
          var f = Vec.scale(spring.k * (Vec.mag(r) - spring.l), Vec.norm(r))
          a.forc = Vec.sub(a.forc, f)
          b.forc = Vec.add(b.forc, f)
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
    me.wavetime += me.wavestep
    if (me.wavetime > 2*Math.PI) {
      me.wavetime -= 2*Math.PI
    }
  }

  this.resize = function(crabs) {
    // do nothing
    return
  }

  this.addMass = function(id, pos, vel, m) {
    this.masses[id] = mass(pos.x, pos.y, vel.x, vel.y, m)
    return id
  }

  this.addSpring = function(id, m0, m1, r, k) {
    if (!this.masses[m0] || !this.masses[m1]) {
      throw 'invalid spring'
    }
    this.springs[id] = spring(m0, m1, r, k)
    return id
  }

  this.addMuscle = function(id, r, a, p) {
    if (!this.springs[id]) {
      throw 'invalid muscle'
    }
    this.muscles[id] = muscle(r, a, p)
    return id
  }
}


