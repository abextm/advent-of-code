use proc_macro::TokenStream;
use std::sync::OnceLock;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use syn::parse::{Parse, ParseStream};
use syn::{Lit, LitStr, Token};

pub enum Attr {
	Day(u8),
	Part(u8, Option<String>),
}

impl Parse for Attr {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ident: syn::Ident = input.parse()?;
		let key = ident.to_string();

		if let Some(day_str) = key.strip_prefix("day") {
			Ok(Attr::Day(day_str.parse().map_err(|e| input.error(format!("failed to parse day: {}", e)))?))
		} else if let Some(part_str) = key.strip_prefix("part") {
			let part = match part_str {
				"1" => 1,
				"2" => 2,
				part => return Err(input.error(format!("invalid part {}", part))),
			};

			let val = if input.peek(Token![=]) {
				let _: Token![=] = input.parse()?;
				if input.peek(syn::Lit) {
					Some(match input.parse()? {
						Lit::Str(v) => v.value(),
						Lit::ByteStr(v) => format!("{:?}", v.value()),
						Lit::CStr(v) => format!("{:?}", v.value()),
						Lit::Byte(v) => format!("{:?}", v.value()),
						Lit::Char(v) => format!("{:?}", v.value()),
						Lit::Int(v) => format!("{}", v.base10_digits()),
						Lit::Float(v) => format!("{}", v.base10_digits()),
						Lit::Bool(v) => format!("{:?}", v.value()),
						lit => return Err(input.error(format!("invalid value {:?}", lit))),
					})
				} else {
					Some(input.to_string())
				}
			} else {
				None
			};

			Ok(Attr::Part(part, val))
		} else {
			Err(input.error("expected dayN, partN or yearN"))
		}
	}
}


impl Attr {
	pub fn get_day<'a>(attrs: impl Iterator<Item=&'a Self>) -> syn::Result<u8> {
		if let Some(day) = attrs.filter_map(|a| if let Attr::Day(d) = a { Some(*d) } else { None }).next() {
			Ok(day)
		} else {
			let v: LitStr = syn::parse(TokenStream::from(quote! { module_path!() }).expand_expr().unwrap()).unwrap();

			static DAY_RE: OnceLock<Regex> = OnceLock::new();
			let day_re = DAY_RE.get_or_init(|| Regex::new(r"d(?:ay)?([0-9]+)").unwrap());
			if let Some(cap) = day_re.captures(&v.value()) {
				Ok(cap.get(1).unwrap().as_str().parse().unwrap())
			} else {
				Err(syn::Error::new(Span::call_site(), "unable to get day, no dayN param or guessable file name"))
			}
		}
	}

	pub fn get_part<'a>(attrs: impl Iterator<Item=&'a Self>, part: u8) -> Option<&'a Option<String>> {
		attrs.filter_map(|a| match a {
			Attr::Part(p, v) => if *p == part { Some(v) } else { None },
			_ => None,
		}).next()
	}
}