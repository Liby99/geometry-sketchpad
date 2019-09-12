#[derive(Copy, Clone, Debug)]
pub struct Id(usize, i32);

pub struct Storage<T> {
  items: Vec<T>,
  generations: Vec<i32>,
  empty_spots: Vec<usize>,
}

impl<T> Storage<T> {
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