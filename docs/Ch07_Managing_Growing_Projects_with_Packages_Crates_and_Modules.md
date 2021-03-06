# 用代码包、代码箱与模组来对日益增长的项目进行管理

在编写大型程序时，由于在头脑里对整个程序保持追踪已成为不可能，因此代码组织就尤为重要。通过将相关功能分组，并以截然不同的特性而将代码加以分离，就会搞清楚在哪里去找到实现了某个特定特性的代码，以及在那里去修改某项特性的运作方式。

到目前为止，这里所编写的程序，都是在一个模组的一个文件中的。而随着项目的增长，就可以通过将项目分解为多个模组及多个文件，来对代码加以组织。一个代码包，可以包含多个二进制的代码箱，并可有选择地包含一个库代码箱。本章会涵盖到所有的这些技巧。对于那些极为大型、有着一套互相关联而共同演化的项目，Cargo 工具提供了工作区（workspaces）概念，关于工作区，将在第 14 章的 [Cargo 工作区](Ch14_More_about_Cargo_and_Crates_io.md#cargo-workspaces)中讲到。

除了功能分组（grouping functionality），对功能实现细节的封装，还实现了高级别的代码重用：一旦实现了某个操作，其他代码就可以在无需掌握该实现原理的情况下，通过该代码的公共接口对该实现代码加以调用。编写代码的方式，就定义了哪些部分是公开给其他代码使用的，哪些部分是私有的实现细节而保留着修改的权力。这是另一种限制那些必须保留在头脑中的细节数量的方式（in addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you've implemented an operation, other code can call that code via the code's pulic interface without knowing how the implementation works. The way you write code defines which part are public for other code to use and which parts are private implementation details that you reserve the right to change. This is another way to limit the amount of detail you have to keep in your head）。

与此相关的概念，就是作用域（scope）：代码编写所在的嵌套语境，有着一套定义在所谓 “在作用域里” 的名字。在代码的读、写及编译时，程序员与编译器都需要掌握，在某个特定点位的某个特定名字，是否是指向了某个变量、函数、结构体、枚举、模组、常量或别的项目，以及该名字所指向项目的意义。作用域的创建，以及改变哪些名字在或不在某个作用域，都是可行的。在同一作用域不能有两个名字相同的项目；是有一些工具，来找出名字冲突的。

Rust 有着数种实现对代码组织进行管理的特性，包括哪些细节被暴露、哪些细节为私有，以及程序中各个作用域中有哪些名字。Rust的这些有关代码组织的特性，有时被统称为 *模组系统（module system）*，包括了：

- **代码包（packages）**：实现代码箱（crates）的构建、测试与分享的 Cargo 特性；
- **代码箱（crates）**：产生出库或可执行文件的模组树（a tree of modules that produces a library or executable）；
- **模组（modules）** 与 **`use`关键字**：实现对代码组织、作用域及路径私有化的控制（let you control the organization, scope, and privacy of paths）；
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

这里是以关键字 `mod` 开头，并随后指定出该模组的名字（在这里，就是`front_of_house`），并在该模组代码体前后放上一对花括号，而定义出模组的。在模组内部，可以有其他模组，即这里的 `hosting` 与 `serving` 模组。模组也可以保有其他项目，诸如结构体、枚举、常量、特质（traits），或者如同在清单 7-1 中那样的一些函数等等。

经由模组的使用，就可以将有关联的一些定义组织在一起，并以这些定义因何相关而命名。使用此代码的程序员们，由于他们可以根据这些分组，而非通读全部的这些定义，来浏览代码，因此在找到他们想要使用的那些定义时，就会容易一些。而对于那些要往该代码增加新功能的程序员，则将明白在哪里放置代码，而保持程序组织有序。

早先曾提到 `src/main.rs` 与 `src/lib.rs` 是叫做代码箱根。之所以他们有着这样的名字，是由于这两个文件的内容，都形成了名为 `crate` 的、位处名为 *模组树（module tree）* 的代码箱模组结构，根部的模组。

以下清单 7-2 给出了清单 7-1 中结构的模组树：

```console
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

*清单 7-2：清单 7-1 中代码的模组树*

该树展示了一些模组是怎样嵌套在另一模组内部的（比如，`hosting` 就嵌套在 `front_of_house` 里头）。该树还展示了一些模组与其他模组互为 *姊妹关系（siblings）*，即他们是定义在同一模组中的（`hosting` 与 `serving` 都是定义在 `front_of_house` 模组中）。继续以家族作比喻，那么在模组 A 是包含在模组 B 里头时，就说模组 A 时模组 B 的 *子模组（child）*，而模组 B 即为模组 A 的 *父模组（parent）*。请留意到这整个模组树，是以那个隐式的名为 `crate` 模组作为根。

模组树或许会令人想到计算机上文件系统的目录树；这可是一个极为恰当的类比！就跟文件系统中的目录一样，使用模组是为对代码进行组织。而与目录中的那些文件一样，这里是需要某种找到那些模组的方式的。

## <a id="paths-for-referring-to-an-item-in-the-module-tree"></a> 用于对目录树中某个项目进行引用的路径

**Paths for Referring to an Item in the Module Tree**

为讲清 Rust 在何处找到模组树中的某个项目，这里要使用与在文件系统中导航时同样方式的路径。在要调用某个函数时，就需要获悉他的路径。

而路径则可以有两种形式：

- *绝对路径（a absolute path）* 是通过使用代码箱名字的（对于来自外部代码箱的代码）或 `crate` 字面值（对于来自当前代码箱的代码）的，开始自某个代码箱根部；
- *相对路径（a relative path）* 则是自当前模组开始，并使用了 `self`、`super` 关键字，或当前模组中的某个标识符。

绝对与相对路径，后面跟着的都是以双冒号（`::`）分隔的一个或多个标识符。

这里来回到清单 7-1 中的示例。该怎样调用那个 `add_to_waitlist` 函数呢？这与问及那个 `add_to_waitlist` 函数的路径为何？是同样的。下面的清单 7-3 包含了清单 7-1，只是移除了一些模组与函数。这里将演示从一个新的、定义在该代码箱根部的函数 `eat_at_restaurant`，调用 `add_to_waitlist` 函数的两种方式。该 `eat_at_restaurant` 函数，是这个库代码箱公共 API 的一部分，因此这里较要将其以 `pub` 关键字进行标记。在后面的 [以 `pub` 关键字对路径进行暴露](#exposing-paths-with-the-pub-keyword) 小节，就会了解到更多有关 `pub` 关键字的知识。请注意这个示例尚不会编译；后面会解释这是为什么。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径方式
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径方式
    front_of_house::hosting::add_to_waitlist();
}
```

*清单 7-3：使用绝对与相对路径方式调用 `add_to_waitlist` 函数*

在 `eat_at_restaurant` 中第一次调用那个 `add_to_waitlist` 函数，使用了绝对路径。由于这个 `add_to_waitlist` 函数是定义在与 `eat_at_restaurant` 同一个代码箱，这意味着此处可以使用 `crate` 关键字，来开启一个绝对路径。

在字面值 `crate` 值之后，这里包含了后续的各个模组，直到抵达 `add_to_waitlist` 处。那么就可以联想到有着这同样结构的文件系统，而在文件系统中，就会指明路径 `/front_of_house/hosting/add_to_waitlist`，来运行那个 `add_to_waitlist` 程序；使用 `crate` 字面值名字，而自该代码箱根部开始，就跟使用 `/` 来从 shell 中文件系统根部开始一样。

在 `eat_at_restaurant` 中第二次调用 `add_to_waitlist` 时，使用了绝对路径。该路径是以 `front_of_house`，即与 `eat_at_restaurant` 定义在模组树的同一级别的模组名字开始的。此处等价的、将会使用到的文件系统路径即为 `front_of_house/hosting/add_to_waitlist`。以某个名字开始，就意味着该路径是绝对的。

对使用相对或是绝对路径的选择，是要基于 Rust 项目将要作出的决定。该决定会取决于是否更偏向于，将程序项目定义代码迁移到单独的地方，还是要将程序项目定义代码与要使用到这些定义的代码放在一起。比如，在将 `front_of_house` 模组与 `eat_at_restaurant` 函数，移入到一个名为 `custom_experience` 的模组中时，那么就需要更新那个到 `add_to_waitlist` 的绝对路径，但那个相对路径则仍将有效。但如果将 `eat_at_restaurant` 函数单独移入到一个名为 `dining` 的模组，那么到 `add_to_waitlist` 函数的绝对路径就会依旧保持那样，但那个相对路径则需要被更新。由于后面多半会对那些代码定义进行迁移，且对程序项的调用是各自独立的，因此优先选项即是指明绝对路径。

接下来就要尝试编译清单 7-3，并找出他为何不编译的原因！所得到的错误如下清单 7-4 所示。

```console
$ cargo build                                                                ✔ 
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

*清单 7-4：构建清单 7-3 中代码时的编译器错误*

该错误消息讲到那个模组 `hosting` 是私有的。也就是说，这里的 `hosting` 模组与 `add_to_waitlist` 函数的路径都是正确的，但由于 Rust 没有到私有部分的访问权限，而不允许使用他们。

模组只对组织代码有用。模组还定义了 Rust 的 *隐私边界（privacy boundary）*：即对那些不允许外部代码知悉、调用或依赖的各种实现细节，加以封装的界线。那么在打算将譬如某个函数或结构体构造为私有的时候，就要将其放入到某个模组中。

Rust 中私有性的工作方式，为全部程序项目（函数、方法，结构体、枚举、模组以及常量等）默认即为私有的。父模组中的程序项目，无法使用子模组内部的那些私有程序项目，但子模组中的程序项目，却可以使用他们祖辈模组中的那些程序项目。理由则是子模组要封装并隐藏他们的那些实现细节，但子模组可以看到他们被定义的上下文。延续之前的饭馆比方，请将私有规则，设想为饭馆的后厨：后厨发生的事情，对饭馆顾客来说是私密的，但行政经理们可以看到并完成他们管理饭馆中的全部事情。

Rust 选择了让模组系统以这种方式发挥作用，这样默认就将内部实现细节给隐藏了。如此一来，就清楚可修改内部代码的哪些部分，而不会破坏外部代码。不过可通过使用 `pub` 关键字将某个程序项目构造为公共项目，而把子模组代码内部部分，暴露给外部的祖辈模组。

### <a id="exposing-paths-with-the-pub-keyword"></a> 以 `pub` 关键字对路径加以暴露

这里来回到清单 7-4 中告知 `hosting` 模组为私有的错误。这里希望在父模组中的 `eat_at_restaurant` 函数，有着到那个子模组中的 `add_to_waitlist` 函数的访问权限，因此就要将 `hosting` 模组以 `pub` 关键字标记起来，如下面清单 7-5 中所示。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

*清单 7-5：将 `hosting` 模组声明为 `pub` 以在 `eat_at_restaurant` 中使用他*

不幸的是，清单 7-5 中的代码仍以错误告终，如下清单 7-6 中所示：

```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

*清单 7-6：构造清单 7-5 中代码时的编译器错误*


怎么回事呢？将 `pub` 关键字加在 `mod hosting` 前面，是把该模组构造为公共模组。有了此修改，那么在能够访问 `front_of_house` 时，就能够访问 `hosting`。但 `hosting` 模组的 *内容（contents）* 仍是私有的；将模组构造为公共模组，并不会将其内容构造为公共的。在模组上的 `pub` 关键字，只是让其祖辈模组中的代码可以引用到他。

清单 7-6 中的错误说到，那个 `add_to_waitlist` 函数是私有的。适用于结构体、枚举、函数即方法等的隐私规则，与适用于模组的一样。

下面就来通过把 `pub` 关键字，添加在 `add_to_waitlist` 函数定义之前，而将其构造为公共函数，如下清单 7-7 所示。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

*清单 7-7：把 `pub` 关键字添加到 `mod hosting` 与 `fn add_to_waitlist`，实现从 `eat_at_restaurant` 对该函数的调用*

现在该代码就会编译了！接下来看看那个绝对与相对路径，并再次确认为何添加 `pub` 关键字，就允许遵循 Rust 隐私规则之下，使用到 `add_to_waitlist` 中的这些路径。

在那个绝对路径中，是以这里的代码箱的模组树根部，字面值 `crate` 开始的。随后的 `front_of_house` 模组，即是定义在该代码箱根部的模组。那个 `front_of_house` 模组不是公共模组，但由于 `eat_at_restaurant` 函数是定义在与 `front_of_house`同一模组中（那就是说，`eat_at_restaurant` 与 `front_of_house` 是姊妹关系），因此这里就可以从 `eat_at_restaurant` 函数对 `front_of_house` 进行引用。接下来就是那个被标记了 `pub` 的 `hosting` 模组了。由于这里可以访问 `hosting` 的父模组，因此就可以访问 `hosting`。最后，由于那个 `add_to_waitlist` 函数被 `pub` 标记过，且这里可以访问他的父模组，因此该函数调用就生效了！

而在那个相对路径中，除开其中的第一步之外，其逻辑与绝对路径是同样的：与从代码箱根开始不同，该相对路径是从 `front_of_house` 处开始的。这个 `front_of_house` 模组，是定义在与 `eat_at_restaurant` 同样的模组中的，因此这个从 `eat_at_restaurant` 定义所在模组开始的相对路径，是有效的。随后由于 `hosting` 与 `add_to_waitlist` 都是以 `pub` 关键字标记过，那么该路径其余部分都是工作的，进而此函数调用就是有效的！

在计划分享库代码箱，进而其他项目可使用到该库代码箱的代码时，公共 API 即是与该代码箱用户立下的、有关该如何与库代码箱代码互动的合约。有关怎样管理对公共 API 的修改，而让人们更易于依赖到所编写的代码箱，有许多考量。这些考量超出了本书的范围；若对这方面感兴趣，那么请参阅 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)。


