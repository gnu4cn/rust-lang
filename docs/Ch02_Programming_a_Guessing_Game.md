# 编写一个猜谜游戏

现在就要通过一个动手项目，投身到 Rust 中！本章将通过给出如何在真实程序中使用几个常见 Rust 概念，而对他们进行介绍。将了解有关 `let`、`match` 关键字，方法、关联函数（associated functions）、使用外部 Rust 代码盒子（Rust crates），及更多的 Rust 概念！而在接下来的章节，就会对这些概念进行深入探索。在本章中，将会对基础知识进行实操。

这里将实现一个经典的新手编程问题：猜谜游戏。他的工作运作机制为：程序将生成一个 1 到 100 之间的随机整数。随后将提示玩家输入猜测的数字。在玩家输入后，程序将表明猜测的数字是小了还是大了。在猜到正确的数字时，游戏就会打印出一条祝贺消息并推出。

## 建立一个新项目

要建立新项目，就要前往第一章所创建出的 `projects` 目录，并使用 Cargo 构造一个新项目，像下面这样：

```console
$ cargo new guessing_game
$ cd guessing_game
```

第一条命令，`cargo new`，取了项目名称（`guessing_game`）作第一个参数。而第二条命令则是前往到这个新项目的目录下。

来看看这个生成的 `Cargo.toml` 文件：

文件名：`Cargo.toml`

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

就跟在第 1 章中所看到的那样，`cargo new` 生成了一个 “Hello, world!” 的程序。检视那个 `src/main.rs` 文件：

文件名：`src/main.rs`

```rust
fn main() {
    println! ("Hello, world!");
}
```

现在就来使用 `cargo run` 命令，在同一个步骤中编译这个 “Hello, world!” 程序并运行他：

```console
$ cargo run                                                                                   ✔ 
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/guessing_game`
Hello, world!
```

在需要在项目上进行快速迭代时，这个 `run` 命令用起来就相当顺手了，这正是这里在这个游戏中将要做的，在前往下一迭代之前，对每次迭代进行快速测试。

请再次打开这个 `src/main.rs` 文件。即将在这个文件中编写所有的代码。

## 处理一次猜数

这个猜数游戏的第一部分，将请求用户的输入、处理那个输入，进而检查该输入是否有着正确格式。这里将实现玩家输入一个猜数开始。请敲入清单 2-1 中的代码到 `src/main.rs` 里去。

文件名：`src/main.rs`

```rust
use std::io;

