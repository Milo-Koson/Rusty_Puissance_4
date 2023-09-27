extern crate queues;
use queues::*;
use std::sync::Arc;

pub fn mainQueue(q: &mut Queue<isize>) {
    println!("Begin Queue usage");

    // Add some elements to it
    q.add(1);
    q.add(-2);
    q.add(3);

    // Check the Queue's size
    println!("Check size of queue: {}", q.size());  // 3

    // Remove an element
    let element = q.remove();  // Ok(1)
    //print_type_of(&element);
    println!(" value inside the queue is {:?}", &q );

    // Check the Queue's size
    println!("check size of queue: {}", q.size());  // 2

    // Peek at the next element scheduled for removal
    println!(" Peek (should -2) : {:?}", q.peek());  // Ok(-2)

    // Confirm that the Queue size hasn't changed
    println!("Size of queue: {0}", q.size());  // 2

    // Remove the remaining elements
    q.remove();  // Ok(-2)
    q.remove();  // Ok(3)

    // Peek into an empty Queue
    println!(" Peek (should error) : {:?}", q.peek());  // Raises an error

    // Attempt to remove an element from an empty Queue
    q.remove();  // Raises an error

}
