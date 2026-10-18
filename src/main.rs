fn main() {
    let x = 5;
    let mut y = 5;
    let z = y;
    y = 6;
    const MAX: i32 = 10;
    println!("Variable Inmutable (final int) x = {x} \nVariable Mutable (int) 1º y = {z} 2º y = {y} \nConstante, Siempre Inmutable y tipado (i32) MAX = {MAX}\n");

    let txt = "   ";
    let txt = txt.len();
    println!("'let' permite dos variables con el mismo nombre, y eso hace que cambie el tipo de la variable. \n'txt' al principio es un string, pero ahora es un index de colecciones txt = {txt}\n");


    
    let cond = true;
    let num = if cond { 5 } else { 6 };
    println!("Como el 'if' es una expresión, se puede asignar con '=' a una variable (ternario de java '? :') \nBooleano cond = {cond}, if = {num}\n");
    
    let n = suma(x, y);
    println!("{n}\n")
}

fn suma(a: i32, b: i32) -> i32 {
    println!("Funciones: se declaran con 'fn nombre(arg1: tipo...) -> tipo de retorno', lo que en java equivale a 'public static tipo nombre(tipo arg1...)'. El valor devuelto es la última expresión sin ';'. Se puede poner return para salir antes de tiempo.");
    a + b
}
