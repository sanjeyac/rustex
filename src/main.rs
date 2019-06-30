use std::char;
use std::collections::LinkedList;


/**
 * Accumulator state
 * 
 * After each character has been read
 * if the character is equals to character in the state
 *      then it will increment the counter,
 *      else the character will increment the list and the counter will be reset
 * 
 * buffer is the final output
 */
struct AccumulatorState {
    last_letter: char,
    count: u8,
    buffer: LinkedList<char> 
}


impl AccumulatorState {

    fn new () -> AccumulatorState {
        return AccumulatorState { last_letter: '0', count: 0, buffer: LinkedList::new() };
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
        let count_as_char = char::from_digit(self.count.into(), 10).unwrap();
        self.buffer.push_back(count_as_char);
        self.buffer.push_back(self.last_letter);     
    }

    fn as_string(&mut self) -> String {
        let final_string = self.buffer.clone().into_iter().collect();
        return final_string;
    }

}



/**
 * 
 */
fn rusthero(input: String) -> String {

    let empty_accumulator = AccumulatorState::new();

    let mut result = input
            .chars()
            .fold( empty_accumulator , | acc, value | acc.accumulate(&value) );

    result.add_to_output();

    return result.as_string();
}


/**
 * MAIN
 */
fn main() {    
    let mut value =  String::from("22164224441");
    for _x in 0..40 {        
         value = rusthero(value);
    }    
    println!("final value length: {}",value.len());
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let result = rusthero("1".to_string());
        assert_eq!( result , "11" );
    }

    #[test]
    fn test_11() {
        let result = rusthero("11".to_string());
        assert_eq!( result , "21" );
    }

    #[test]
    fn test_31() {
        let result = rusthero("31".to_string());
        assert_eq!( result , "1311" );
    }

    #[test]
    fn test_3211() {
        let result = rusthero("3211".to_string());
        assert_eq!( result , "131221" );
    }

    #[test]
    fn test_111223() {
        let result = rusthero("111223".to_string());
        assert_eq!( result , "312213" );
    }        


    #[test]
    fn test_312213() {
        let result = rusthero("312213".to_string());
        assert_eq!( result , "1311221113" );
    }        
}