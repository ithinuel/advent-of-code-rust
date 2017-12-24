use std::io::stdin;
use std::collections::HashMap;

/// https://math.stackexchange.com/a/163101
/// 
/// function spiral(n)
///         k=ceil((sqrt(n)-1)/2)
///         t=2*k+1
///         m=t^2 
///         t=t-1
///         if n>=m-t then return k-(m-n),-k        else m=m-t end
///         if n>=m-t then return -k,-k+(m-n)       else m=m-t end
///         if n>=m-t then return -k+(m-n),k else return k,k-(m-n-t) end
/// end
fn spiral(n: i32) -> (i32, i32) {
    let k = (((n as f32).sqrt() - 1f32) / 2f32).ceil() as i32;
    let t = 2*k+1;
    let mut m = t*t;
    let t = t-1;
    if n>=(m-t) {
        return (k-(m-n), -k)
    } else {
        m = m-t;
    }
    if n>=(m-t) {
        return (-k, -k+(m-n));
    } else {
        m = m-t;
    }
    return if n>=(m-t) {
        (-k+(m-n), k)
    } else {
        (k, k-(m-n-t))
    }
}

fn main() {
    let mut string = String::new();
    stdin().read_line(&mut string).unwrap();
    let val: i32 = string.trim().parse().unwrap();
    
    let sz = (val as f32).sqrt().ceil() as i32; 
    let mut htable: HashMap<(i32, i32), i32> = HashMap::new();
    htable.insert((0, 0), 1);
    let mut current = 1;
    
    while current <= val {
        let coords = spiral((htable.len() + 1) as i32);
        let new = 
            htable.get(&(coords.0+1, coords.1)).unwrap_or(&0) +
            htable.get(&(coords.0+1, coords.1+1)).unwrap_or(&0) +
            htable.get(&(coords.0, coords.1+1)).unwrap_or(&0) +
            htable.get(&(coords.0-1, coords.1+1)).unwrap_or(&0) +
            htable.get(&(coords.0-1, coords.1)).unwrap_or(&0) +
            htable.get(&(coords.0-1, coords.1-1)).unwrap_or(&0) +
            htable.get(&(coords.0, coords.1-1)).unwrap_or(&0) +
            htable.get(&(coords.0+1,coords.1-1)).unwrap_or(&0);
        //println!("{:?}", htable);
        //println!("{}:{:?} {}", vtable.len() + 1, coords, new);
        htable.insert(coords, new);
        current = new;
    }
    println!("{}", current);
}
