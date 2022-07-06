use exchanger::Exchanger;

#[test]
pub fn test_simple_exchange() -> (){
    let ex1 = Exchanger::new();
    let ex2 = ex1.clone();

    let handle = std::thread::spawn(move ||{
        return ex1.exchange("Ciao".to_string());
    });

    let handle2 = std::thread::spawn(move ||{
        return ex2.exchange("Bello".to_string());
    });
    let rec = handle.join().unwrap();
    let rec2 = handle2.join().unwrap();
    assert_eq!(rec,"Bello".to_string());
    assert_eq!(rec2,"Ciao".to_string());
}

#[test]
pub fn test_simple_double() -> (){
    let ex1:Exchanger<String> = Exchanger::new();
    let ex2 = ex1.clone();

    let handle = std::thread::spawn(move ||{
        println!("1 Calling excahge");
        let rec = ex1.exchange("Ciao".to_string());
        println!("1 got {}",rec);
        return ex1.exchange(rec);
    });

    let handle2 = std::thread::spawn(move ||{
        println!("2 Calling excahge");
        let rec2 = ex2.exchange("Bello".to_string());
        println!("2 got {}",rec2);
        return ex2.exchange(rec2);
    });
    let rec = handle.join().unwrap();
    let rec2 = handle2.join().unwrap();
    assert_eq!(rec2,"Bello".to_string());
    assert_eq!(rec,"Ciao".to_string());
}


#[test]
fn exchange_a_slice() {
    let exchanger = Exchanger::new();

    static MY_VEC1: [i32; 3] = [1, 2, 3];
    let e1 = exchanger.clone();
    let t1 = std::thread::spawn(move || {
        let my_slice1 = &MY_VEC1[..];
        e1.exchange(my_slice1)
    });

    static MY_VEC2: [i32; 3] = [4, 5, 6];
    let e2 = exchanger.clone();
    let t2 = std::thread::spawn(move || {
        let my_slice2 = &MY_VEC2[..];
        e2.exchange(my_slice2)
    });

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();
    assert_eq!(r1, [4,5,6]);
    assert_eq!(r2, [1,2,3]);
}

#[test]
fn card_exchange_test() ->() {
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
        received = ex.exchange(my_card);
        println!("My card is {}, received {}",my_card,received);
    }
    println!("Received {}",my_card);
}