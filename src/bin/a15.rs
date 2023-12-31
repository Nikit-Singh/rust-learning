// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32),
}

fn main() {
    let bs = Ticket::Backstage(42.0, "Zappy".into());
    let vip = Ticket::Vip(420.0, "Happy".into());
    let standard = Ticket::Standard(4.20);
    let ticks = vec![bs, vip, standard];

    for ticket in ticks {
        match ticket {
            Ticket::Backstage(price, name) => println!("{} - {}", name, price),
            Ticket::Vip(price, name) => println!("{} - {}", name, price),
            Ticket::Standard(price ) => println!("{}", price)
        }
    }
}
