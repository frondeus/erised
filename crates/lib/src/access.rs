// use proc_macro2::TokenStream;
// use quote::quote;

// use crate::heap_types::Item;

// pub(crate) trait Access {
//     fn access(&self) -> TokenStream;
// }

// impl Access for Item {
//     fn access(&self) -> TokenStream {
//         match self {
//             Item::ExternCrate { .. } => quote!(todo!("ExternCrate")),
//             Item::Union(_) => quote!(todo!("Union")),
//             Item::Trait(_) => quote!(todo!("Trait")),
//             Item::TraitAlias(_) => quote!(todo!("TraitAlias")),
//             Item::Typedef(_) => quote!(todo!("Typedef")),
//             Item::OpaqueTy(_) => quote!(todo!("OpaqueTy")),
//             Item::Constant(_) => quote!(todo!("Constant")),
//             Item::Static(_) => quote!(todo!("Static")),
//             Item::ForeignType => quote!(todo!("ForeignType ")),
//             Item::Macro { .. } => quote!(todo!("Macro")),
//             Item::ProcMacro(_) => quote!(todo!("ProcMacro")),
//             Item::Primitive(_) => quote!(todo!("Primitive")),
//             Item::Module(_)
//             | Item::Import(_)
//             | Item::Impl(_)
//             | Item::Function(_)
//             | Item::AssocConst { .. }
//             | Item::AssocType { .. } => {
//                 println!("Access: {self:#?}");
//                 quote!(#self)
//             }
//             Item::Struct(strukt) => {
//                 let path = strukt.meta.get_formatted_path();
//                 quote!(
//                     <#path as Reflect>::TYPE_INFO
//                 )
//             }
//             Item::Enum(enum_) => {
//                 let path = enum_.meta.get_formatted_path();
//                 quote!(
//                     <#path as Reflect>::TYPE_INFO
//                 )
//             }
//         }
//     }
// }

// impl Access for crate::heap_types::Type {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// // impl Access for

// impl Access for crate::heap_types::FunctionPointer {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::GenericArgs {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::ExternalCrate {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::ItemSummary {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::Module {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for String {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::Identifiable {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::Path {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for Box<crate::heap_types::GenericArgs> {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::StructField {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::Discriminant {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::Deprecation {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }

// impl Access for crate::heap_types::Span {
//     fn access(&self) -> TokenStream {
//         crate::destruct::ToTokens::to_tokens(self)
//     }
// }
