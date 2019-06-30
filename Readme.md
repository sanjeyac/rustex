First Application in Rust for the RUSTHERO contest

A first quick prototype have been done in JS in the file prototype/main.js

After that the prototype has been implemented in Rust

```
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
```

after this a solution using  `fold` has been implemented

```
fn rusthero(input: String) -> String {

    let empty_accumulator = AccumulatorState::new();

    let mut result = input
            .chars()
            .fold( empty_accumulator , | acc, value | acc.accumulate(&value) );

    result.add_to_output();

    return result.as_string();
}
```