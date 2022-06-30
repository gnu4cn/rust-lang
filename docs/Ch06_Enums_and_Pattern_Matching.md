# 枚举与模式匹配

**Enums and Pattern Matching**

在本章，将会对 *枚举（enumerations）* 进行审视，枚举也被当作 *enums*。枚举实现了通过列举出类型可能的 *变种（variants）*，来定义出一个类型。这里会首先定义并使用一个枚举，来展示枚举能如何将意义和数据编码起来。接下来，就会探索一个特别有用、名为 `Option` 的枚举，该枚举表示了某个值既可以是某事物，也可以什么也不是。随后就会看看在 `match` 表达式中的模式匹配，是怎样令到根据枚举中不同的值，而运行各异的代码容易起来的。最后，将会讲到 `if let` 结构是怎样成为另一种处理代码中枚举值的、便利而简洁的习惯用法的。

许多语言都有枚举这一特性，不过在各个语言中的枚举能力是不同的。Rust 的枚举与那些函数式语言，诸如 F#、OCaml 及 Haskell 等中的 *代数数据类型（algebraic data types）* 最为相似。

## 定义一个枚举

枚举是不同于结构体的第二种定义定制数据类型的方式。下面就来看看一种在代码中可能表达的情形，并见识一下为何在此情形下，相比于结构体，枚举是有用且更恰当的。假设说这里需要对 IP 地址进行处理。目前仅有两种用于 IP 地址的标准：版本四和版本六。由于这两个标准是程序将遇到的 IP 地址仅有的可能性，因此就可以 *列举出（enumerate）* 全部可能的变种，这正是枚举（enumeration） 名字得来之处。

任何 IP 地址都只能是版本四或版本六的地址，而不会同时两个都是。由于枚举值只能是枚举变种之一，那么 IP 地址的这个属性，令到枚举数据结构（the enum data structure）恰当起来。而版本四和版本六两种地址，从根本上说都是 IP 地址，那么在代码对适用于任意类别 IP 地址的情形加以处理时，版本四和版本六地址都应当作同一类型对待。

在代码中，可通过定义一个 `IpAddrKind` 枚举，并列出 IP 地址可能的类别，即 `V4` 和 `V6`，来表达这个概念。下面就是该枚举的变种：

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

现在 `IpAddrKind` 就是一个可在代码中别的地方使用的定制数据类型了。


### 枚举取值

可像下面这样，创建出 `IpAddrKind` 两个变种的实例来：

```rust
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

请注意，该枚举的两个变种，是在其标识符的命名空间之下的，且这里使用了双冒号将标识符和变种分隔开。由于现在这两个值 `IpAddrKind::V4` 与 `IpAddrKind::V6` 都是这同一类型：`IpAddrKind`，因此这就变得有用了。随后就可以，比如，定义一个取任意 `IpAddrKind` 类型值的函数：

```rust
fn route(ip_kind: IpAddrKind) {}
```

进而就能以这两个变种对这个函数进行调用了：

```rust
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
```

枚举的使用甚至还有更多好处。在还没有一种存储具体 IP 地址 *数据（data）* 的时候，就要进一步思考一下这里的 IP 地址类型；这是只知道 IP 地址数据为什么 *类别（king）*。根据在第 5 章中掌握的结构体知识，那么可能很想用下面清单 6-1 中的结构体来解决这个问题。

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

*清单 6-1：使用结构体 `struct` 来存储 IP 地址的数据与 `IpAddrKind` 变种*

这里已定义了有着两个字段的结构体 `IpAddr`：一个类型为 `IpAddrKind` （即先前定义的那个枚举）的 `kind` 字段，以及一个类型为 `String` 的 `address` 字段。这里有该结构体的两个实例。第一个是 `home`，而他有着与地址数据 `127.0.0.1` 关联的 `IpAddrKind::V4` 作为其 `kind` 的值。第二个实例为 `loopback`。这个实例则有不同的 `IpAddrKind` 变种作为其 `kind` 的值，即 `V6`，与 `kind` 关联的是地址 `::1`。由于这里使用了结构体将 `kind`  与 `address` 值捆绑在一起，因此现在这个 `IpAddrKind` 的变种就与那个 `String` 值关联起来了。

不过，仅使用一个枚举来表示这同一概念，就会更加简练：与其将枚举放在结构体内部，可将数据直接放在各个枚举变种里头。那么这新的 `IpAddr` 枚举定义，就是说 `V4` 与 `V6` 两个变种，将同时有着关联的 `String` 值：

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

这里把数据直接附加到枚举的各个变种上，因此就无需额外的结构体了。这里还更易于发现枚举工作原理的另一细节：所定义的各个枚举变种的名字，还成为了构造该枚举实例的函数。那就是说，`IpAddr::V4()` 现在是个取 `String` 参数并返回该 `IpAddr` 类型实例的函数调用了。作为定义枚举的结果，这里让这个构造函数自动就定义好了。

这里还有另一个使用枚举而非结构体的好处：各个变种可以有不同类型及数量的关联数据。版本四类型的 IP 地址，将始终有着四个会有着 `0` 到 `255` 之间值的数字部分。在希望将 `V4` 地址存储为四个 `u8` 值，而仍然将 `V6` 地址表示为一个 `String` 值时，那就没法用结构体了，而枚举则能轻易处理这样的情况：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

到这里，就已经给出了好几种定义用于存储版本四和版本六 IP 地址的数据结构了。然而事实表明，想要存储 IP 地址，及对这些 IP 地址所属类别进行编码是如此普遍，以致 [标准库就有一个可加以使用的定义](https://doc.rust-lang.org/std/net/enum.IpAddr.html)！下面就来看看，标准库是怎样定义 `IpAddr` 的：他有着与这里曾定义和使用过的相同枚举和变种，不过标准库是将地址数据，以两个不同结构体的形式，嵌入到变种里的，对两个枚举变种，定义了不同的结构体。

```rust
struct Ipv4Addr {
    // --跳过--
}