> **带有二进制与库的包之最佳实践（Best Practice for Packages with a Binary and a Library）**
>
> 前面提到过 Rust 包可以同时包含一个 `src/main.rs` 二进制代码箱根，与一个 `src/lib.rs` 库代码箱根，且这两个代码箱都将默认有着该 Rust 包的名字。一般来说，以这种模式的包都会在二进制代码箱中，有着启动一个会调用库代码箱中代码的、可执行程序的足够代码。由于其中库代码箱的代码可被共享，因此这就实现了其他项目受益于该 Rust 包所提供的绝大部分功能。
>
> 模组树应定义在 `src/lib.rs` 中。随后，任何公开的程序项目，都可通过以该包名字开头的路径，在二进制代码箱中被使用。二进制代码箱就跟一个将会用到那个库代码箱的完全外部箱一样，成了库代码箱的一名用户：他只能使用公共 API。这样做就有助于设计出良好的 API；你不仅是库代码箱的作者，还是一名库代码箱的客户了！
>
> 在 [第 12 章](Ch12_An_I_O_Project_Building_a_Command_Line_Program.md)，就会以一个将同时包含二进制代码箱与库代码箱的命令行程序，对这种组织实践进行演示。


### 以 `super` 关键字起头的相对路径

这里还可通过在路径开头使用 `super` 关键字，构建出在父模组处开始的相对路径。这就跟以 `..` 语法开始的文件系统路径一样。而为什么会希望这样做呢？

