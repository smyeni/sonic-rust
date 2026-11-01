  struct Account {
      name: String,
  }
  
  impl Account {
      fn new(name: &str) -> Account {
          Account { name: name.to_string() }
      }
  }
  
  impl Drop for Account {
      fn drop(&mut self) {
          println!("  (dropping house \"{}\")", self.name);
      }
  }
  
  // Java's rename(): follow the reference and repaint the caller's house.
  fn rename(acct: &mut Account) {
      acct.name = String::from("Changed");
  }
  
  // Java's replace(): only the local copy of the reference is re-pointed.
  #[allow(unused_assignments)]
  fn replace_local(acct: &mut Account) {
      let mut cur = acct;                      // our own local reference
      let mut other = Account::new("Other");   // a brand-new house
      cur = &mut other;                        // re-point the local reference only
      println!("  inside replace_local, cur sees \"{}\"", cur.name);
  }
  
  // What Java cannot do: overwrite the caller's house in place through the reference.
  fn replace_in_place(acct: &mut Account) {
      *acct = Account::new("Overwritten");     // old contents are dropped right here
  }
  
  // Moving the whole value in: the caller loses it.
  fn consume(acct: Account) {
      println!("  consume got \"{}\"", acct.name);
  }   // acct dropped here
  
  fn main() {
      let mut a = Account::new("Original");
      println!("start:            {}", a.name);
  
      rename(&mut a);
      println!("after rename:     {}", a.name);
  
      replace_local(&mut a);
      println!("after replace_local: {}", a.name);
  
      replace_in_place(&mut a);
      println!("after replace_in_place: {}", a.name);
  
      consume(a);
      // println!("{}", a.name);   // error: borrow of moved value: `a`
      println!("main ends");
  }
