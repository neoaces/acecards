use ace;

fn main() {
    let new: ace::Card = ace::Card::makeCard("Who is the president?", "obama");

    println!("{:#?}", new);
    new.print_card();
}