设想下面清单 7-8 中的、建模了某位大厨修正了某个不正确点餐，并亲自将其交出给顾客的代码。其中定义在 `back_of_house` 模组中的函数 `fix_incorrect_order`，通过将到 `deliver_order` 的路径，以 `super` 作为开头进行指明，而调用了定义在父模组中的该 `deliver_order` 函数：

文件名：`src/lib.rs`

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

*清单 7-8：使用以 `super` 开头的相对路径调用某个函数*


该 `fix_incorrect_order` 函数是在 `back_of_house` 模组中的，因此就可以使用 `super` 关键字，来前往到 `back_of_house` 的父模组，即这里的 `crate`，代码箱的根。在那里，就会查找 `deliver_order` 并找到了他。成功！那么这里就会当作这个 `back_of_house` 模组与这个 `deliver_order` 函数，大概会保持着相互之间同样的关系，并在往后决定对该代码箱的模组树重新组织时，他们会一起被移动。因此，这里使用了 `super`，如此在以后此代码被移入到某个不同模组时，要更新代码的地方就会少一些。

### 将结构体与枚举构造为公共项目

**Making Structs and Enums Public**

这里还可以使用 `pub` 关键字，来将结构体与枚举指定为公开项目，不过有着一些额外情况。当在结构体定义前使用 `pub` 关键字时，就将该结构体构造为了公共结构体，但该结构体的那些字段仍将是私有的。可根据具体情况，将各个结构体各个字段构造为公开或不公开。在下面清单 7-9 中，就定义了一个有着公开的 `toast` 字段，与一个私有的 `seasonal_fruit` 字段的公开 `back_of_house::Breakfast` 结构体。这就对在某个饭馆中，顾客可在哪里挑选与正餐搭配的面包类型，但主厨会根据当季有些什么、仓库有些什么，决定由哪些水果来搭配正餐，这样的情况进行了建模。可用的水果变化很快，因此顾客就无法对水果进行选择，甚至他们看不到会得到什么样的水果。

