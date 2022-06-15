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

前面讲过，`read_line`方法将用户敲入的东西，放入到传递给他的那个字符串中，然而 `read_line` 还会返回一个值 -- 在此实例中，返回的就是一个 `io::Result` 类型值。Rust 在他的标准库中，有着数个名为 `Result` 的类型：这是一个泛型的 `Result`，对于那些子模组都有着特定版本，比如这里的 `io::Result`。`Result` 的那些类型都属于 [枚举（enumerations）](Ch06_Enums_and_Pattern_Matching.md#enums)，枚举常被写为 `enums`，枚举有着一套被称作 *变种（variants）* 的可能值。枚举常常是和 `match` 关键字一起使用的，而 `match` 则是一种条件判断，在符合某个条件时，就可以很方便地根据枚举中的哪个变种，来执行不同代码。

第 6 章将深入涵盖到枚举数据结构。而这些 `Result` 类型的目的，则是对错误处理信息进行编码。

这个 `Result` 的变种，就是 `Ok` 与 `Err`。`Ok` 变种表示该操作是成功的，而在 `Ok` 内部，就是成功生成的值。相反 `Err` 变种，则意味着操作失败了，同时 `Err` 包含了关于操作失败的方式与原因。

`Result` 类型的那些值，跟其他任何类型都差不多，在这些值上都定义了一些方法。`io::Result` 实例，就有一个可供调用的 [`expect` 方法](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect)。在这个 `io::Result` 实例是个 `Err` 变种时，那么`expect` 方法就会导致程序崩溃，并将传递给 `expect` 方法的参数显示出来。若 `read_line` 方法返回了一个 `Err`，那很可能是来自所采用操作系统错误的结果（if the `read_line` method returns an `Err`, it would likely be the result of an error coming from the underlying operating system）。而若该 `io::Result` 实例是个 `Ok` 值，那么 `expect` 方法就会取得那个 `Ok` 所保存的返回值，并只将该值返回，从而就可以使用到这个返回值。在此实例中，那个值，就是用户输入中的字节数目。

若这里没有对 `expect` 方法进行调用，那么该程序会编译，不过会收到一条告警信息：

```console
$ cargo build                                                                                    ✔ 
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 | /     io::stdin()
11 | |         .read_line(&mut guess);
   | |_______________________________^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
```

Rust 警告说不曾对返回自 `read_line` 的 `Result` 值进行使用，表示程序没有对可能的错误加以处理。

消除该警告信息的正确方式，就是要老老实实地编写错误处理代码，而在这个实例中，则只要在问题发生时，崩溃掉这个程序即可，因此这里就可以使用 `expect`。在 [第 9 章](Ch09_Error_Handling.md#recoverable-errors-with-result) 会掌握到如何从错误中恢复过来。

## 使用 `println!` 的占位符将值打印出来

**Printing Values with `println!` Placeholders**

紧接着那个结束花括号前面，就只有剩下的一行代码要讨论了：

```rust
    println! ("你猜的数是：{}", guess);
```

这行代码是将此刻包含了用户输入的那个字符串打印出来。其中的那套花括号 `{}` ，就是一个占位符（placeholder）：请将`{}`当作是些在那个地方留有一个值的小螃蟹。使用一些这样的花括号，就可以打印出多个值来：第一套花括号保留着在格式化字符串之后列出的第一个值，第二套保留着第二个值，如此等等。一个 `println!` 调用中多个值的打印，看起来会是下面这样：

```rust
let x = 5;
let y = 10;

println! ("x = {} 同时 y = {}", x, y);
```

此代码将打印出 `x = 5 同时 y = 10`。

## 对第一部分的测试

下面就来测试一下这猜数游戏的第一部分。用 `cargo run` 运行他：

```console
$ cargo run                ✔ 
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
     Running `target/debug/guessing_game`
猜出这个数来！
请输入你猜的数。
6
你猜的数为：6
```

此刻，这游戏的第一部分就算完成了：这里正从键盘获取到输入，并随后将输入打印出来。


## 生成秘密数字

接下来，就需要生成一个用户将要试着去猜的秘密数字了。生成的秘密数字应每次都不相同，这样这游戏在多次玩的时候才有趣。为了不让这个游戏太难，这里要用一个 `1` 到 `100` 之间的随机数。Rust 在其标准库中尚未包含随机数功能。不过 Rust 团队还真的提供了一个 [`rand` 代码箱](https://crates.io/crates/rand)，这里就姑且把这样的代码箱，称之为功能吧。

### 运用代码箱（a Crate） 获取到更多功能

请记住，所谓代码箱，即为一些 Rust 源代码文件的集合。之前曾构建好的项目，则是一个 *二进制的代码箱（binary crate）*，那是个可执行程序。而 `rand` 代码箱，则是个 *库代码箱（library crate）*，这样的库代码箱，包含了预期将在其他程序中会用到的代码，同时库代码箱自身并不能执行（the `rand` crate is a *library crate*, which contains code intended to be used in other programs, and can't be executed on its own）。

Cargo 对外部代码箱的协调能力，正是 Cargo 真正闪耀之处。在能够编写出用到 `rand` 库代码箱的代码之前，先要将 `Cargo.toml` 加以修改，将 `rand` 代码箱作为依赖包含进来。打开那个文件并将下面的行，添加到底部、那个 Cargo 创建出的`[dependencies]` 小节标题之下。要确保像这里一样，带着版本号地精确指明 `rand` 代码箱，否则此教程中的代码示例就不会工作。

文件名：`Cargo.toml`

```toml
rand = "0.8.3"
```

在这 `Cargo.toml` 文件中，凡在某个标题之后的东西，都是那个小节的一部分，直到另一小节开始为止。在 `[dependencies]` 小节，告诉 Cargo 的是项目依赖了哪些外部代码箱（external crates），以及所需的这些代码箱版本。在此实例中，就指明了有着语义版本指示符（the semantic version specifier） `0.8.3` 的 `rand` 库代码箱。Cargo 能明白 [语义化版本控制（Sementic Versioning）](http://semver.org/)（有时也叫做 *`SemVer`*），这是编制版本号的标准。数字 `0.8.3` 实际上是 `^0.8.3` 的缩写，表示高于 `0.8.3` 却低于 `0.9.0` 的任何版本。Cargo 认为这些版本有着与 `0.8.3` 兼容的公共 APIs，同时这样的规定，确保了将获取到在本章中代码仍可编译的情况下，最新的补丁发布。那些 `0.9.0` 及更高的版本，无法保证接下来示例用到同样的 API。

现在，在不修改任何代码的情况下，来构建一下这个项目，如清单 2-2 所示：

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

*清单 2-2-1：在添加了作为依赖的 `rand` 代码箱后运行 `cargo build` 的输出（书上的输出）*

```console
$ cargo build              ✔ 
    Updating crates.io index
  Downloaded cfg-if v1.0.0
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
  Downloaded getrandom v0.2.7
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand v0.8.5
  Downloaded libc v0.2.126
  Downloaded 7 crates (773.8 KB) in 3.41s
   Compiling libc v0.2.126
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling getrandom v0.2.7
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 56.66s
```

*清单 2-2-2：在添加了作为依赖的 `rand` 代码箱后运行 `cargo build` 的输出（实际输出）*

这里可能会看到不同的一些版本号（归功于 `SemVer`，这些不同版本号将与示例代码全都兼容！）、不同的输出行（取决于所在的操作系统），以及这些行可能以不同顺序出现。

在包含外部依赖时，Cargo 会从 *登记处（registry）* 拉取到那个依赖所需的全部最新版本的代码箱，而所谓登记处，则是 [Crates.io](https://crates.io/) 数据的一份拷贝。Crates.io 是 Rust 生态中的人们，发布给其他人使用的开放源代码项目的地方。

在更新了登记处索引之后，Cargo 就对 `[denpendencies]` 小节进行查看，并下载所列代码箱中尚未下载的那些。在此实例中，尽管只列出了依赖 `rand`，Cargo 还抓取了其他 `rand` 赖以运作的一些代码箱。在下载了这些代码箱之后，Rust 会对他们进行了编译，并随后以这些可用的依赖，对这项目进行了编译。

若不做任何修改，就立即再次运行 `cargo build`，那么除了那行 `Finished` 输出之外，就再也没有别的输出了。Cargo 明白他以及下载并编译好了那些依赖，还明白尚未对 `Cargo.toml` 文件做任何修改。Cargo 还知道，这里并未对项目代码做任何修改，因此他也没有对项目代码重新编译。既然无事可做，那么他就直接退出了。

```console
$ cargo build                                                            ✔ 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

若此时打开 `src/main.rs` 文件，做个细微修改，然后保存并再次构建，那么就只会看到下面这两行输出:

```console
cargo build                                                            ✔ 
   Compiling guessing_game v0.1.0 (/home/peng/rust-lang/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

这些行显示 Cargo 只更新了对 `src/main.rs` 文件细微修改的构建。由于依赖不曾改变，因此 Cargo 清除他可以重用那些已经下载和编译好的依赖。

### 使用 `Cargo.lock` 文件确保可重现的构建

**Ensuring Reproducible Builds with the `Cargo.lock` File**

Cargo 具备一种不论是自己还是其他要构建代码的人来说，确保每次都可以构建出同样程序组件（the same artifact）的机制：除非另有指定，Cargo 都将只使用在 `[denpendencies]` 小节中所指定的依赖版本。比如说下周 `0.8.4` 版本的 `rand` 就要释出，且那个版本包含了一个重要的错误修复，但也包含了一个会破坏咱们代码的特性撤回。为了应对这样的情况，Rust 在首次运行 `cargo build`时，就创建了 `Cargo.lock` 文件，也就是现在在 `guessing_game` 目录下就有这么个文件。

在首次构建项目时，Cargo 会找出那些依赖满足条件的所有版本，并将其写入到这 `Cargo.lock` 文件。在今后对项目进行构建时，Cargo 就会查看是否存在那个 `Cargo.lock` 文件，并使用其中所指定的那些版本，而不会再次完成找出那些版本的工作了。这样就自动实现了可重现的构建。也就是说，得益于这个 `Cargo.lock` 文件，除非显式地升级了 `rand` 的版本号，项目将保持其版本为 `0.8.3`。

### 更新代码箱来获取新版本

**Updating a Crate to Get a New Version**


