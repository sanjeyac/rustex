/**
 * Rust Hero Proptotype
 */



function assertEquals(expcted, value) {
    if (expcted == value) {
        console.log("  Test pass ", expcted);
    }
    else {
        console.log(" failed ", value, " expected ", expcted);
    }
}


function rusthero(input) {
    var output = "";
    var state = input[0];
    var counter = 0;
    for (var i = 0; i <= input.length; i++) {
        if (input[i] == state) {
            counter++;
        } else {
            output += counter + state;
            state = input[i];
            counter = 1;
        }
    }
    return output;
}


assertEquals(rusthero("1"), "11");
assertEquals(rusthero("2"), "12");
assertEquals(rusthero("11"), "21");
assertEquals(rusthero("31"), "1311");
assertEquals(rusthero("3211"), "131221");
assertEquals(rusthero("111223"), "312213");
assertEquals(rusthero("312213"), "1311221113");
                                  

function superhero(battletest){
    for (var i = 0; i < 40; i++) {

        var prev = battletest.length;
        battletest = rusthero(battletest);
        
        console.log(">>", battletest.length, "\t\t prev ",prev,"\t\t diff ",battletest.length - prev);    
    }
}


superhero("111223");

superhero("22164224441");