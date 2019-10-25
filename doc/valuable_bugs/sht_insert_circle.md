# Insert circle in sht

Date: Oct, 25, 2019

## Original code

``` rust
for j in top.max(0)..(bottom.min(self.x_tiles as i64) + 1) {
  for i in left.max(0)..(right.min(self.y_tiles as i64) + 1) {
    // ...
  }
}
```

## Fixed code

``` rust
//                                    v this was x, should be y
for j in top.max(0)..(bottom.min(self.y_tiles as i64) + 1) {
  //                                    v this was y, should be x
  for i in left.max(0)..(right.min(self.x_tiles as i64) + 1) {
    // ...
  }
}
```

## Merit

- Any word related to `top`, `bottom`, should be in `y` direction
- Any word related to `left`, `right`, `horizontal`, should be in `x` direction

This is not always the case though...