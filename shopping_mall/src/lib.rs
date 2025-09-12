mod mall;
use std::collections::HashMap;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut biggest:Store = Store::new(HashMap::<String, Employee>::new(), 0);
    let mut max = 0;
    let floors = mall.floors.values();
    let mut n = String::new();
    for floor in floors {
        for (name, store) in &floor.stores {
            if max < store.square_meters {
                max = store.square_meters;
                n = name.to_string();
                biggest = store.clone()
            }
        }
    }

    (n, biggest)
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut highest_salary = 0.;
    let mut result:Vec<(&String, &Employee)> = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if highest_salary < employee.salary {
                    highest_salary = employee.salary;
                    result.clear();
                    result.push((name, employee))
                }
            }
        }
    }

    result
}

pub fn nbr_of_employees(mall: &Mall)->usize {
    let mut employees:usize = 0;
    let guards:usize = mall.guards.len();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (_name, _employee) in &store.employees {
                employees+=1;
            }
        }
    }
    employees+guards

}

pub fn check_for_securities(mall: &mut Mall, mut guards:HashMap<String,Guard>){
     let mut total = 0;
        let floors=mall.floors.values();
        for floor in floors{
            total+=floor.size_limit;
            
        }

       let guards_ln = mall.guards.len();
       println!("the number of guars in the mall: {:?}", guards_ln);
       println!("The total before: {:?}", total);

       total  -= guards_ln as u64 * 200 ; 
       println!("The total after: {:?}", total);


        while total  >= 200 {
            if let Some((key, value)) = guards.iter().next() {

                mall.guards.insert(key.to_string() , *value);

                let key = key.clone();

                guards.remove(&key);

            }
            total-=200;
        };
}


pub fn cut_or_raise(mall: &mut Mall){
    for floor in mall.floors.values_mut(){
        for store in floor.stores.values_mut(){
            for  employee in store.employees.values_mut(){
                let working = employee.working_hours.1-employee.working_hours.0;
                let amount = employee.salary/10.0;
                if working >=10{
                    employee.raise(amount);
                }else {
                    employee.cut(amount);
                }
                // println!("{:?}", working);
            }
        }
    }
}

