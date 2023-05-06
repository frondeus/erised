use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use quote::TokenStreamExt;
use syn::buffer::TokenBuffer;

pub struct TokenMatcher;

impl TokenMatcher {
    pub fn gen(&self, stream: TokenStream) -> TokenStream {
        let buf = TokenBuffer::new2(stream);
        let mut cursor = buf.begin();
        self.gen_inner(&mut cursor)
    }

    const UNTOUCHABLE: &'static [&'static str] = &["usize", "bool", "u32"];

    fn gen_inner(&self, cursor: &mut syn::buffer::Cursor) -> TokenStream {
        let mut new = TokenStream::new();
        while let Some((token_tree, next)) = cursor.token_tree() {
            *cursor = next;
            match token_tree {
                TokenTree::Ident(ident) => {
                    if ident == "String" || ident == "PathBuf" {
                        new.append_all(quote!(&'static str));
                    } else if ident == "HashMap" {
                        new.append_all(quote!(&'static));
                        let (_, mut next) = cursor.token_tree().expect("Expected < token");
                        let inner = self.gen_inner(&mut next);

                        *cursor = next;

                        new.append(Group::new(
                            Delimiter::Bracket,
                            quote!(
                                (#inner)
                            ),
                        ));
                    } else if ident == "Vec" {
                        new.append_all(quote!(&'static));

                        let (_, mut next) = cursor.token_tree().expect("Expected < token");
                        let inner = self.gen_inner(&mut next);

                        *cursor = next;

                        new.append(Group::new(Delimiter::Bracket, inner));
                    } else if ident == "Box" || ident == "Arc" || ident == "Weak" {
                        new.append_all(quote!(fn () -> ));

                        let (_, mut next) = cursor.token_tree().expect("Expected < token");
                        let inner = self.gen_inner(&mut next);
                        *cursor = next;

                        new.append_all(inner);
                    } else if ident == "Option" {
                        let (_, mut next) = cursor.token_tree().expect("Expected < token");
                        let inner = self.gen_inner(&mut next);
                        *cursor = next;

                        new.append_all(quote!(#ident < #inner > ));
                    } else if Self::UNTOUCHABLE.contains(&ident.to_string().as_str()) {
                        new.append(ident);
                    } else {
                        let new_ident = Ident::new(&format!("Static{ident}"), ident.span());

                        new.append(new_ident);
                    }
                }
                TokenTree::Group(g) => {
                    let inner = g.stream();
                    let inner = self.gen(inner);
                    new.append(Group::new(g.delimiter(), inner));
                }
                TokenTree::Punct(p) => {
                    if p.as_char() == '>' {
                        return new;
                    }
                    new.append(p);
                }
                TokenTree::Literal(l) => {
                    new.append(l);
                }
            }
        }
        new
    }
}
