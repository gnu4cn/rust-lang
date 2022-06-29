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