文件名：`src/lib.rs`

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 点下一份带有黑麦土司的夏日早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println! ("请给我一份 {} 土司", meal.toast);

    // 若不把接下来的行注释掉，那么就不会编译；这里是不允许查看或修改
    // 餐食搭配应季水果的
    // meal.seasonal_fruit = String::from("blueberries");
}
```

*清单 7-9：一个有着一些公共字段与私有字段的结构体*

由于 `back_of_house::Breakfast` 结构体中的 `toast` 字段是公开的，因此在 `eat_at_restaurant` 中就可以使用点符号，对 `toast` 字段进行写入与读取。请注意由于 `seasonal_fruit` 是私有的，因此这里不能在 `eat_at_restaurant` 中使用那个 `seasonal_fruit` 字段。尝试将那个对 `seasonal_fruit` 字段值进行修改的行解除注释，看看将得到什么样的错误！


```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
  --> src/lib.rs:25:10
   |
25 |     meal.seasonal_fruit = String::from("blueberries");
   |          ^^^^^^^^^^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
error: could not compile `restaurant` due to previous error
```

还请留意由于 `back_of_restaurant::Breakfast` 有个私有字段，因此该结构体就需要提供一个公开的、构造出`Breakfast` 实例的关联函数（这里将该函数命名为了 `summer`）。若 `Breakfast` 没有这样一个函数，那么由于在 `eat_at_restaurant` 中无法设置那个私有 `seasonal_fruit` 字段的值，因此就没法在 `eat_at_restaurant` 中创建处一个 `Breakfast` 的实例来。

作为对照，在将枚举构造为公开时，该枚举的全部变种此时都是公开的。这里就只需在 `enum` 关键字前的 `pub` 关键字，如下清单 7-10 中所示。

文件名：`src/lib.rs`

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

*清单 7-10：将枚举指定为公开，则会将其全部变种构造为公开*

由于这里将那个 `Appetizer` 枚举构造为了公开，因此就可以在 `eat_at_restaurant` 中使用 `Soup` 与 `Salad` 变种。除非枚举的各个变种是公开的，否则枚举就不是非常有用了；若在所有场合，都必须以 `pub` 关键字来对全部枚举变种进行注解，那就会让人觉得烦恼不已，因此默认枚举变种就是公开的。而结构体则通常无需其字段保持公开就有用处，因此结构体的那些字段，就遵循了除非以 `pub` 关键字注释，而默认全部为私有的一般规则。

还有一个尚未讲到的涉及 `pub` 关键字的情况，那也是最后一项模组系统特性：`use` 关键字。后面会先讲 `use` 本身，然后再给出怎样结合 `pub` 与 `use`。

### 使用 `use` 关键字将路径带入到作用域

或许到目前为止前面写的那些调用函数的路径，看起来相当不方便的长还重复。比如在清单 7-7 中，对于到 `add_to_waitlist` 函数，无论是选择绝对路径还是相对路径，每次在打算调用 `add_to_waitlist` 时，都必须还要指明 `front_of_house` 与 `hosting`。幸运的是，有简化此过程的办法。这里可以使用 `use` 关键字，一次性创建出到某个路径的快捷方式，尔后就可以在该作用域中所有地方，使用这个较短的名字了。

在下面清单 7-11 中，就将 `crate::front_of_house::hosting` 模组，带入到了 `eat_at_restaurant` 函数的作用域，因此就只须指明 `hosting::add_to_wait`，来在 `eat_at_restaurant` 中调用这个 `add_to_waitlist` 函数了。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

*清单 7-11：使用 `use` 关键字，将模组带入到作用域*

在作用域中将 `use` 关键字与一个路径一同使用，与在文件系统中创建一个符号链接类似。通过在该代码箱根本添加 `use crate::front_of_house::hosting`，在那个作用域中，现在 `hosting` 就是一个有效的名字了，就跟这个 `hosting` 模组已在该代码箱根中被定义过了一样。使用 `use` 关键字带入到作用域中的那些路径，与其他任何路径一样，同样会对隐私性进行检查。

请注意 `use` 关键字只会针对在其出现的特定作用域，创建快捷方式。下面清单 7-12 将 `eat_at_restaurant` 移入到了一个新的名为 `customer` 的子模组，随后这就与那个 `use` 语句，属于不同作用域了，进而那个函数体就不会编译：

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;


mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

下面的编译器错误，显示那个快捷方式不再适用于 `customer` 模组内部：

```console
$ cargo build
   Compiling restaurant v0.1.0 (/home/peng/rust-lang/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:33:9
   |
33 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`

warning: unused import: `crate::front_of_house::hosting`
  --> src/lib.rs:28:5
   |
28 | use crate::front_of_house::hosting;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` due to previous error; 1 warning emitted
```

请注意这里还给出了那个 `use` 在其作用域中已不再被使用的告警！为修复此问题，就还要将那个 `use` 语句移入到 `customer` 模组内部，或者在那个 `customer` 模组内部，以 `super::hosting` 来引用父模组中的那个快捷方式。

### 惯用的 `use` 路径创建

在上面的清单 7-11 中，你或许会想，为什么那里指定了 `use crate::front_of_house::hosting`，并随后在 `eat_at_restaurant` 函数中调用了 `hosting::add_to_waitlist`，而不是将那个 `use` 路径，指定为到 `add_to_waitlist` 函数的整个路径，而达到同样目的，即如下清单 7-13 中那样。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

*清单 7-13：单调的使用 `use` 将 `add_to_waitlist` 带入到作用域，bringing the `add_to_waitlist` function into scope with `use`, which is unidiomatic*

尽管清单 7-11 与 7-13 都完成了同样任务，但清单 7-11 则是以 `use` 关键字，将函数带入到作用域的惯用方式。以 `use` 关键字将函数的父模组带入到作用域中，就意味着在调用该函数时，必须指明那个父模组。而在调用该函数时指明其父模组，就让该函数是个非本地函数这一事实变得明了，与此同时仍最大程序减少了那个函数完整路径的重复。而清单 7-13 中的代码，对于 `add_to_waitlist` 在何处创建并不清楚。

另一方面，在以 `use` 关键字将结构体、枚举及其他程序项目带入时，惯用的就是指明完整路径了。下面的清单 7-14 给出了将标准库的 `HashMap` 带入到某个二进制代码箱的惯用方式。

文件名：`src/lib.rs`

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

*清单 7-14：以惯用方式将 `HashMap` 带入到作用域*

这种惯用语法背后并无强有力的理由：他只不过是业已形成的约定，且人们已经习惯了这样阅读和编写 Rust 代码。

由于 Rust 不允许使用 `use` ，将两个有着同样名字的程序项目带入到作用域，因此这就是此惯用语法的例外。下面清单 7-15 展示了怎样将两个有着同样名字，但父模组不同的 `Result` 类型，带入作用域，及怎样去引用他们。

文件名：`src/lib.rs`

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --跳过--
}

fn function2() -> io::Result {
    // --跳过--
}
```

*清单 7-15：将有着同样名字的两种类型带入到同一作用域，就要求使用他们的父模组*

可以看到，这里使用父模组，就将两个 `Result` 类型区分开了。而如果指明的是 `use std::fmt::Result;` 与 `use std::io::Result;`，那么就会得到同一作用域中的两个 `Result` 类型，而 Rust 就不明白在使用 `Result` 时，到底是要哪个了。

### 以 `as` 关键字提供新名字

解决以 `use` 关键字，将有着同样名字的两个类型，带入到同一作用域的问题，还有另一方法：在路径后面，可指明 `as` 及一个该类型的新本地名字，或别名。下面清单 7-16 给出了通过将那两个 `Result` 类型中的一个，使用 `as` 关键字进行重命名，而编写清单 7-15 中代码的另一种方式。

文件名：`src/lib.rs`

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --跳过--
}

