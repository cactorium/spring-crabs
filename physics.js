'use strict'

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
  }

  this.resize = function(crabs) {
    // do nothing
    return
  }

  this.addMass = function(id, pos, vel, m) {
    this.masses[id] = mass(pos.x, pos.y, vel.x, vel.y, m)
  }

  this.addSpring = function(id, m0, m1, r, k) {
    this.springs[id] = spring(m0, m1, r, k)
  }
}


