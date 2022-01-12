#![allow(dead_code)] // 使ってない変数とかの警告消す

// 定数 明示的な型の指定が必要
const PI: f32 = 3.14159;

// 構造体
struct Human {
    name: String,
    age: u16,
    skill: Skill,
}

// 構造体、タプルでも作れるってよ
struct Location(i32, i32);

// 何もない構造体作れるってよ。意味わからん。
struct Marker;

// ジェネリクス
struct MyBox<A> {
    item: A,
}

// ジェネリクス
struct MyOptionBox<A> {
    item: Option<A>,
}

// enum
enum Skill {
    Fire,
    Thunder,
    Ice,
    Dark,
}

// mainメソッドはResultを返却できる
// fn main() {
fn main() -> Result<(), String> {
    println!("Hello, world!");

    // comment

    // 推論
    let x = 10;
    println!("{}", x);

    // 型指定
    let x: f64 = 3.14159; // 同じ変数名に複数回割り当てできる。おえっ。
    println!("number is {}", x);

    // 可変の値
    let non_mutable_value = 10;
    let mut mutable_value = 10;
    println!(
        "non_mutable_value {} mutable_value {}",
        non_mutable_value, mutable_value
    );

    // non_mutable_value = 10; // error
    mutable_value = 10;
    println!(
        "non_mutable_value {} mutable_value {}",
        non_mutable_value, mutable_value
    );

    // boolean
    let flg_on = true;
    let flg_off = false;
    println!("{} {}", flg_on, flg_off);
    println!("{} {}", flg_on as u8, flg_off as u8); // 所詮数値

    // 符号なし整数型
    let unsigned_u8 = 255u8;
    let unsigned_u32 = 4294967295u32;
    let unsigned_u64 = 18446744073709551615u64;
    let unsigned_u128 = 340282366920938463463374607431768211455u128;
    println!(
        "{} {} {} {}",
        unsigned_u8, unsigned_u32, unsigned_u64, unsigned_u128
    );

    // 符号付き整数型
    let signed_i8 = 127i8;
    let signed_i32 = 2147483647i32; // デフォルトではi32
    let signed_i64 = 9223372036854775807i64;
    let signed_i128 = 170141183460469231731687303715884105727i128;
    println!(
        "{} {} {} {}",
        signed_i8, signed_i32, signed_i64, signed_i128
    );

    // 違う数値型で足し算できない
    // let sum = signed_i8 + signed_i32;
    // as で変えてあげる
    let sum = signed_i8 as i64 + signed_i32 as i64;
    println!("{}", sum);

    // float
    let float_32 = 0.000000000000000000000000000000000000000000001f32;
    let float_64 = 0.000000000000000000000000000000000000000000001f64; // デフォルトではf64
    println!("{} {}", float_32, float_64);

    // tupple
    let tpl = (
        127i8,
        true,
        0.000000000000000000000000000000000000000000001f32,
    );
    println!("{} {} {}", tpl.0, tpl.1, tpl.2);

    // str
    let txt = "hello";
    println!("{}", txt);

    // const
    println!("{}", PI);

    // ary
    let nums: [i8; 4] = [1, 2, 3, 4];
    println!("{:?}", nums);
    println!("{}", nums[0]);

    // function
    println!("add {}", add(2147483647i32, 2147483647i32));

    let swap_rslt = swap(10, 100);
    println!("{} {}", swap_rslt.0, swap_rslt.1);

    let (swap_rslt_1, swap_rslt_2) = swap(10, 100);
    println!("{} {}", swap_rslt_1, swap_rslt_2);

    let do_nothing_rslt = do_nothing();
    println!("{:?}", do_nothing_rslt);

    // if
    let if_sample = 10;
    if if_sample == 0 {
        println!("if_sample is 0");
    } else if if_sample == 2 {} else {
        println!("if_sample is number");
    }

    // loop
    let mut count = 0;
    loop {
        count += 1;
        println!("count is {}", count);
        if count == 5 {
            break;
        }
    }
    let mut count = 0;
    let rslt_str = loop {
        count += 1;
        if count == 10 {
            break "10になりましたで"; // returnみたいなんできる
        }
    };
    println!("{}", rslt_str);

    // while
    let mut count = 0;
    while count < 5 {
        count += 1;
        println!("count is {}", count);
    }

    // for
    for x in 0..5 {
        println!("for {}", x);
    }
    for x in 0..=5 {
        println!("for {}", x);
    }

    // match
    let x = 55;
    match x {
        0 => {
            println!("match is 0");
        }
        1 | 2 => {
            println!("match is 1 or 2");
        }
        3..=4 => {
            println!("match is 3..=4");
        }
        catch_x @ 5..=100 => {
            println!("match is 5..=100 catch {}", catch_x);
        }
        _ => {
            println!("no match");
        }
    }

    // さんこうしき
    let x = 10;
    println!("さんこうしき {}", if x > 5 { "x>5" } else { "aaaaaaaaaaa" });

    // staticメソッドの呼び出し
    let x = String::from("create by String static method");
    println!("{}", x);

    // インスタンスメソッドの呼び出し
    println!("{}", x.len());

    // 構造体作成
    let hosono = Human {
        name: String::from("hosono"),
        age: 24,
        skill: Skill::Dark,
    };
    println!("human name:{} age:{}", hosono.name, hosono.age);

    let tokyo = Location(32, 32);
    println!("tokyo rad:{} red:{}", tokyo.0, tokyo.1);

    let _m = Marker;

    // enum - match
    match hosono.skill {
        Skill::Fire => println!("hosono use Fire!!!"),
        Skill::Ice => println!("hosono use Ice!!!"),
        Skill::Thunder => println!("hosono use Thunder!!!"),
        Skill::Dark => println!("hosono use Dark!!!"),
    }
    // TODO　これ調べる。この書き方なんかめっちゃキモいことする。
    match hosono.skill {
        Fire => println!("hosono use Fire!!!"),
        Ice => println!("hosono use Ice!!!"),
        Thunder => println!("hosono use Thunder!!!"),
        Dark => println!("hosono use Dark!!!"),
    }

    // ジェネリクス生成
    let i32box = MyBox::<i32> { item: 30i32 };
    let suiron = MyBox { item: 3.14 };
    let nest = MyBox {
        item: MyBox { item: false },
    };
    // let like_null = MyBox { item: None }; できない

    // Option None | Some
    let option_box = MyOptionBox::<u8> { item: None };
    if option_box.item.is_none() {
        println!("option_box is None");
    }
    let option_box = MyOptionBox { item: Some(8u8) };
    if option_box.item.is_some() {
        println!("option_box is Some");
    }
    match option_box.item {
        None => println!("match option_box is None"),
        Some(v) => println!("match option_box is Some {}", v),
    }

    // Result
    match give_me_0(0) {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{}", v),
    }
    match give_me_0(1) {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{}", v),
    }
    let rslt = give_me_0(0)?;
    println!("Result use ? {}", rslt);
    // let rslt = give_me_0(1)?; // ここでreturnしちゃう
    // println!("Result use ? {}",rslt); 

    // Vector
    let mut i32_vector = Vec::new();
    i32_vector.push(12);
    i32_vector.push(9);
    println!("i32_vector : {:?}", i32_vector);
    let string_vector = vec![String::from("1"), String::from("2")];
    println!("string_vector : {:?}", string_vector);

    // mainメソッドはResultを返却できる
    let code = 0;
    if code == 0 {
        Ok(())
    } else {
        Err(String::from("main method err"))
    }
}

// 関数
fn add(x: i32, y: i32) -> i64 {
    // ブロックの最後のセミコロンが無い式は戻り値として認識される
    // return書いたら警察に捕まりそうだね
    x as i64 + y as i64
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    // returnもできるってよ
    return (y, x);
}

fn do_nothing() {}

// OK
fn rslt_str_func_1() -> String {
    return String::from("rslt string");
}

// FUCK
fn rslt_str_func_2() -> &'static str {
    return "rslt string";
}

fn give_me_0(x: u8) -> Result<String, String> {
    if x == 0 {
        Ok(String::from("it is 0"))
    } else {
        Err(String::from("it is not 0"))
    }
}