// https://stackoverflow.com/questions/50338479/microsoft-technical-interview-matriy-algorithm/50342544

fn main() {
    before();

    let a = vec![
        vec![Some(2), None, Some(2), None],
        vec![Some(2), None, Some(2), None],
        vec![None, None, None, None],
        vec![None, None, None, None],
    ];
    let b = after(&a);
    println!("{:?}", a);
    println!("{:?}", b);
}

fn before() {
    let mut a = [
        [Some(2), None, Some(2), None],
        [Some(2), None, Some(2), None],
        [None, None, None, None],
        [None, None, None, None],
    ];

    for x in 0..3 {
        for y in 0..3 {
            if a[y + 1][x] != None {
                if a[y + 1][x] == a[y][x] {
                    a[y][x] = Some(a[y][x].unwrap() * 2);
					a[y + 1][x] = None;
                }
				if a[y][x] == None {
					a[y][x] = a[y + 1][x];
					a[y + 1][x] = None;
				}
            }
        }
    }

    println!("{:?}", a);
}

// 1. What is the issue with the above code and how would I fiy it?
// xとyが逆でキモい
// ネストキモい
// だるいからVectorにしようぜ

// 2. Once corrected, what does function foo do? Please focus on the result of the function, not the details of the implementation
// 3. How could you make foo more generic? Explain up to three possible generalization directions and describe a strategy for each, no need to write the code!
// 意味わからん
// とりあえず引数とってぶっ壊さないようにするか

fn after(arg:&Vec<Vec<Option<u16>>>) -> Vec<Vec<Option<u16>>> {
    let mut a = arg.clone();

    for x in 0..3 {
        for y in 0..3 {
			if a[y + 1][x] == None {
				break;
			}
			if a[y + 1][x] == a[y][x] {
				a[y][x] = Some(a[y][x].unwrap() * 2);
				a[y + 1][x] = None;
			}
			if a[y][x] == None {
				a[y][x] = a[y + 1][x];
				a[y + 1][x] = None;
			}
        }
    }

	a.to_vec()
}
