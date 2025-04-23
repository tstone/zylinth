# "Switches"

- Tileset based on https://toosday.itch.io/sci-fi-rougelike
- Drone character: https://papoycore.itch.io/free-drone
- Collectable resources: https://free-game-assets.itch.io/free-cyberpunk-resource-pixel-art-3232-icons

#### TODO

- [] Fix the render to not have a solid wall when above double vertical
- [] Implement crude "connection" process
- [] Implement selectable "power source" where panels have to be powered to work

- [] Draw a "wire" from the panel to the switch(es)
  - [] Pathfind from panel to switches
- [] Make rendering a custom command instead of spawning a TileLayer
- [] Support multiple switches per door panel (boolean AND)
- [] Support door states starting open
- [] Support multiple doors on the same door control circuit
- [] Generate a map based on door/switch input
- [] Implement map switching/progression
- [] Figure out "pickup" mechanics for item

#### Mechanic Ideas

- Panels have to be powered before they can be used
- Switches can be "painted" as a memory marker for users
- Switches don't come pre-connected. That connection has to be made by the user
- Switches can be connected to the door or panel. The panel will remember the state when the switch is disconnected. Doors will not.
- Inverter box, inverts the switches signal
- Multiplier box, allows multiple doors to be connected to it
