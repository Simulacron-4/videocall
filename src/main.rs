use std::{
  cell::RefCell,
  rc::{Rc, Weak},
};

#[derive(Debug)]
struct User {
  name: String,
  rooms: Vec<Rc<Room>>,
}

impl Drop for User {
  fn drop(&mut self) {
      println!("Dropping user=\"{}\" from rooms=\"{:?}\"", self.name, self.rooms);
      // Remove user from all rooms
      self.rooms.iter().for_each(|room| {
          room.users.borrow_mut().retain(|element| element.upgrade().is_some());
      });
  }
}

#[derive(Debug)]
struct Room {
  name: String,
  users: RefCell<Vec<Weak<User>>>,
}

impl Drop for Room {
  fn drop(&mut self) {
      println!("Dropping room {}", self.name);
  }
}



fn main() {
  let rust_room: Rc<Room> = Rc::new(Room {
      name: "rust".to_string(),
      users: RefCell::new(vec![]),
  });
  let fosdem_room: Rc<Room> = Rc::new(Room {
      name: "fosdem".to_string(),
      users: RefCell::new(vec![]),
  });
  let user_mike: Rc<User> = Rc::new(User {
      name: "mike".to_string(),
      rooms: vec![rust_room.clone(), fosdem_room.clone()],
  });
  let user_ovidiu: Rc<User> = Rc::new(User {
      name: "ovidiu".to_string(),
      rooms: vec![rust_room.clone()],
  });

  rust_room.users.borrow_mut().push(Rc::downgrade(&user_mike));
  rust_room.users.borrow_mut().push(Rc::downgrade(&user_ovidiu));
  fosdem_room.users.borrow_mut().push(Rc::downgrade(&user_mike));

  print_users_in_a_room(&rust_room);
  print_users_in_a_room(&fosdem_room);
  drop(user_mike);
  print_users_in_a_room(&rust_room);
  print_users_in_a_room(&fosdem_room);
  drop(user_ovidiu);
  print_users_in_a_room(&rust_room);
  print_users_in_a_room(&fosdem_room);
}

fn print_users_in_a_room(room: &Rc<Room>) {
  for u in room.users.borrow().iter() {
      match u.upgrade() {
          Some(u) => println!("User {} in room {}", u.name, room.name),
          None => {
              println!("Should not be possible because the user is dropped from the room with retain filter");
          },
      }
  }
}