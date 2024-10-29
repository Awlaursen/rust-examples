fn main() {
    {
        // This block will compile
        let s1 = String::from("hello");

        // Function takes an immutable reference
        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }

    {
        // This block will not compile
        let s = String::from("hello");

        // Function takes an immutable reference
        // but tries to mutate the borrowed data
        change(&s);
    }

    {
        // This block will not compile
        let mut s = String::from("hello");

        // There can be only one mutable reference to a
        // particular piece of data at a time
        let r1 = &mut s;
        let r2 = &mut s;

        println!("{}, {}", r1, r2);
    }

    {
        // This block will compile
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.
    
        let r2 = &mut s;
    
    }

    {   
        // This block will compile
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // variables r1 and r2 will not be used after this point
    
        let r3 = &mut s; // no problem
        println!("{r3}");

        // Will not compile if we add the following line
        // println!("{r1} and {r2}");
    
    }
}

fn calculate_length(s: &String) -> usize {
    // reference is {read-only / immutable / borrowed}
    s.len() // returns the length of the string without needing ownership
}

fn change(some_string: &String) {
    // mutability is not allowed for borrowed references
    some_string.push_str(", world"); // This will not compile
}
