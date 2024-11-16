#![feature(proc_macro_expand)]

mod attr;

use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{ParseStream, Parser};
use syn::{parse_macro_input, FnArg, ItemConst, ItemFn, Pat, Token, Type};
use syn::__private::Span;
use syn::punctuated::Punctuated;
use syn::Result;
use crate::attr::Attr;


///
/// #[aoc]
/// fn pt1(input: &str) -> usize {
///   input.replace("problem", "solution")
/// }
///
///

#[proc_macro_attribute]
pub fn aoc(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let parse = move |item: ParseStream| {
		let mut p = AOCBuilder {
			attrs: Parser::parse(Punctuated::<Attr, Token![,]>::parse_terminated, attr)?,
			items: vec![item.cursor().token_stream()],
		};

		p.parse(item)?;

		let items = p.items;
		Ok(quote! { #(#items)* })
	};

	parse_macro_input!(item with parse).into()
}

struct AOCBuilder {
	attrs: Punctuated<Attr, Token![,]>,
	items: Vec<TokenStream>,
}


impl AOCBuilder {
	fn parse(&mut self, input: ParseStream) -> Result<()> {
		if input.peek(Token![fn]) {
			let itemfn: ItemFn = input.parse()?;
			let fnname = &itemfn.sig.ident;
			let (call, multipart): (Box<dyn Fn(u8) -> TokenStream>, _) = if let Some(FnArg::Typed(arg)) = itemfn.sig.inputs.get(1) {
				let ident = if let Pat::Ident(t) = arg.pat.as_ref() {
					&t.ident
				} else {
					return Err(syn::Error::new_spanned(&arg.pat, "expected ident argument"));
				};
				let arg = constant_for_arg(ident, &arg.ty)?;
				(Box::new(move |p| {
					let arg = arg(p);
					quote! { #fnname(input, #arg) }
				}), true)
			} else {
				(Box::new(|_| quote! { #fnname(input) }), false)
			};

			let day = Attr::get_day(self.attrs.iter())?;

			let part1 = Attr::get_part(self.attrs.iter(), 1).cloned();
			let part2 = Attr::get_part(self.attrs.iter(), 2).cloned();

			if !multipart && part1.is_some() && part2.is_some() {
				return Err(input.error("Marked with part 1 and 2 but no arguments"));
			}

			if multipart || part1.is_some() {
				self.generate_solution(fnname, day, 1, call(1), part1);
			}
			if multipart || part2.is_some() {
				self.generate_solution(fnname, day, 2, call(2), part2);
			}

			Ok(())
		} else if input.peek(Token![const]) {
			let itemconst: ItemConst = input.parse()?;

			let day = Attr::get_day(self.attrs.iter())?;
			let part1 = Attr::get_part(self.attrs.iter(), 1).cloned();
			let part2 = Attr::get_part(self.attrs.iter(), 2).cloned();

			let ident = itemconst.ident;

			if let Some(Some(v)) = part1 {
				self.generate_test(day, 1, &ident.to_string(), quote! { ::std::option::Option::Some(#ident) }, &v);
			}
			if let Some(Some(v)) = part2 {
				self.generate_test(day, 2, &ident.to_string(), quote! { ::std::option::Option::Some(#ident) }, &v);
			}

			Ok(())
		} else {
			Err(input.error("expected a function or const str"))
		}
	}

	fn generate_solution(&mut self, fnname: &Ident, day: u8, part: u8, call: TokenStream, test: Option<Option<String>>) {
		let name = Ident::new(&format!("__abex_aoc_solution_{}_{}", fnname.to_string(), part), Span::call_site());

		self.items.push(quote! {
			#[::aoc_helper::internal::linkme::distributed_slice(::aoc_helper::internal::SOLUTIONS)]
			#[linkme(crate=::aoc_helper::internal::linkme)]
			static #name: ::aoc_helper::Solution = ::aoc_helper::Solution {
				day: #day,
				part: #part,
				solve: |input| {
					let result = #call;
					::std::format!("{:?}", result)
				},
			};
		});

		if let Some(Some(t)) = test {
			self.generate_test(day, part, "real", quote! { ::std::option::Option::None }, &t);
		}
	}

	fn generate_test(&mut self, day: u8, part: u8, name: &str, input: TokenStream, test: &str) {
		let static_name = Ident::new_raw(&format!("__abex_aoc_test_ref_{}_{}", name, part), Span::mixed_site());
		let fn_name = Ident::new_raw(&format!("__abex_aoc_test_{}_{}", name, part), Span::mixed_site());
		let result = Literal::string(&test);
		self.items.push(quote! {
			#[::aoc_helper::internal::linkme::distributed_slice(::aoc_helper::internal::TESTS)]
			#[linkme(crate=::aoc_helper::internal::linkme)]
			static #static_name: ::aoc_helper::Test = ::aoc_helper::Test {
				day: #day,
				part: #part,
				name: #name,
				input: #input,
				result: #result,
			};

			#[test]
			fn #fn_name() {
				#static_name.run();
			}
		})
	}
}

fn constant_for_arg(ident: &Ident, ty: &Type) -> Result<impl Fn(u8) -> TokenStream> {
	enum Mode {
		Num,
		Part1Bool,
		Part2Bool,
	}
	let mode = if let Type::Path(p) = ty {
		match p.path.segments.last().unwrap().ident.to_string().as_str() {
			"u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64" | "isize" => Mode::Num,
			"bool" => {
				let name = ident.to_string();
				if name.ends_with("1") {
					Mode::Part1Bool
				} else if name.ends_with("2") {
					Mode::Part2Bool
				} else {
					return Err(syn::Error::new_spanned(p, "boolean arg must end in 1 or 2"))
				}
			}
			_ => return Err(syn::Error::new_spanned(p, "unsupported type"))
		}
	} else {
		return Err(syn::Error::new_spanned(ty, "unsupported type"))
	};

	Ok(move |part| {
		match mode {
			Mode::Num => Literal::u8_unsuffixed(part).into_token_stream(),
			Mode::Part1Bool => bool(part == 1),
			Mode::Part2Bool => bool(part == 2),
		}
	})
}

fn bool(v: bool) -> TokenStream {
	if v {
		quote! { true }
	} else {
		quote! { false }
	}
}