fn main() {
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取行失败");

    println! ("你猜的数为：{}", guess);
}
```

*清单 2-1，从用户获取到一个猜数并将其打印出来的代码*

此代码包含了很多信息，那么这里就来一行一行的走一遍。要获取到用户输入并将结果打印出来，就需要将 `io` 输入/输出库带入到作用域中。而 `io` 库则是来自名为 `std` 的标准库：

```rust
use std::io;
```

默认情况下，Rust 只有少数几个定义在标准库中、由标准库带入到每个程序的项目（by default, Rust has a few items defined in the standard library that it brings into the scope of every program）。这个集合被称为 Rust 序曲（`prelude`），在 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html) 中可找到全部的标准库 `prelude` 项目。

在要使用的类型，不在 Rust 序曲集合中时，就必须将那个类型，显式地通过 `use` 语句带入到作用域中。`std::io` 库的使用，提供了数个有用特性，包括接收用户输入的能力。

就跟在第 1 章所见到的那样，`main` 函数即是这个程序的进入点：

```rust
fn main() {
```

`fn` 语法声明了一个函数，而这个圆括号，`()`，表示这里没有参数，同时那个花括号，`{`，是该函数的函数体的开始。

同样与在第 1 章中所了解的那样，`println!` 是个将字符串打印到屏幕的宏（macro）：

```console
    println! ("猜出这个数来！");

    println! ("请输入你猜的数。");
```

这段代码在打印提示消息，表明该游戏是什么及正在请求用户输入。

## 使用变量保存那些值

接下来，就要创建一个 *变量（variable）* 来存储用户输入，像下面这样：

```rust
    let mut guess = String::new();
```

现在这个程序就变得有趣起来了！这小小一行，可是有很多东西。这里使用了 `let` 语句来创建这个变量。下面是另一个示例：

```rust
let apples = 5;
```

这行代码创建了一个新的名为 `apples` 的变量，并将其绑定到了值 `5`。在 Rust 中，默认变量是不可变的（immutable）。在后续第 3 章的 [变量及可变性](Ch03_Common_Programming_Concepts.md#variables-and-mutability) 小节，将对此概念加以讨论。而要让变量可变，就要将变量名字前加上 `mut` 关键字：

```rust
let apples = 5; // 不可变（immutable）
let mut bananas = 5; // 可变（mutable）
```

> 注意：这里的 `//` 语法，开始了一条持续到那个行结束的代码注释。Rust 会忽略注释中的全部内容。在 [第 3 章](Ch03_Common_Programming_Concepts.md#comments) 将更加详细地讨论代码注释。

回到这个猜数游戏程序，那么此刻就明白了那个 `let mut guess` 将引入一个名为 `guess` 的可变变量。而那个等号（`=`），则是告诉 Rust，现在要将某个东西绑定到该变量了。等号右边就是要绑定到 `guess` 的那个值，而这个值则是调用 `String::new` 的结果，这个 `String::new`，则又是一个返回一个 `String` 实例的函数。`String` 是由标准库提供的一个字符串类型，为一个可增大的、经 UTF-8 位编码的文本（a growable, UTF-8 encoded bit of text）。

在那个 `::new` 代码行中的 `::` 语法，表示其中的 `new` 是 `String` 类型的一个关联函数（an associated funtion of the `String` type）。至于 *关联函数（associated function）*，指的是应用到某种类型上的函数，在此实例中，类型就是 `String` 了。这个 `new` 函数创建了一个新的、空空的字符串。由于`new` 是个构造某种新值的常见函数，因此在许多类型上，都将找到 `new` 函数。

整体上看，这个 `let mut guess = String::new();` 语句，完成了一个当前绑定到新的、`String` 类型空实例的可变变量的创建。总算讲清楚了！

## 接收用户输入

回顾程序第一行上，以 `use std::io;` 从标准库所包含进来的输入/输出功能。那么现在就要调用那个 `io` 模组中的 `stdin` 函数，该函数将实现对用户输入的处理：

```rust
    io:stdin()
        .readline(&mut guess)
```

若在程序的开头不曾以 `std::io` 方式，将 `io` 库导入，那么仍然可以将该函数写作 `std::io::stdin` 形式，而对其进行使用。`stdin` 函数返回的是 `std::io::Stdin` 的实例， 而 `std::io::Stdin` 则表示终端标准输入句柄的类型（the `stdin` function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal）。

接下来的代码行 `.readling(&mut guess)` 调用了标准输入句柄类型实例上的 `read_line` 方法，用于获取用户输入。这里还将 `&mut guess` 作为 `read_line` 的参数进行了传递，以告诉 `read_line` 函数，将用户输入存入到哪个字符串中。`read_line` 的整个职能，就要将用户敲入到标准输入的东西，追加到某个字符串（在不覆盖掉这个字符串内容的情况下），因此这里是将那个字符串作为参数传递的。为了这个 `read_line` 方法可以修改其内容，这里的字符串就要是可变的。

其中的 `&` 表明该参数是个 *引用（reference）*，而引用则是一种无需将数据多次拷贝到内存中的情况下，就可以实现代码多个部分对该数据进行读写的特性（注：在 C 家族语言中，`&`表示内存地址，因此 Rust 中的引用，与指针有类似之处）。引用是一项复杂特性，同时 Rust 的主要优点之一，就是安全而便利地运用引用的方式。对于完成这个猜数游戏，是不必对这些细节有过多了解的。现在要明白的是，与变量类似，引用默认也是不可变的。因此，这里就要写上 `&mut guess` 而不是 `&guess`，来令到这个到 `guess` 的引用为可变的。（第 4 章将更详细地对引用进行解释。）

## 对潜在结果类型失败的处理

**Handle Potential Failure with the Result Type**

这里还在解析代码行。尽管这里讨论的是代码文本的第三行，但他仍是单个逻辑代码行的一部分。接下来的部分是这个方法：

```rust
        .expect("读取输入失败");
```

这代码本可以写成下面这样：

```rust
io::stdin().read_line(&mut guess).expect("读取输入失败");
```

不过这样的一个长代码行，难于阅读，因此最好将其分开为多个断行。在以 `.method_name()` 语法调用方法时，通过引入另起一行及缩进，来将长的代码行拆分为短代码行，通常是明智的。下面就来说说这一行完成了什么。

前面讲过，`read_line`方法将用户敲入的东西，放入到传递给他的那个字符串中，然而 `read_line` 还会返回一个值 -- 在此实例中，返回的就是一个 `io::Result` 类型值。Rust 在他的标准库中，有着数个名为 `Result` 的类型：