struct Ipv4Addr {
    // --跳过--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

这段代码说明可将任何类别的数据放在枚举变种里面：比如字符串、数字类型，或结构体等等。甚至可以包含另一枚举！还说明了，标准库类型，通常也并不比咱们自己编写的代码复杂多少。

请注意，由于这里不曾将标准库的 `IpAddr` 定义带入到这里的作用域，因此即使标准库包含了一个 `IpAddr` 的定义，这里也仍然可以毫无冲突地创建与使用自己的 `IpAddr` 定义。在第 7 章就会讲到有关带入类型到作用域的问题。

来看看下面清单 6-2 中另一个枚举的示例：这个枚举有着嵌入到其各个变种中的种类繁多的类型。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}
```

*清单 6-2：每个变种都存储了不同数量和类型值的 `Message` 枚举*

这个枚举有着四个带有不同类型数据的变种：

- `Quit` 变种完全没有与其关联的数据；
- `Move` 变种像结构体一样，有着两个命名的字段；
- `Write` 变种包含了单个 `String`；
- `ChangeColor` 编程包含了三个 `i32` 的值。

定义一个有着一些如上面清单 6-2 中变种的枚举，与定义不同种类的结构体定义类似，不同在于枚举未使用关键字 `struct`，且所有变种在 `Message` 类型下组织在了一起。下面这些结构体，就可保存之前各个枚举变种所保存的那些同样数据：

```rust
struct QuitMessage; // 单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);    // 元组结构体
struct ChangeColorMessage(i32, i32, i32);   //  元组结构体
```

不过假如这里使用了不同的、有着各自类型的结构体，那么就无法轻易地定义出一个接收原本在清单 6-2 中定义的、单一类型的  `Message` 枚举那样的，接收全部这些类别消息的函数了。

枚举与结构体之间，还有另外一个相似点：正如在结构体上使用 `impl` 关键字定义出一些方法，在枚举上定义方法也是可以的。下面就是一个可定义在这里的 `Message` 枚举上、名为 `call` 的方法：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 方法体将定义在这里
    }
}

fn main() {

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

方法体将使用 `self`，来获取方法调用所在变种实例值。在此示例中，已创建了一个有着值 `Message::Write(String::from("hello"))` 的变量 `m`，而那就是在 `m.call()` 运行时，`call` 方法体中的那个 `self`。

下面就来看看标准库中另一个甚为常见和有用的枚举：`Option`。


### `Option` 枚举及其超越空值的诸多优点

**The `Option` Enum and Its Advantages Over Null Values**

本小节会探讨 `Option` 的案例研究，`Option` 是由标准库定义的另一个枚举。`Option` 类型编码了某个值可能会是某个物件，或可能什么都不属于的这种甚为常见的场景（the `Option` type encodes the very common scenario in which a value could be something or it could be nothing）。比如在请求某个含有一些项目的清单的第一个项目时，就会得到一个值。而在请求某个空清单的第一个项目时，则会什么也得不到。以类型系统字眼，来表达这个概念，就表示编译器能够对，是否已处理了本应处理的全部情形，进行检查；此项功能可阻止那些在其他编程语言中极为常见的代码错误。

编程语言的设计，通常要考量包含哪些特性，而要排除哪些特性也至关重要。Rust 没有许多其他语言都有的空值特性。*空值（Null）* 是一个表示此处无值的值。在带有 `null` 的那些语言中，变量总是会处于下面两个状态之一：空值或非空值。


