# Spatial Hash Table Manager Handle Point Update

## Before

``` rust
for dep in dependency_graph.get_all_dependents(ent) {
  spatial_entity_map.remove_from_all(dep);
  insert(&dep, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
}
```

## After

``` rust
for dep in dependency_graph.get_all_dependents(ent) {
  if hiddens.get(dep).is_none() {
    spatial_entity_map.remove_from_all(dep);
    insert(&dep, &mut spatial_entity_map, &screen_points, &screen_lines, &screen_circles);
  }
}
```

## Merit

Should check if it is hidden before go into spatial entity map