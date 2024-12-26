pub fn ownership_and_borrowing() {
    // Ownership example
    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1; // ownership moves to s2
                 // println!("{}", s1);             // This would fail - s1 no longer owns the string
    println!("{}", s2); // This works fine

    // Multiple immutable references
    let text = String::from("immutable example");
    let ref1 = &text; // First immutable reference
    let ref2 = &text; // Second immutable reference
    let ref3 = &text; // Third immutable reference - no problem!
    println!("{}, {}, {}", ref1, ref2, ref3); // All references can be used

    // Single mutable reference
    let mut number = 42;
    let mut_ref = &mut number; // Single mutable reference
    *mut_ref += 1; // Modify the value through reference
    println!("Number is now: {}", mut_ref);

    // This demonstrates why you can't have multiple mutable references
    let mut value = String::from("hello");
    {
        let r1 = &mut value; // First mutable reference
        r1.push_str(" world");
        // let r2 = &mut value;        // This would fail - can't have two mutable references
        // let r3 = &value;            // This would also fail - can't have immutable reference
        // while mutable reference exists
        println!("{}", r1);
    } // r1 goes out of scope here

    // Now we can create a new reference
    let r4 = &value; // This works because r1 is no longer in scope
    println!("{}", r4);
}
