use std::{
    borrow::BorrowMut,
    sync::{Arc, RwLock, Weak},
};

#[derive(Debug)]
struct User {
    name: String,
    rooms: Vec<Arc<Room>>,
}

impl Drop for User {
    fn drop(&mut self) {
        println!(
            "Dropping user=\"{}\" from rooms=\"{:?}\"",
            self.name, self.rooms
        );
        // Remove user from all rooms
        self.rooms
            .iter()
            .for_each(|room| match room.users.write().borrow_mut() {
                Ok(ok) => {
                    ok.retain(|element| element.upgrade().is_some());
                }
                Err(err) => {
                    print!("Could not remove user from room, because we couldn't get a write lock")
                }
            });
    }
}

#[derive(Debug)]
struct Room {
    name: String,
    users: RwLock<Vec<Weak<User>>>,
}

impl Drop for Room {
    fn drop(&mut self) {
        println!("Dropping room {}", self.name);
    }
}

fn main() {
    let rust_room: Arc<Room> = Arc::new(Room {
        name: "rust".to_string(),
        users: RwLock::new(vec![]),
    });
    let fosdem_room: Arc<Room> = Arc::new(Room {
        name: "fosdem".to_string(),
        users: RwLock::new(vec![]),
    });
    let user_mike: Arc<User> = Arc::new(User {
        name: "mike".to_string(),
        rooms: vec![rust_room.clone(), fosdem_room.clone()],
    });
    let user_ovidiu: Arc<User> = Arc::new(User {
        name: "ovidiu".to_string(),
        rooms: vec![rust_room.clone()],
    });

    match rust_room.users.write().borrow_mut() {
        Ok(ok) => ok.push(Arc::downgrade(&user_mike)),
        Err(_) => {}
    };
    match rust_room.users.write().borrow_mut() {
        Ok(ok) => ok.push(Arc::downgrade(&user_ovidiu)),
        Err(_) => {}
    };
    match fosdem_room.users.write().borrow_mut() {
        Ok(ok) => ok.push(Arc::downgrade(&user_mike)),
        Err(_) => {}
    };
    print_users_in_a_room(&rust_room);
    print_users_in_a_room(&fosdem_room);
    drop(user_mike);
    print_users_in_a_room(&rust_room);
    print_users_in_a_room(&fosdem_room);
    drop(user_ovidiu);
    print_users_in_a_room(&rust_room);
    print_users_in_a_room(&fosdem_room);
}

fn print_users_in_a_room(room: &Arc<Room>) {
    match room.users.read() {
        Ok(ok) => ok
            .iter()
            .filter_map(|user| user.upgrade())
            .for_each(|u| println!("User {:?} in room {}", u.name, room.name)),
        Err(_) => {}
    };
}
