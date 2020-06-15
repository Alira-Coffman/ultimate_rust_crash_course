fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
   let mut missiles = 8;
   let ready = 2;
   println!("Firing {} of my {} missiles...", ready, missiles);

   missiles -= ready;
   println!("Firing {} of my {} missiles...", ready, missiles);

   let mut missiles = STARTING_MISSILES;
   let ready = READY_AMOUNT;

   println!("Firing {} of my {} missiles...", ready, missiles);

   missiles -= ready;
   println!("Firing {} of my {} missiles...", ready, missiles);


   let(mut missiles, ready) = (9, 2);
   println!("Firing {} of my {} missiles...", ready, missiles);

   missiles -= ready;
   println!("Firing {} of my {} missiles...", ready, missiles);
    let set = 2;
}
