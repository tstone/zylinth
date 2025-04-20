# Bevy Maze Gen Test

- Map tileset: https://petricakegames.itch.io/cosmic-legacy-scifi-tileset
- Drone character: https://papoycore.itch.io/free-drone

#### Major Changes

- Probably should have a "regions" module separate from a "tiles" module separate from what is actually being generated
- Gen: Regions => Tiles => Sprites
  - a "room" is just a region of a specific room type
  - the room gen process shouldn't resolve into tiles yet, but should have an enum RegionType
  - would be nice to have something like .connect_right to join rooms
  - also maybe a .intersect
- Each region has it's own type T
- Update ctx to be regions+src+dest so that either can be referenced
- Layer 0 is always ground, layer 1 is buildings
- It should be possible to render regions with different tilesets
  - This means each tile needs to know: function, tileset, collidable
- Prefabs
  - Insert a region of "Prefab(15)" tile and the whole thing inserts
  - Some nice way to define prefabs

#### TODO

- [] More border fixes
- [] BG decorations aren't showing up
- [] Make rendering a custom command instead of spawning a TileLayer
- [] Prefabs
