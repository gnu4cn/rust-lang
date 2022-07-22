# 用代码包、代码箱与模组来对日益增长的项目进行管理

在编写大型程序时，由于在头脑里对整个程序保持追踪已成为不可能，因此代码组织就尤为重要。通过将相关功能分组，并以截然不同的特性而将代码加以分离，就会搞清楚在哪里去找到实现了某个特定特性的代码，以及在那里去修改某项特性的运作方式。

到目前为止，这里所编写的程序，都是在一个模组的一个文件中的。而随着项目的增长，就可以通过将项目分解为多个模组及多个文件，来对代码加以组织。一个代码包，可以包含多个二进制的代码箱，并可有选择地包含一个库代码箱。本章会涵盖到所有的这些技巧。对于那些极为大型、有着一套互相关联而共同演化的项目，Cargo 工具提供了工作区（workspaces）概念，关于工作区，将在第 14 章的 [Cargo 工作区](Ch14_More_about_Cargo_and_Crates_io.md#cargo-workspaces)中讲到。

除了功能分组（grouping functionality），对功能实现细节的封装，还实现了高级别的代码重用：一旦实现了某个操作，其他代码就可以在无需掌握该实现原理的情况下，通过该代码的公共接口对该实现代码加以调用。编写代码的方式，就定义了哪些部分是公开给其他代码使用的，哪些部分是私有的实现细节而保留着修改的权力。这是另一种限制那些必须保留在头脑中的细节数量的方式（in addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you've implemented an operation, other code can call that code via the code's pulic interface without knowing how the implementation works. The way you write code defines which part are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head）。

与此相关的概念，就是作用域（scope）：代码编写所在的嵌套语境，有着一套定义在所谓 “在作用域里” 的名字。在代码的读、写及编译时，程序员与编译器都需要掌握，在某个特定点位的某个特定名字，是否是指向了某个变量、函数、结构体、枚举、模组、常量或别的项目，以及该名字所指向项目的意义。作用域的创建，以及改变哪些名字在或不在某个作用域，都是可行的。在同一作用域不能有两个名字相同的项目；是有一些工具，来找出名字冲突的。

Rust 有着数种实现对代码组织进行管理的特性，包括哪些细节被暴露、哪些细节为私有，以及程序中各个作用域中有哪些名字。Rust的这些有关代码组织的特性，有时被统称为 *模组系统（module system）*，包括了：

- **代码包（packages）**：实现代码箱（crates）的构建、测试与分享的 Cargo 特性；
- **代码箱（crates）**：产生出库或可执行文件的模组树（a tree of modules that produces a library or executable）；
- **模组（modules）**与 **`use`关键字**：实现对代码组织、作用域及路径私有化的控制（let you control the organization, scope, and privacy of paths）；
- **路径（paths）**：对结构体、函数或模组等进行命名的方式（a way of naming an item, such as a struct, function, or module）。

在本章中，就要涉及到这些特性，讨论到他们之间互动的原理，并就如何使用这些特性来对作用域进行管理。在本章结束时，就会对 Rust 的模组系统有扎实掌握，并能够像专业 Rust 程序员那样，以作用域来编写程序！

## 代码包与代码箱

Rust 模组系统的第一部分，将涉及到代码包与代码箱。

*代码包（package）* 即为一个或多个的、提供一套功能集的代码箱。代码包包含了一个描述了如何来构建这些代码箱的 `Cargo.toml` 文件。

而 *代码箱（crate）* 则可以是二进制代码箱（a binary crate），或者是库代码箱(a library crate)。*二进制代码箱* 是一些可编译为可以运行的一些程序，譬如命令行的程序或服务器程序。二进制代码箱必须有着一个叫做 `main` 的、定义了在可执行文件运行时所发生事情的函数。到目前为止本书中创建的全部代码箱，都是二进制代码箱。

*库代码箱* 是没有 `main` 函数的，且他们不会编译到可执行文件。他们对计划在多个项目下使用的功能进行定义。比如在 [第二章](Ch02_Programming_a_Guessing_Game.md#generating-a-random-number) 中用到的、提供了生成随机数功能的 `rand` 代码箱。

*代码箱根（crate root）* 是个 Rust 编译器开始之处的源文件，并构成了代码箱的根模组（the *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate. 后面在 [定义对作用域和私有化进行控制的模组](#defining-modules-to-control-scope-and-privacy) 小节，将深入探讨到模组概念）。

代码包能包含些什么，是由数条规则确定的。一个代码包可包含至多一个的库代码箱。而对于二进制代码箱，则是想要多少就可包含多少，不过代码包必须包含至少一个代码箱（库或二进制均可）。

下面就来过一下在创建代码包时，发生了些什么。首先，这里要敲入命令 `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project                                                                            ✔ 
Cargo.toml  src
$ ls my-project/src                                                                        ✔ 
main.rs
```

在敲入了该命令后，Cargo 就创建出了 `Cargo.toml` 文件，于是给到了一个代码包。查看 `Cargo.toml` 文件的内容，其中由于 Cargo 依循了一条 `src/main.rs` 即为与该代码包同名二进制代码箱的箱根的约定，因此 `Cargo.toml` 内容中并未提及 `src/main.rs`。与此类似，Cargo 还知道在代码包目录包含了 `src/lib.rs` 时，这个代码包就包含了与该包同名的一个库代码箱，而那个 `src/lib.rs` 就是该库代码箱的箱根。Cargo 会将代码箱根文件，传递给 `rustc` 来构建出相应的库或二进制程序。

这里有一个只包含了 `src/main.rs` 的代码包，就意味着他只包含了名为 `my-project` 的二进制代码箱。而在代码包同时包含了 `src/main.rs` 与 `src/lib.rs` 时，他就会有两个代码箱：一个二进制和一个库代码箱，二者都有着与该代码包同样的名字。代码包通过将一些文件放入到 `src/bin` 目录：每个文件都将是单独的二进制代码箱，这样就能够有着多个二进制代码箱。


## 定义对作用域和私有化进行控制的模组

在本小节中，将讲到模组与模组系统的其他部分，即实现对各种项目（items, 变量、函数、结构体、枚举、模组、常量或别的项目）命名的 *路径（paths）*；还有将某个路径引入到作用域的 `use` 关键字；以及将那些项目构造为公开项目的 `pub` 关键字。这里还将对 `as` 关键字、外部代码包，以及全局操作符等加以讨论。

首先，这里将以一个往后对代码进行组织时，易于参考的规则列表开始。随后就会对这些详细解释。

### 模组速查

下面就是模组、路径、`use` 关键字与 `pub` 关键字在编译器中的工作原理，以及多数开发者组织他们代码的方式。这里将逐个地对这些规则进行示例演绎，同时这也是作为理解 Rust 模组工作原理，而展望未来的极佳之处。

- **自代码箱根开始（start from the crate root）**：在编译代码箱时，编译器首先看的是代码箱根文件（对于库代码箱，通常为 `src/lib.rs`，或者二进制代码箱的 `src/main.rs`）；

+ **模组的声明（declaring modules）**：在代码箱根文件中，可声明一个命名的模组，比如说以 `mod gargen;`，声明的 `garden` 模组。此时编译器就会在下面这些地方，查找该模组内部的代码：
    - 内联代码，即直接跟着 `mod garden` 语句之后的那些代码，此时 `mod garden` 之后不再是分号，而是花括号的代码块；
    - 在文件 `src/garden.rs` 里的代码；
    - 在文件 `src/garden/mod.rs` 里的代码；

+ **子模组的声明（declaring submodules）**：在除了代码箱根文件之外的、将被编译为代码箱一部分的其他任何文件（比如 `src/garden.rs`）中，都可以声明子模组（比如 `mod vegetables；`）。编译器将在命名为父模组的目录里，在以下地方，查找那些子模组内部代码：
    - 内联代码，即直接在 `mod vegetables` 语句之后的代码，此时 `mod vegetables` 之后不再是分号，而是花括号的代码块；
    - 在文件 `src/garden/vegetables.rs` 文件中；
    - 在文件 `src/garden/vegetables/mod.rs` 文件中。

- **到模组中代码的路径（paths to code in modules）**：一旦模组被编译为了代码箱的一部分，就可以在该代码箱的任何地方，通过使用路径，对那个模组中的代码（比如，`garden` 模组的子模组 `vegetables` 中的 `Asparagus` 类型）进行引用。

- **私有与公共（private vs public）**：默认情况下，模组内的代码对该模组的父模组是私有的。要令到模组成为公共模组，就要使用 `pub mod` 而非 `mod` 来声明该模组。而要令到公共模组中的各个项目也成为公共项目，那么就要在这些项目的声明之前使用 `pub` 关键字。

- **`use` 关键字**：在某个作用域内，`use` 关键字会创建出到各个项目的快捷方式，从而减少那些长路径的重复。在任何可引用到 `crate::garden::vegetables::Asparagus` 的作用域中，都可以使用 `use crate::garden::vegetables::Asparagus;` 语句，创建出一个快捷方式，而在随后就只需要写出 `Asparagus` 来在该作用域中使用上那个类型了。


下面是个名为 `backyard`、对这些规则加以演示的二进制代码箱。该代码箱的目录，也叫做 `backyard`，包含了下面这些文件与目录：

```console
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

该代码箱的根文件，在此实例中即为 `src/main.rs`，包含下面的代码：

文件名：`src/main.rs`

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println! ("I'm growing {:?}!", plant);
}
```

语句 `pub mod garden;`，表示编译器会包含他在 `src/garden.rs` 中找到的代码，也就是：

文件名：`src/garden.rs`

```rust
pub mod vegetables;
```

而语句 `pub mod vegetables;` 表示在 `src/garden/vetables.rs` 中的代码也会被编译器包含：

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

现在就来进入到这些规则的细节，并在实际操作中对他们进行演示吧！


### 将有关联的代码，组织在模组中

**Grouping Related Code in Modules**

*模组（modules）* 实现了为代码可读性与易于重用目的，而将代码箱中的代码，组织为一些组别。模组特性还控制了各个项目的 *隐私性（privacy）*，项目的隐私性，是指某个项目是否可被外部代码使用（即 *公开，public*），抑或该项目属于内部实现的细节，而不对外部用途可用（即 *私有，private*）。

举例来说，这里要编写一个提供到饭馆功能的库代码箱。那么就会定义一些函数的签名，而将这些函数的函数体留作空白，这样来专注于代码的组织，而非在代码中具体实现一个饭馆出来。

在餐饮行业，饭馆的一些部分被称作 *前台，front of house*，而其余部分则被称作 *后厨，back of house*。前台是顾客们所在的地方；这是饭馆领台给食客安排位置、服务员拿到菜单和买单，以及调酒师制作饮品的地方。而后厨则是大厨和厨师们在厨房做菜、洗碗工做清洁工作，以及经理们完成行政工作的地方。

这里就可以将那些函数组织到嵌套在代码箱的一些模组中，从而把该代码箱以真实饭馆运作的方式，架构起来。通过运行 `cargo new --lib restaurant` 命令，创建出一个新的、名为 `restaurant` 的库；然后把下面清单 7-1 中的代码，放入到文件 `src/lib.rs` 里，而定义出一些模组与函数签名来。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

*清单 7-1：一个包含着其他的、随后具有函数模组的`front_of_house` 模组（a `front_of_house` module containing other modules that then contain functions）*

这里是以关键字 `mod` 开头，并随后指定出该模组的名字（在这里，就是`front_of_house`），并在该模组代码体前后放上一对花括号，而定义出模组的。在模组内部，可以有其他模组，即这里的 `hosting` 与 `serving` 模组。模组也可以保有其他项目，诸如结构体、枚举、常量、特质（traits），或者在清单 7-1 中的那些函数等等。


