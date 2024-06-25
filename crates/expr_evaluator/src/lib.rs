pub mod expr;
mod lexer;
mod parser;

#[cfg(test)]
mod tests {
    use super::*;
    use expr::{ExprContext, Expression, LeafNode, Node, NodeType};    
    
    #[test]
    fn test1() {
        let mut expr = Expression::new();
        expr.parse_expr(String::from("5 + 4 * 2 / 16 - 1")).unwrap();
        
        match expr.eval(){
            Ok(v) => {println!("result = {:?}", v); assert!(v == 4.5);},
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    pub fn sum(values: Vec<f64>) -> expr::Result<f64>{
        let mut sum = 0.0;
        for v in values.iter(){
            sum += v;
        }
        return Ok(sum);
    }

    #[test]
    fn test2() {
        let mut ctx = ExprContext::new();
        ctx.set_var(String::from("x"), 5.0);
        ctx.set_func(String::from("sum"), sum);

        let mut expr = Expression::new();
        expr.parse_expr(String::from("sum(x, 3)")).unwrap();
        expr.set_context(ctx);        
        println!("context: {:#?}", expr.context);

        match expr.eval(){
            Ok(v) => {println!("result = {:?}", v); assert!(v == 8.0);},
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn test3() {
        let mut expr = Expression::new();
        expr.parse_expr(String::from("(5 + 4) * (3 - 1)")).unwrap();
        
        match expr.eval(){
            Ok(v) => {println!("result = {:?}", v); assert!(v == 18.0);},
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn test4() {
        let mut ctx = ExprContext::new();
        ctx.set_var(String::from("x"), 5.0);
        ctx.set_var(String::from("y"), 13.0);
        ctx.set_func(String::from("sum"), sum);

        let mut expr = Expression::new();
        expr.parse_expr(String::from("sum(x, y) / 2 * 7")).unwrap();
        expr.set_context(ctx);        
        println!("context: {:#?}", expr.context);

        match expr.eval(){
            Ok(v) => { println!("result = {:?}", v); assert!(v == 63.0);},
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn test5() {
        let mut ctx = ExprContext::new();
        ctx.set_var(String::from("x"), 5.0);
        ctx.set_var(String::from("y"), 7.0);
        ctx.set_var(String::from("z"), 11.0);
        ctx.set_func(String::from("sum"), sum);

        let mut expr = Expression::new();
        expr.parse_expr(String::from("- x - z + sum(x, y, z)")).unwrap();
        expr.set_context(ctx);        
        println!("context: {:#?}", expr.context);

        match expr.eval(){
            Ok(v) => { println!("result = {:?}", v); assert!(v == 7.0); },
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn test6() {
        let mut ctx = ExprContext::new();
        ctx.set_var(String::from("x"), 16.0);        
        
        let mut expr = Expression::new();
        expr.parse_expr(String::from("sqrt(x)")).unwrap();
        expr.set_context(ctx.clone());
        println!("context: {:?}", expr.context);
        
        match expr.eval(){
            Ok(v) => { println!("result = {:?}", v); assert!(v == 4.0); },
            Err(e) => println!("An error ocurred: {:?}", e),
        }

        ctx.set_var(String::from("x"), 3.0);
        
        expr = Expression::new();
        expr.parse_expr(String::from("exp(x)")).unwrap();
        expr.set_context(ctx.clone());
        
        match expr.eval(){
            Ok(v) => { println!("result = {:?}", v); assert!(((v * 100.0).round()/100.0) == 20.09); },
            Err(e) => println!("An error ocurred: {:?}", e),
        }

        ctx.set_var(String::from("x"), 2.0);
        ctx.set_var(String::from("y"), 3.0);
        
        expr = Expression::new();
        expr.parse_expr(String::from("pow(x,y)")).unwrap();
        expr.set_context(ctx);
        
        match expr.eval(){
            Ok(v) => { println!("result = {:?}", v); assert!(v == 8.0); },
            Err(e) => println!("An error ocurred: {:?}", e),
        }
    }

    #[test]
    fn test_parallel() {
        std::thread::scope(|scope|{
            scope.spawn(||{
                println!("Hello from thread 1");
                test1();
            });

            scope.spawn(||{
                println!("Hello from thread 2");
                test2();
            });

            scope.spawn(||{
                println!("Hello from thread 3");
                test3();
            });
        });        
    }
}