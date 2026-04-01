#[test]
fn test_day01(){
    pub fn day01() {
        // 不可变变量
        let a: i32 = 1;
        println!("let 不可变变量 {}", a);
        // 不可变变量只能赋值一次
        let b: i32;
        b = 2;
        println!("let 不可变变量只能赋值一次 {}", b);
        // 可变变量
        let mut c: i32 = 3;
        c = 4;
        c = 5;
        println!("let 可变变量可赋值多次 {}", c);
        // 常量：常量不能修改
        const D: i32 = 6;
        println!("const 常量 {}", D);
        // 遮蔽
        let a = 7;
        println!("let 遮蔽 {}", a);
        // 标量：单个数据
        let a: i32 = 8;
        //长度	有符号	无符号
        //8-bit	i8	    u8
        //16-bit	i16	    u16
        //32-bit	i32	    u32
        //64-bit	i64	    u64
        //128-bit	i128	u128
        //架构相关	isize	usize
        println!("整型 {}", a);
        let b: f64 = 9.0;
        //f32 和 f64，默认类型是 f64
        println!("浮点型 {}", b);
        let c: bool = true;
        println!("布尔类型 {}", c);
        let d: char = 'a';
        // 使用单引号来表示 char 字面值，而字符串字面值使用的是双引号
        println!("字符类型 {}", d);
        // 复合：多个数据
        let e: () = ();
        println!("空元组 {:?}", e);
        let f: (i32, f64, bool) = (1, 2.0, true);
        println!("元组初始化 {:?}", f);
        let (x, y, z) = f;
        println!("元组解构 {} {} {}", x, y, z);
        let f: [i32; 5] = [1, 2, 3, 4, 5];
        println!("数组初始化 {:?}", f);
        let g: [i32; 5] = [1; 5];
        println!("数组初始化 {:?}", g);
        let h: [i32; 5] = [1, 2, 3, 4, 5];
        for i in h.iter() {
            println!("数组 标量 {}", i);
        }
        first_function(111);
        let x = first_function(222);
        println!("{}", x);
        let (x, y) = second_function();
        println!("{} {}", x, y);
        third_function();
        fourth_function();
        fifth_function();
    }

    // 函数
    fn first_function(x: i32) -> i32 {
        if x < 100 {
            return x;
        } else if x < 50 {
            return x * 2;
        }
        let x = if x > 0 { x } else { 0 };
        x
    }

    fn second_function() -> (i32, i32) {
        // 循环标签：在多个循环之间消除歧义
        'asd: loop {
            println!("外循环");
            loop {
                println!("内循环");
                break 'asd;
            }
        }
        (1, 2)
    }

    fn third_function() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("{}", result)
    }

    fn fourth_function() {
        let mut number = 3;
        // while 循环
        while number != 0 {
            println!("{number}");
            number -= 1;
        }
        println!("while 循环");
    }

    fn fifth_function() {
        // for 循环
        for number in (1..4).rev() {
            println!("{number}");
        }
        println!("for 循环");

        six_function();
    }

    fn six_function() {
        // 字符串：&str 字面量，字符串字面量使用双引号
        let s: &str = "123";
        println!("字符串 {}", s);
        // 可增长的堆字符串：String
        let mut s = String::from("hello");
        println!("字符串 {}", s);
        seven_function();
    }

    // 所有权
    fn seven_function() {
        // 移动
        fn move_demo(s: String) {}
        let s = String::from("hello");
        move_demo(s); // s 不再可用

        // 复制
        fn copy_demo(x: i32) {}
        let x = 10;
        copy_demo(x); // x 仍可用

        // 不可变借用
        fn borrow_demo(s: &String) {}
        let s = String::from("hello");
        borrow_demo(&s); // s 仍可用

        // 可变借用
        fn mut_borrow_demo(s: &mut String) {}
        let mut s = String::from("hello");
        mut_borrow_demo(&mut s); // s 仍可用
    }
}
