# Ways to approach automatic linking and graph simplification

## Top Left Vertex graph approach:
### Features:
- Add a `PortGrid` to `SimData`
  - Consists of 2d vec (size, size of component grid + 1 more)
  - Each cell stores `top`, `left`. These refer to the top and left edges of a cell, these are the points where ports can exist. These will hold up to two `GridEdge`s. (Horizontal and vertical)
  - Ports solely owned by PortGrid with components storing positions in this grid or alternatively a borrow of the `Arc<AtomicU8>`
  - `PortGrid` should implement `FromWorld` to create a new `Self`, based off of the `GridSize` resource
### Issues / Considerations:
- Each edge would need a way to store two ports in the same edge as to enabled direct feed into another component.
  - Could be implemented by using a `[Option<Port>;2]`
- Would have `GridEdge`s unused stretching off the side of the component grid
  - Small impact
  - Made never accessable by fetch method on `PortGrid`
- Saving and loading
  - Could either generate port grid when loaded or store with the save
- While adding to grid is easy, removing from it due to having two possible `Port`s is slightly harder, requiring iterating over both checking the direction
- Port order in each `GridEdge` is not static due to placing components in different orders
  - This could be adjusted to always be the same and make deleting easier if we store a origin side instead of just an array of the two possible points
  - E.g:

  ```
  struct PortGrid(Vec<Vec<PlaneHolder>>);

  struct PlaneHolder {
    left: Edge::Left,
    up: Edge::Up
  }

  enum Edge {
    Left{left_origin: Option<Port>, right_origin: Option<Port>},
    Up{up_origin: Option<Port>, down_origin: Option<Port>}
  }

  struct Port(Arc<AtomicU8>);
  
  impl PortGrid {
    
  }

### Adding to the grid
To facilitate this I need to:
1. Add a ports method to either DummyComponents or Component trait
2. Should take in &self and return a Vec of ([usize; 2], side: Side)
3. If possible should let the components themselves store an `Arc<AtomicU8>` of the connection otherwise we need to do mutliple vec lookups whenever we read / set the port

1. Read .ports() method of component to add to grid.
2. **Start of sim**: Combine all wires to make `Arc<AtomicU8>` all connected.
3. For each component do .ports() and save into 


- Used enum-map crate to create a mapping between enum and the port data.
  - Stored internally has an array, fast lookup times without hashing
  - Similar to no hash haser
- Used EnumMap::from_array to use it inside a constant, evaluated at compile time instead of the enum_map! macro. This means that the order that the enum is defined in is important to define the ports. https://xkcd.com/221/