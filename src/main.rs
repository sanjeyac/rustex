use std::char;
use std::collections::LinkedList;

/**************************************
 * 
 *  Simple Version 
 * 
 *  time 1.731 sec
 *  
 *  with compiler optimization
 *  time 0.127 sec
 * 
***************************************/
fn rusthero_simple(input: String) -> String {

    let mut output = String::from("");
    let mut counter = 0;
    let mut last_char  = '0';

    for c in input.chars() { 

        if last_char == c || last_char == '0' {
            counter +=1;
        } else {
            output.push_str(&counter.to_string());
            output.push_str(&last_char.to_string()); 
            counter = 1;              
        }
        last_char = c;
    }
    output.push_str(&counter.to_string());
    output.push_str(&last_char.to_string());  

    return output;

}



/**************************************
 * 
 *  Accumulated State Version -  
 * 
 *  time 1.099 sec
 * 
 *  with compiler optimization
 *  time 0.097 sec
 * 
***************************************/

struct AccumulatorState {
    last_letter: char,
    count: u8,
    output: String
}


impl AccumulatorState {

    fn new () -> AccumulatorState {
        return AccumulatorState { last_letter: '0', count: 0, output: String::new() };
    }

    fn accumulate(mut self, current_letter: &char) -> AccumulatorState {       

        if *current_letter == self.last_letter || self.last_letter == '0'{
            self.count += 1;
        }else{   
            self.add_to_output();
            self.count = 1;
        }    
        self.last_letter = *current_letter;
        return self;
    }

    fn add_to_output(&mut self) {
        self.output.push_str(&self.count.to_string());
        self.output.push(self.last_letter);         
    }

}


fn rusthero_accumulator_state(input: String) -> String {

    let empty_accumulator = AccumulatorState::new();

    let mut result = input
            .chars()
            .fold( empty_accumulator , | acc, value | acc.accumulate(&value) );

    result.add_to_output();

    return result.output;
}



/**************************************
 * 
 *  BEST SOLUTION SO FAR
 * 
 *  Fast  Version - 
 * 
 *  time 0,407
 *  
 *  with compiler optimazation
 *  time 0.087 sec
 * 
 * 
 * we could use an array instead of a list to improve performance
 * but then we need to manage array re-allocation
 * when the capacity of the array is close to the end
 * 
***************************************/
fn rusthero_fast(input: String) -> String {


    let mut buffer: LinkedList<char> = LinkedList::new(); // append with O(1)
    let mut size = 0;

    let mut output = String::from("");
    let mut counter = 0;
    let mut last_char  = '0';

    for c in input.chars() { 
        if last_char == c || last_char == '0' {
            counter +=1;
        } else {
            buffer.push_back(char::from_digit(counter, 10).unwrap());
            buffer.push_back(last_char);
            counter = 1;              

        }
        last_char = c;
               
    }
    buffer.push_back(char::from_digit(counter, 10).unwrap());
    buffer.push_back(last_char);

    return buffer.into_iter().collect();
}

// rustc  main.rs
fn main() {

    let input = String::from("22164224441");
    let mut value = input.clone();

    let mut max = 0;

    for _x in 0..40 {
        
         value = rusthero_fast(value);
        // value = rusthero_accumulator_state(value);
        // value = rusthero_simple(value);

        if (max < value.len()){
            max = value.len();
        }
    } 

    println!("max needed buffer size {}",max);
    println!("final value length: {} of input {}",value.len(),input);

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let result = rusthero_fast("1".to_string());
        assert_eq!( result , "11" );
    }

    #[test]
    fn test_11() {
        let result = rusthero_fast("11".to_string());
        assert_eq!( result , "21" );
    }

    #[test]
    fn test_31() {
        let result = rusthero_fast("31".to_string());
        assert_eq!( result , "1311" );
    }

    #[test]
    fn test_3211() {
        let result = rusthero_fast("3211".to_string());
        assert_eq!( result , "131221" );
    }

    #[test]
    fn test_111223() {
        let result = rusthero_fast("111223".to_string());
        assert_eq!( result , "312213" );
    }        


    #[test]
    fn test_312213() {
        let result = rusthero_fast("312213".to_string());
        assert_eq!( result , "1311221113" );
    }        
}