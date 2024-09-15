fn main(){

    let mut dato_a_factorizar = String :: new();
   
    println!("ingrese un numero a factorizar: ");
    std::io::stdin().read_line(&mut dato_a_factorizar).expect("failed to read line");
    
    let mut x: i128 = dato_a_factorizar.trim().parse().expect("input not an integer");


    let c = factorizar(x);

    println!("{}",c);

    fn factorizar(mut x : i128)-> f64{

        let mut y : f64 = x as f64;

        if y == 0.0 || y == 1.0
        {let y : f64 = 1.0; y} else{
        for i in(1..x).rev(){x = i * x}x as f64
        }}
    }
