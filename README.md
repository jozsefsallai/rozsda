# rozsda

![](https://github.com/jozsefsallai/rozsda/raw/fő/logo.png)

Aren't you _fáradt_ from writing Rust programs in English? Do you like saying
"bazdmeg" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Magyar touch to your
programs?

**rozsda** (Hungarian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Hungarian, using Hungarian keywords, Hungarian function
names, Hungarian idioms.

This has been designed to be used as the official programming language to
develop the future Hungarian sovereign operating system.

You don't feel at ease using only Hungarian words? Don't worry! Hungarian Rust
is fully compatible with English-Rust, so you can mix both at your convenience.

Here's an example of what can be achieved with Rozsda:

### trait and impl (aka jellemvonás és kivitelezés)

```rust
rozsda::rozsda! {
    használj std::collections::Hasítótábla mint Tábla;

    jellemvonás KulcsÉrték {
        függvény ír(&önmaga, kulcs: Zsinór, érték: Zsinór);
        függvény olvas(&önmaga, kulcs: Zsinór) -> Eredmény<Talán<&Zsinór>, Zsinór>;
    }

    statikus megváltoztatható SZÓTÁR: Talán<Tábla<Zsinór, Zsinór>> = Semennyi;

    struktúra Konkrét;

    kivitelezés KulcsÉrték ehhez Konkrét {
        függvény ír(&önmaga, kulcs: Zsinór, érték: Zsinór) {
            legyen szótár = veszélyes {
                SZÓTÁR.megszerez_vagy_beilleszt_ezzel(Alapértelmezett::alapértelmezett)
            };
            szótár.beszúr(kulcs, érték);
        }
        függvény olvas(&önmaga, kulcs: Zsinór) -> Eredmény<Talán<&Zsinór>, Zsinór> {
            ha legyen Néhány(szótár) = veszélyes { SZÓTÁR.mint_referencia() } {
                Jó(szótár.megszerez(&kulcs))
            } különben {
                Jaj("nincs szótár".ebbe())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[megenged(code_inaccessible)]
függvény másodlagos() {
    bazdmeg!("óh ne"); // for the true Hungarian experience
    kurva_anyját!("mi a rák"); // for friends who want to be more articulate
    pánik!("hozz egy sört"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. It's that egyszerű.

## Hozzájárulások

First of all, _köszönöm szépen_ for considering participating to this joke, the
Hungarian government will thank you later! Feel free to throw in a few
identifiers here and there, and open a pull-request against the `fő` (Hungarian
for `main`) branch.

Please don't introduce swear words, though: we will not excuse your Hungarian.

## Other languages

- French (original): [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)

## Nagy köszönetek

- [@bnjbvr](https://github.com/bnjbvr) for making the original macro.

## Licensz

[WTFPL](http://www.wtfpl.net/).
