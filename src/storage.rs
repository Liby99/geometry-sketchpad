#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(usize, i32);

#[derive(Clone)]
pub struct Storage<T> {
  items: Vec<T>,
  generations: Vec<i32>,
  empty_spots: Vec<usize>,
}

impl<T> Storage<T> {
  pub fn new() -> Storage<T> {
    Storage {
      items: vec![],
      generations: vec![],
      empty_spots: vec![],
    }
  }

  pub fn get(&self, Id (index, gen): Id) -> Option<&T> {
    if index < self.generations.len() && self.generations[index] == gen {
      Some(&self.items[index])
    } else {
      None
    }
  }

  pub fn remove(&mut self, Id (index, gen): Id) -> bool {
    if index < self.generations.len() && self.generations[index] == gen {
      self.generations[index] += 1;
      self.empty_spots.push(index);
      true
    } else {
      false
    }
  }

  pub fn update(&mut self, Id (index, gen): Id, item: T) -> bool {
    if index < self.generations.len() && self.generations[index] == gen {
      self.items[index] = item;
      true
    } else {
      false
    }
  }

  pub fn add(&mut self, item: T) -> Id {
    match self.empty_spots.pop() {
      Some(index) => {
        self.items[index] = item;
        Id(index, self.generations[index])
      },
      None => {
        self.items.push(item);
        self.generations.push(0);
        Id(self.items.len() - 1, 0)
      }
    }
  }

  pub fn exists(&self, id: Id) -> bool {
    self.get(id).is_some()
  }

  pub fn sanity_check(self) -> bool {
    self.items.len() == self.generations.len()
  }
}

pub struct StorageIter {
  item_index: usize,
  gen_iter: std::vec::IntoIter<i32>,
  empty_iter: std::vec::IntoIter<usize>,
}

impl Iterator for StorageIter {
  type Item = Id;

  fn next(&mut self) -> Option<Self::Item> {

    // Move forward if empty spot matches i
    let mut i = self.item_index;
    while self.empty_iter.nth(0) == Some(i) {
      self.empty_iter.next();
      self.gen_iter.next();
      i += 1;
    }

    // Iterate
    self.item_index = i + 1;
    if let Some(gen) = self.gen_iter.next() {
      Some(Id(i, gen))
    } else {
      None
    }
  }
}

impl<T> IntoIterator for Storage<T> {
  type Item = Id;
  type IntoIter = StorageIter;

  fn into_iter(self) -> Self::IntoIter {

    // First sort the array
    let mut sorted_empty_indices = self.empty_spots.clone();
    sorted_empty_indices.sort();

    // Then generate the iterator
    StorageIter {
      item_index: 0,
      gen_iter: self.generations.into_iter(),
      empty_iter: sorted_empty_indices.into_iter(),
    }
  }
}