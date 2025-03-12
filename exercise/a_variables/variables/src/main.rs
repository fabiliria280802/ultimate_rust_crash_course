fn main() {
    main_part_1();
    main_part_2();
    main_extra_part_2_1();
    main_extra_part_2_2();
    main_extra_part_2_3();
    main_extra_part_2_4();
    main_extra_part_2_5();
}

//Part 1
fn main_part_1() {
/*
    Notes: let values in rust are unchangeablea unless you declare them as mutables (mut).

        why so?
            cause if you can ever change their value it improves on safety, concurrecy & Speed.

        - why safety?
            cause if you can change a value, you can change it to anything, even if it's not safe to do so.

        - why concurrency?
            cause if you can change a value, you can change it while it's being used by another thread.

        - why Speed?
            cause if you can change a value, the code is optimized in time.
    */
    let (missiles, ready) = (8,2);
    println!("Part 1: Firing {} of my {} missiles...", ready, missiles);
}

//Part 2
// we declare const before the function is called.
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main_part_2() {
/*
This next code will return a Error! cause values are set as let instend of let + mut:

    let (missiles, ready) = (8,2);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);
*/
   let mut missiles= STARTING_MISSILES;
   let ready = READY_AMOUNT;
   println!("Part 2:\nFiring {} of my {} missiles...", ready, missiles);
   missiles = missiles - ready;
   println!("{} missiles left", missiles);
}

//Extra chanlangues
/*
- Explicitly annotate the variables with the type `i32`
*/

/*
- Try binding the variables all at once on one line using a pattern (parentheses and commas) -- can you figure out where `mut` goes?
*/
fn main_extra_part_2_1() {
   let mut missiles: i32 = STARTING_MISSILES;
   let ready: i32 = READY_AMOUNT;
   println!("Part 2.1:\nFiring {} of my {} missiles...", ready, missiles);
   missiles = missiles - ready;
   println!("{} missiles left", missiles);
}
/*
- Can you figure out the correct type annotation when you assign them all in one line?  Hint: it will use the same sort of pattern as the variables and values.
*/
fn main_extra_part_2_2() {
    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Part 2.2:\nFiring {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
 }
/*
- Instead of changing missiles, use the value `missiles - ready` directly in the second `println!(...)`
    What warning does cargo emit when you run your program now? Can you fix it?
        it seems like warning: variable does not need to be mutable.

    Fixing points:
    - The `mut` keyword is not needed when you're not changing the value of a variable.
    - Yoy must need to declare especific variables with {name} on a println cause if no it will take the firsr declare value.
    - On println you can declare variables and make simple ecuations.
*/
fn main_extra_part_2_3() {
    let missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Part 2.3:\nFiring {} of my {} missiles...", ready, missiles);
    println!("{missiles} missiles left", missiles = missiles - ready);
 }
/*
- Add another variable to your program *but don't use it*.
  - What does cargo say when you run your program?
    warning: unused variable: `boom`
*/
fn main_extra_part_2_4() {
    let boom: bool = false;
    let mut missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Part 3:\nFiring {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
 }
/*
- Try modifying a constant in `main()` (for example, `READY_AMOUNT = 1`). What does the error look like?
    error: cannot assign twice to immutable variable `READY_AMOUNT`
*/
fn main_extra_part_2_5() {
    let mut missiles: i32 = STARTING_MISSILES;
    READY_AMOUNT = 1;
    let ready: i32 = READY_AMOUNT;
    println!("Part 3:\nFiring {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
 }