fn function2() -> io::IoResult {
    // --跳过--
}
```

*清单 7-16：在将某个类型带入作用域时，使用 `as` 关键字对其进行重命名*


在这里的第二个 `use` 语句中，就选择了 `IoResult` 作为 `std::io::Result` 类型的新名字，这就不会与同时带入到作用域的、来自 `std::fmt` 的 `Result` 冲突了。清单 7-15 与清单 7-16 都被当作惯用方式，因此选择哪个就随你所愿了！


### 使用 `pub use` 将名字重新导出

**Re-exporting Names with `pub use`**

在以 `use` 关键字将某个名字带入到作用域中时，在新的作用域中可以的这个名字是私有的。为了其他调用这个代码的其他代码，能够对那个带入的名字，像定义在调用代码作用域中定义的名字一样，对此名字进行引用，那么就可以结合运用 `pub` 与 `use` 关键字。由于这里是将某个程序项目带入到作用域，而又同时将那个程序项目构造为可被其他代码将其带入他们的作用域，因此该技巧被称为 *重导出（re-exporting）*。

下面清单 7-17 展示了之前清单 7-11 对应的，将根模组中 `use` 修改为了 `pub use`。

文件名：`src/lib.rs`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

*清单 7-17：在新作用域中使用 `pub use` 将某个名字构造为对全部代码可用（making a name available for any code to use from a new scope with `pub use`）*

在这个修改之前，外部代码不得不通过使用路径 `restaurant::front_of_house::hosting::add_to_waitlist()`，来调用那个 `add_to_waitlist` 函数。现在既然该 `pub use` 已将这个 `hosting` 模组，从根模组中重新导出，那么外部代码就可以使用 `restaurant::hosting::add_to_waitlist()` 路径予以代替了。

在所编写代码内部结构，与那些调用代码的程序员对该领域有着不同设想时，重导出就是有用的了。比如，在这个饭馆的比喻中，运营该饭馆的人设想的是“前厅”与“后厨”。但造访饭馆的食客，或许不会用这样的词汇，来认识饭馆的这些部位。有了 `pub use`，就可以一种结构来编写代码，而以另一种结构将代码暴露出来。这样做就让这个库，同时对在这个库上编写代码的程序员，与调用这个库的程序员，具备良好的组织。在第 14 章的 [“运用 `pub use` 导出便利的公共 API”](Ch14_More_about_Cargo_and_Crates_io.md#exporting-a-convenient-public-api-with-pub-use) 小节，将会看到另一个 `pub use` 的示例，并了解他是怎样影响到代码箱的文档。


### 运用外部 Rust 包

在第 2 章中，那里编写了个使用了名为 `rand` 的外部包，来取得一个随机数的猜数游戏项目。为在项目中使用 `rand`，那里添加了下面这行到 `Cargo.toml` 文件：

文件名：`Cargo.toml`

```toml
rand = `0.8.3`
```

将 `rand` 作为一个依赖，添加到 `Cargo.toml` 就告诉了 Cargo，去 [crates.io](https://crates.io/) 下载那个 `rand` 包和全部依赖，进而令到 `rand` 对此项目可用。

随后，为了将 `rand` 的一些定义带入到所编写项目包中，那里添加了一个以那个代码箱名字，即 `rand`，开头的，以及那里打算要带入到作用域的那些条目的 `use` 行。回顾第 2 章中的 [“生成一个随机数”](Ch02_Programming_a_Guessing_Game.md#generating-a-random-number) 小节，那里就将那个 `Rng` 特质带入到了作用域，并调用了 `rand::thread_rng` 函数：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_rang(1..=100);
}
```

