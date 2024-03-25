use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Debug)]
struct User {
  room: Rc<Room>,
}

impl Drop for User {
  fn drop(&mut self) {
    println!("Dropping user in room {}", self.room.name);
    self.room.users.borrow_mut().retain(|u| u.upgrade().is_some());
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


fn make_user(room_rc: &Rc<Room>) -> Rc<User> {
  let user = User { room: room_rc.clone() };
  let user_rc = Rc::new(user);
  room_rc.users.borrow_mut().push(Rc::downgrade(&user_rc));
  user_rc
}

fn make_room(name: &str) -> Rc<Room> {
  Rc::new(Room { name: name.to_string(), users: RefCell::new(Vec::new()) })
}

fn main() {
   let mut users: HashMap<String, Rc<User>> = HashMap::new();

   let room_rc = make_room("rust");

   users.insert("ovidiu".to_string(), make_user(&room_rc));
   users.insert("mike".to_string(), make_user(&room_rc));

   print_users(&users);
   drop(room_rc);
   
   let room_rc = make_room("fosdem");
   users.insert("niels".to_string(), make_user(&room_rc));
   users.insert("victor".to_string(), make_user(&room_rc));

   drop(room_rc);

   users.remove("victor");
   print_users(&users);
   users.remove("niels");

   print_users(&users);

}

fn print_users(users: &HashMap<String, Rc<User>>) {
  println!("############## Users in the system:");
  for (name, user) in users {
    println!("{} is in room {} with rc {}, wc {}", name, user.room.name, 
      Rc::<Room>::strong_count(&user.room), Rc::<Room>::weak_count(&user.room));
    println!("  Users in room:");
    for u in user.room.users.borrow().iter() {
      if let Some(u) = u.upgrade() {
        println!("    {:?}", u);
      }
    }
  }
}


