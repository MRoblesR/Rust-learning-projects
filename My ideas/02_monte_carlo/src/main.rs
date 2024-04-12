//In this case I saw that with this crate you can define the probability of each element to happen
//I think in this case this is reemplazable by just the rand crate
use weighted_rand::builder::WalkerTableBuilder;
use weighted_rand::builder::NewBuilder; 


fn main() {
    //This is the part of the random number generation
    let index_weights = [1, 1, 1];
    let builder = WalkerTableBuilder::new(&index_weights);
    let wa_table = builder.build();

    //Counting 
    let mut without_change:usize = 0;
    let mut with_change:usize = 0;
    
    for _ in 1..i16::MAX{
        //We first define the doors, witch is a vector of under 3 elements composed initially of 2 goats and 1 car
        //I would like to include this 2 lines on the generate_doors method, but I had some problems with borrowing.
        let doors: &mut Vec<String> = &mut Vec::new();
        let pos = wa_table.next();

        //We generate the list, randomly ordered
        generate_doors(doors, pos);

        //Choose and remove one of the doors
        let doors = choose_first_door(doors);

        //Evaluate in which door it was
        if reveal_car_door_with_change(doors){
            with_change+=1;
        }
        if reveal_car_door_without_change(doors){
            without_change+=1;
        }
    }
    //Print the results
    print!("Without changing: {}", without_change);
    print!("Changing: {}", with_change);
}

fn generate_doors(doors: &mut Vec<String>, pos: usize) {
    doors.push(String::from("goat"));
    doors.push( String::from("goat"));
    doors.insert(pos, String::from("car"));
}


fn choose_first_door(input_doors: &mut Vec<String> ) -> &mut Vec<String>{
    //The removed door must always be a "goat", in case both of them are a goat one is randomly picked
    if input_doors[1].eq("goat") && input_doors[2].eq("goat"){
        if rand::random(){
            input_doors.remove(2);
        }else{

            input_doors.remove(1);
        }
    }else {
        if input_doors[1].eq("goat"){
            input_doors.remove(1);
        }else{
            input_doors.remove(2);
        }
    }
    
    return input_doors;
}

fn reveal_car_door_without_change(input_doors: &mut Vec<String>)->bool{
    return input_doors[0].eq("car");
}
fn reveal_car_door_with_change(input_doors: &mut Vec<String>)->bool{
    return input_doors[1].eq("car");
}