fn main() {
    println!("Hello, world!");
    {
        let x = 5;
        let (x, y) = (1, 2);
        let mut x: i32 = 4; //xはシャドーイングされた.
        x = 10;
        let z: i8 = 15;
        println!("z = {}", z);
        // print_sum(x, y); これはエラー
        print_sum(y, z);
    }
    {
        let mut x: i8 = 1;
        x = 7; //イミュータブルなので変更可能.
        let x = x; //ミュータブルになって7に束縛される.

        let y = 4;
        let y = "string"; //シャドーイングは型すら変える.
        print_number(x);
        let z = 10;
        println!("{} add_one -> {}", z, add_one(z));
        println!("{} add_two -> {}", z, add_two(z));

    }
    {
        //let i: String = diverges();
        //println!("{}", i);
    }
    {
        let f: fn(i32) -> i32;
        let f: fn(i32) -> i32 = add_one;
        let i = 4;
        println!("function pointer f = {}", f(i));
    }
    {
        let c = 'c';
        //Rustのchar型は4Byte.
        let two_hearts = '💕';
        let x = 42; // xはi32型を持つ
        let y = 1.0; // yはf64型を持つ
        //配列.
        // 配列は[T; N]という型を持つ.
        let a = [1, 2, 3]; // a: [i32; 3]
        let mut m = [1, 2, 3]; // m: [i32; 3]
        println!("a has {} elements", a.len());
        println!("m second value is {}", m[1]);
    }
    {
        //スライス.
        let a = [0, 1, 2, 3, 4];
        let complete = &a[..]; // aに含まれる全ての要素を持つスライス
        let mut middle = &a[1..4]; // 1、2、3のみを要素に持つaのスライス
        println!("スライス テスト{}", a[2]);
        //スライスは他のデータ構造への参照らしいのでこんなこともできるのか?
        //middle[1] = 10;
        //println!("スライス テスト{}", a[2]);
    }
    {
        //タプル.
        let x = (1, "hello");
        let y: (i32, &str) = (1, "hello");

        (0,); // 1要素のタプル
        (0); // 丸括弧に囲まれたゼロ

        let tuple = (1, 2, 3);
        let a = tuple.0;
        let b = tuple.1;
        let c = tuple.2;
        println!("a is {}", a);
    }

    {
        //if文
        //一般的なもの
        let x = 6;
        if x == 5 {
            println!("x は 5 です!");
        } else if x == 6 {
            println!("x は 6 です!");
        } else {
            println!("x は 5 でも 6 でもありません :(");
        }
    }
    {
        //if文
        //なんやこら
        let x = 5;
        let y = if x == 5 {
            10
        } else {
            15
        }; // y: i32
    }
    {
        //if文
        //慣れたらそうでもない
        let x = 5;
        let y = if x == 5 { 10 } else { 15 }; // y: i32
    }
    {
        //無限ループ
        // loop {
        //     println!("Loop forever!");
        // }
    }
    {
        //while
        let mut x = 5; // mut x: i32
        let mut done = false; // mut done: bool
        while !done {
            x += x - 3;
            println!("{}", x);
            if x % 5 == 0 {
                done = true;
            }
        }
        // while true {}もできる.
        // けどloop使えよって話...
    }
    {
        //for文
        // for (x = 0; x < 10; x++) {
        //     printf( "%d\n", x );
        // }
        //今までと同じようには書けない.

        for x in 0..10 {
            println!("{}", x); // x: i32
        }
    }

    {
        // ループでbreakやcontinueを使う際にラベルを指定できる.
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x % 2 == 0 { continue 'outer; } // x のループを継続
                if y % 2 == 0 { continue 'inner; } // y のループを継続
                println!("x: {}, y: {}", x, y);
            }
        }
    }

    {
        // 列挙型.
        enum Message {
            Quit,
            ChangeColor(i32, i32, i32),
            Move { x: i32, y: i32 },
            Write(String),
        }
        let x: Message = Message::Move { x: 3, y: 4 };
        enum BoardGameTurn {
            Move { squares: i32 },
            Pass,
        }
        let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
    }

    {
        // マッチ.
        let x = 5;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            4 => println!("four"),
            5 => println!("five"),
            _ => println!("something else"),
        }
    }

    {
        // パターン.
        let x = 'x';
        let c = 1;
        // cがxにマッチするなら、ではない。
        match c {
            // cの値をxという名前で束縛.
            x => println!("x: {} c: {}", x, c),
        }
        let a = 1;
        match a {
            // 複合パターン.
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            // 範囲.
            4 ... 5 => println!("four or five"),
            // 範囲に名前を付けて.
            e @ 6 ... 10 => println!("a is {}", e),
            _ => println!("anything"),
        }
        let b = 10;
        match b {
            // ガード
            x if x > 50 => println!("guard = {}", x),
            // ifは1と2両方に適応.
            1 | 2 if true => println!("one or tow"),
            // (1 | 2) if true => ... ；こう.
            // 1 | (2 if true) => ... ；違う.
            _ => println!("non"),
        }
    }

    {
        //メソッド
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl Circle {
            fn reference(&self) {
                println!("taking self by reference!");
            }
            fn mutable_reference(&mut self) {
                println!("taking self by mutable reference!");
            }
            fn takes_ownership(self) {
                println!("taking ownership of self!");
            }
        }
        // circle.hogehoge()で呼び出せる.
        // ( ref,method() )

        // 関連関数.
        impl Circle {
            // selfを引数として取らない.
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle {
                    x: x,
                    y: y,
                    radius: radius,
                }
            }
        }
        // 他の言語では静的メソッドと呼ばれる.
        let c = Circle::new(0.0, 0.0, 2.0);
        // ( Struct::function() )
    }

    {
        // Bilderパターン.
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
        struct CircleBuilder {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl CircleBuilder {
            fn new() -> CircleBuilder {
                CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
            }
            fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
                self.x = coordinate;
                self
            }
            fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
                self.y = coordinate;
                self
            }
            fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
                self.radius = radius;
                self
            }
            fn finalize(&self) -> Circle {
                Circle { x: self.x, y: self.y, radius: self.radius }
            }
        }
        let c = CircleBuilder::new()
                    .x(1.0)
                    .y(2.0)
                    .radius(2.0)
                    .finalize();
        println!("area: {}", c.area());
        println!("x: {}", c.x);
        println!("y: {}", c.y);
    }

    {
        // ベクタ.
        let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
        let w = vec![0; 10]; // 0が10個
        // 今更だけどprintln!だったりvec!というマクロは[]でも()でもいいらしい.
        // 分かりやすさ敵にvec!のときは[]が使われてそう.
        println!("The third element of v is {}", v[2]);

        //アクセスのためのインデックスはusize型でなければならない.
        let i: usize = 0;
        let j: i32 = 0;
        // v[i]; ok
        // v[j]; ng
    }

    {
        // ベクタのいてレーティング.
        let mut v = vec![1, 2, 3, 4, 5];
        for i in &v {
            println!("A reference to {}", i);
        }
        for i in &mut v {
            println!("A mutable reference to {}", i);
        }
        for i in v {
            println!("Take ownership of the vector and its element {}", i);
        }
    }

    {
        // 文字列.
        // &strは文字列スライスと呼ばれる.
        // 固定サイズで(サイズの?)変更不可.
        let greeting = "Hello there."; // greeting: &'static str
        // 改行と空白を含む.
        let s = "foo
        bar";
        assert_eq!("foo\n        bar", s);
        // 改行と空白を削る
        let s = "foo\
            bar";
        assert_eq!("foobar", s);

        // Stringというヒープアロケートされる文字列もある.
        let mut s = "Hello".to_string(); // mut s: String
        println!("{}", s);

        s.push_str(", world.");
        println!("{}", s);

        // &strに型強制.(キャスト)
        let slice_s = &s;
        // &strをStringに変換するとメモリアロケーションが発生するのであまりやらないほうがいい.
        let mut s = "Hello".to_string();
    }
    {
        // インデクシング.
        let hachiko = "忠犬ハチ公";
        for b in hachiko.as_bytes() {
            print!("{}, ", b);
        }
        println!("");
        for c in hachiko.chars() {
            print!("{}, ", c);
        }
        println!("");
        //出力される数が異なるので注意.

        //スライシング.
        let dog = "hachiko";
        let hachi = &dog[0..5];

        //(注意)オフセットはバイトであり、文字のオフセットではない.
        // let hachi = &hachiko[0..2]; error
    }
    {
        // 文字列の連結.
        // Stringのが存在すれば, &strを末尾に連結可能.
        let hello = "Hello ".to_string();
        let world = "world!";
        let hello_world = hello + world;
        // Stringに対しては&を付けて型強制させるといい.
        let hello = "Hello ".to_string();
        let world = "world!".to_string();
        let hello_hello_world = hello + &world;
    }

    {
        // トレイト.
        // プロトタイプ宣言のようであり、namespaceのようだけど結構違う.
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        trait HasArea {
            fn area(&self) -> f64;
        }
        impl HasArea for Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }

        // ジェネリック関数におけるトレイト境界.

        // ジェネリック関数を用いてもTにarea()が実装されているか分からないので、エラーになる.
        // fn print_area<T>(shape: T) {
        //     println!("This shape has an area of {}", shape.area());
        // }
        // TはHasAreaトレイトを実装するあらゆる型という意味を持つので、実行可能.
        fn print_area<T: HasArea>(shape: T) {
            println!("This shape has an area of {}", shape.area());
        }
    }

    {
        // トレイトはプリミティブ型のメソッドとしても実装できる.
        // あんまりやらないほうが良い.
        trait HasArea {
            fn area(&self) -> f64;
        }
        impl HasArea for i32 {
            fn area(&self) -> f64 {
                println!("this is silly");
                *self as f64
            }
        }
        5.area();
    }

    {
        // 標準ライブラリてきな?
        // トレイト Drop.
        struct Firework {
            strength: i32,
        }
        impl Drop for Firework {
            fn drop(&mut self) {
                println!("BOOM times {}!!!", self.strength);
            }
        }
        {
            let firecracker = Firework { strength: 1 };
            let tnt = Firework { strength: 100 };
        }
    }

}




//引数の型宣言は必須.
fn print_number(x: i8){
    println!("number is {}", x);
}
fn print_sum(x: i8, y: i8){
    println!("sum is {}", x+y);
}
fn add_one(x: i32) -> i32{
    //セミコロンを付けない.
    x + 1

    /*
    これはRustが式と文で出来ていることを表している
    式は値を返すが、文は値を返さない.
    式: x + 1
    文: x + 1;
    式は値を返すので、戻り値として値を返すことができる.
    */
}
fn add_two(x: i32) -> i32{
    // returnも書ける.
    return x + 2;

    //実行されない.
    x + 10
}
fn diverges() -> ! {
    // panic!はprintln!と同様にマクロ.
    // panic!は実行中の現在のスレッドを与えられたメッセージとともにクラッシュさせる.
    // クラッシュさせるので値を返さない.
    // そのためこの関数の戻り値は"!".
    // "!"は"発散する(diverges)"と読む.
    panic!("This function never returns!");
}