use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Jaj" => "Err",
        "Jó" => "Ok",
        "Zsinór" => "String",
        "Hasítótábla" => "HashMap",
        "Alapértelmezett" => "Default",
        "Hiba" => "Error",
        "Talán" => "Option",
        "Néhány" => "Some",
        "Semennyi" => "None",
        "Eredmény" => "Result",
        "Önmaga" => "Self",
        "sorkiír" => "println",
        "félbeszakít" => "break",
        "aszinkron" => "async",
        "megvár" => "await",
        "ciklus" => "loop",
        "mozgat" => "move",
        "láda" => "crate",
        "elérhetetlen_kód" => "unreachable_code",
        "mint" => "as",
        "állandó" => "const",
        "jellemvonás" => "trait",
        "veszélyes" => "unsafe",
        "ebben" => "in",
        "ebből" => "from",
        "dinamikus" => "dyn",
        "kibont" => "unwrap",
        "alapértelmezett" => "default",
        "mint_referencia" => "as_ref",
        "bk" => "io",
        "külső" => "extern",
        "hamis" => "false",
        "függvény" => "fn",
        "remek" => "super",
        "beszúr" => "insert",
        "megszerez" => "get",
        "megenged" => "allow",
        "bazdmeg" | "kurva_anyját" | "pánik" => "panic",
        "modul" => "mod",
        "megváltoztatható" => "mut",
        "új" => "new",
        "ahol" => "where",
        "minden" | "ehhez" => "for",
        "megszerez_vagy_beilleszt_ezzel" => "get_or_insert_with",
        "fő" => "main",
        "nyilvános" => "pub",
        "nincs" => None?,
        "visszatérít" => "return",
        "kivitelezés" => "impl",
        "referencia" => "ref",
        "egyeztet" => "match",
        "ha" => "if",
        "különben" => "else",
        "önmaga" => "self",
        "legyen" => "let",
        "statikus" => "static",
        "struktúra" => "struct",
        "elvárom_hogy" => "expect",
        "amíg" => "while",
        "használj" => "use",
        "ebbe" => "into",
        "igaz" => "true",
        "felsorolás" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rozsda(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