Rust 社群业已构造了 [crates.io](https://crates.io/) 上的许多 Rust 包，而将任意的这些包拉取到本地 Rust 包，都涉及到这些同样步骤：将他们列在本低保的 `Cargo.toml` 文件中，并使用 `use` 来将他们代码箱中的条目，带入到作用域中。

请注意标准库（`std`）同样是个相对本地包的外部代码箱。由于标准库是 Rust 语言本身附带的，因此就无需修改 `Cargo.toml` 文件来包含 `std`。但为了将 `std` 中的条目带入到本地包作用域，而仍需以 `use` 来引用他。比如，以 `HashMap` 来说，就要使用下面这行：

```rust
use std::collections::HashMap;
```

这是一个以 `std`，即标准库名字开头的绝对路径。

### 运用嵌套路径来清理大型 `use` 清单

**Using Nested Paths to Clean Up Large `use` Lists**

在使用同一代码箱或同一模组中的多个条目时，若一行一行地列出各个条目，就会占据文件中的很多纵向空间。比如，在清单 2-4 中的猜数游戏中，就有下面这两个 `use` 语句，他们将 `std` 中的两个条目带入到作用域：

文件名：`src/main.rs`

```rust
// --跳过--
use std::cmp::Ordering;
use std::io;
// --跳过--
```

相反，这里就可以使用嵌套路径，来在一行中将来自同一代码箱或包的那些条目，带入到作用域。通过指明路径的共同部分，接上一对冒号，并随后跟上以花括号封闭起来的，那些路径各异部分的清单，如下代码清单 7-18 所示。

文件名：`src/main.rs`

```rust
// --跳过--
use std::{cmp::Ordering, io};
// --跳过--
```

在那些较大型程序中，使用嵌套路径，从同一代码箱或模组，将许多条目带入到作用域，可极大地减少使用单独 `use` 语句数目！

在路径中的任何级别，都可使用嵌套路径，这一点在对两个共用了子路径的 `use` 语句进行组合时，是有用的。比如下面清单 7-19 就给出了两个 `use` 语句：一个将 `std::io` 带入到作用域，而另一个则是将 `std::io::Write` 带入到作用域。

文件名：`src/lib.rs`

```rust
use std::io;
use std::io::Write;
```

*清单 7-19：其中一个为另一个子路径的 `use` 语句*

这两个路径的共同部分，即是 `std::io`，且那就是完整的第一个路径。为将这两个路径融合为一个 `use` 语句，这里可以在嵌套路径中，使用 `self` 关键字，如下清单 7-20 中所示。

文件名：`src/main.rs`

```rust
use std::io::{self, Write};
```

*清单 7-20：将清单 7-19 中的两个路径组合为一个 `use` 语句*

这行代码就将 `std::io` 与 `std::io::Write` 带入到了作用域。

### 全局操作符

在打算将某个路径中的 *全部* 公开条目带入到作用域时，那么就可以将那个路径，后面跟上 `*`，即全局操作符，予以指明出来：

```rust
use std::collections::*;
```

这个 `use` 语句，将定义在 `std::collections` 中的全部公开项目，都带入到了当前作用域。在使用这个全局操作符时要当心！全局带入，会令到更难于分清哪些名字是作用域中的，以及在当前程序中用到的名字，是在何处定义的。

通常在测试时，要将正在测试的全部程序项目带入到 `tests` 模组，才使用这个全局操作符；在第 11 章中的 [怎样编写测试](Ch11_Writing_Automated_Tests.md#how-to-write-tests) 小节，就会讲到这个问题。在前奏模式（the prelude pattern）中，有时也会用到全局操作符：请参阅 [标准库文档](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)，了解有关更多前奏模式的知识。

## 将那些模组拆分为不同文件

**Separating Modules into Different Files**

到目前为止，本章的全部示例，都是将多个模组定义在一个文件中的。在这些模组变得大起来时，就会想要将他们的定义，迁移到单独文件，从而令到代码易于导览。

比如，这里就从之前清单 7-17 中的代码开始，并将那些模组提取到文件中，而非将所有那些模组都定义在那个代码箱根文件里。在此情况下，代码箱根文件为 `src/lib.rs`，但这个过程同样对那些根文件为 `src/main.rs` 的二进制代码箱有效。

首先，会将那个 `front_of_house` 模组，提取到他自己的文件。要移除 `front_of_house` 模组花括号里头的代码，而仅留下 `mod front_of_house;` 语句声明，这样那个 `src/lib.rs` 就会包含如下清单 7-21 中展示的代码了。请注意在创建出后面清单 7-22 中的 `src/front_of_house.rs` 文件之前，这是不会编译的。

文件名：`src/lib.rs`

```rust
mod front_of_house; 

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

*清单 7-21：对模组代码体将在 `src/front_of_house.rs` 中的 `front_of_house` 模组进行声明*

接下来，就要把原先在花括号中的代码，放入到一个新的名为 `src/front_of_house.rs` 文件中，如下清单 7-22 中所示。由于编译器在该代码箱根中，找到了名字 `front_of_house`，因此他就明白要在这个文件中看看。

文件名：`src/front_of_house.rs`

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

*清单 7-22：在文件 `src/front_of_house.rs` 中那个 `front_of_house` 模组的一些定义*

请注意在模组树中的某处，只需使用 `mod` 声明，而一次性地将某个文件的内容加载进来。一旦编译器获悉该文件是项目的一部分（并因已放置 `mod` 语句的位置，而了解到被加载代码在模组树中所处的位置），项目中的其他文件，则应使用之前 [用于引用模组树中项目的路径](#paths-for-referring-to-an-item-in-the-module-tree) 小节中，所讲的到模组声明处的路径，来引用那个文件中的代码。也就是说，这里的 `mod` *并非* 其他编程语言有的那种 “include” 操作。

接下来，就要将那个 `hosting` 模组，提取到他自己的文件了。而由于 `hosting` 是 `front_of_house` ，而非根模组的子模组，因此过程略有不同。`hosting` 模组的文件，将在一个命名为在模组树中其位置的目录中。

这里要将 `src/front_of_house.rs` 文件，修改为只包含 `hosting` 模组声明，而作为迁移 `hosting` 模组的开始：

文件名：`src/front_of_house.rs`

```rust
pub mod hosting;
```

随后就要创建一个 `src/front_of_house` 的目录，以及一个文件 `src/front_of_house/hosting.rs`，来包含在 `hosting` 模组中构造的那些定义：

文件名：`src/front_of_house/hosting.rs`

```rust
pub fn add_to_waitlist() {}
```

相反如果将 `hosting.rs` 放在 `src` 目录，那么编译器就会以为 `hosting.rs` 的代码，是在声明于代码箱根部的 `hosting` 模组中的，而不是那个 `front_of_house` 模组的子模组中的。编译器获悉要查看哪个文件，来找到模组代码所遵循的规则，意味着目录与文件的结构，与模组树结构更为贴切。

> **备用文件路径**
>
> 本小节讲的是 Rust 编译器使用的最惯用文件路径；但较早的文件路径仍被支持。
>
> 对于定义在代码箱根部的名为 `front_of_house` 模组，编译器会在下面这些地方查找该模组的代码：

- `src/front_of_house.rs` （即这里讲到的）；
- `src/front_of_house/mod.rs` （较早的，仍被支持的路径）。

> 而对于作为 `front_of_house` 的子模组的名为 `hosting` 的模组，编译器会在以下地方查找该模组的代码：

- `src/front_of_house/hosting.rs` （即这里讲到的）；
- `src/front_of_house/hosting/mod.rs` （较早的，仍被支持的路径）。

> 在对统一模组同时使用这两种文件路径时，那么就会得到一个编译器错误。而对同一项目中的不同模组，采用不同方式的文件路径是被允许的，只是会对那些导览项目的人造成困扰。
>
> 使用那些名为 `mod.rs` 文件方式的主要缺点，就是那样的话项目最终会以许多名为 `mod.rs` 文件而终结，这样在同时于代码编辑器中打开这些 `mod.rs` 文件时，就会感到混乱。

将各个模组的代码，移入到单独文件现在就完成了，而模组树还是保持原来那样。尽管那些定义存在于不同文件，但无需任何修改，那个在 `eat_at_restaurant` 中的函数调用仍会工作。这样的技巧，就允许我们在模组尺寸增长时，将他们迁移到新的文件。

请注意在 `src/lib.rs` 中的那个 `pub use crate::front_of_house::hosting;` 语句同样不曾改变，同时那个 `use` 也不会对哪些文件作为代码箱的部分而被编译有任何的影响。这个 `mod` 关键字定义了模组，而 Rust 则会在与该模组有着同样名字的文件中，查找要进到那个模组中的代码。

## 总结

Rust 允许将包拆分为多个代码箱，进而将代码箱拆分为多个模组，这样就可以从一个模组，对定义在另一模组中的程序项目加以引用。通过指明绝对或相对路径，就可以进行这样的引用。使用 `use` 语句，就可以将这些程序项目的路径，带入到作用域，如此就可以在那个作用域中，在多次用到所带入的程序项目时，使用较简短的路径。默认模组代码是私有的，但可通过添加 `pub` 关键字，而将一些定义构造为公开的。

在接下来的章中，就会关注标准库中的一些，可在本地组织良好代码中使用的集合数据结构了。
