use exchanger::Exchanger;
fn main() {
    let ex:Exchanger<i32> = Exchanger::new();
    let ex1 = ex.clone();
    let ex2 = ex.clone();
    let ex3 = ex.clone();
    let th1 = std::thread::spawn(||{
        let my_card = 1;
        card_exchange(ex1, my_card);
    });

    let th2 = std::thread::spawn(||{
        let my_card = 2;
        card_exchange(ex2, my_card);
    });

    let th3 = std::thread::spawn(||{
        let my_card = 3;
        card_exchange(ex3, my_card);
    });

    th1.join().unwrap();
    th2.join().unwrap();
    th3.join().unwrap();
}

fn card_exchange(ex : Exchanger<i32>, my_card: i32) ->(){
    let mut received = ex.exchange(my_card);
    println!("My card is {}, received {}",my_card,received);
    while received != my_card{
        received = ex.exchange(received);
        println!("My card is {}, received {}",my_card,received);
    }
    println!("Received {}",my_card);
}
