use std::char;

/**************************************
 * 
 *  Simple Version - time 1.731 sec
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
 *  Accumulated State Version -  time 1.099 sec
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
 *  Fast  Version - lot of memory needed BUT TIME 0.307 sec
 * 
***************************************/
fn rusthero_fast(input: String) -> String {

    let mut buffer: [char; 500000] = ['0'; 500000];
    let mut size = 0;

    let mut output = String::from("");
    let mut counter = 0;
    let mut last_char  = '0';

    for c in input.chars() { 
        if last_char == c || last_char == '0' {
            counter +=1;
        } else {
            buffer[size] = char::from_digit(counter, 10).unwrap();
            buffer[size+1] = last_char;
            counter = 1;              
            size +=2;
        }
        last_char = c;
               
    }
    output.push_str(&counter.to_string());
    output.push_str(&last_char.to_string());  
    buffer[size] = char::from_digit(counter, 10).unwrap();
    buffer[size+1] = last_char;
    counter = 1;  
    size +=2;

    return buffer[0 .. size].into_iter().collect();
}


fn main() {

    let input = String::from("22164224441");
    let mut value = input.clone();

    let mut max = 0;

    for _x in 0..40 {
        
         value = rusthero_fast(value);
        // value = rusthero_accumulator_state(value);
        //value = rusthero_simple(value);

        if (max < value.len()){
            max = value.len();
        }
    } 

    println!("max needed buffer size {}",max);
    println!("final value length: {} of input {}",value.len(),input);

}

