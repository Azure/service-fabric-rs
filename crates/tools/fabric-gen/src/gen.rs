use std::collections::HashMap;

use windows_metadata::{
    InterfaceImpl, ParamAttributes, Reader, RowReader, Type, TypeDef, TypeKind, TypeName, TypeRef,
};

#[derive(Default, Clone)]
pub struct InterfaceLayout {
    type_entry: TypeEntry,
    func_pairs: HashMap<String, FuncPair>, // key is the function name without begin/end
}

#[derive(Default, Clone)]
struct ParamEntry {
    name: String,
    type_entry: TypeEntry,
    attr: ParamAttributes,
}

#[derive(Default, Clone, Debug)]
struct TypeEntry {
    ns: String,
    name: String,
    prefix: String, // mut & etc stuff
}

impl TypeEntry {
    fn from_type_reader(t: &Type, r: &Reader, attr: ParamAttributes) -> TypeEntry {
        let empty_str = "".to_string();
        let (name, ns, qualifer) = match t {
            Type::TypeDef(x, _) => {
                let te = TypeEntry::from_typedef_reader(x, r);
                let tk = r.type_def_kind(*x);
                // itf param should be ref
                let prefix = match tk {
                    TypeKind::Interface => "&",
                    _ => "",
                };
                (te.name, te.ns, prefix.to_string())
            }
            Type::PWSTR => {
                // twist the type based on attr
                let t = match attr {
                    ParamAttributes::In => "PCWSTR",
                    _ => "PWSTR",
                };
                (t.to_string(), "::windows_core".to_string(), empty_str)
            }
            Type::U32 => ("u32".to_string(), "".to_string(), empty_str),
            Type::U16 => ("u16".to_string(), "".to_string(), empty_str),
            Type::U8 => ("u8".to_string(), "".to_string(), empty_str),
            Type::I64 => ("i64".to_string(), "".to_string(), empty_str),
            Type::F64 => ("f64".to_string(), "".to_string(), empty_str),
            Type::MutPtr(t, _) => {
                let q = match attr {
                    ParamAttributes::In => "&",
                    _ => "&mut",
                };
                let te = TypeEntry::from_type_reader(t, r, attr);
                (te.name, te.ns, format!("{}{}", q, te.prefix)) // te prefix is empty always
            }
            Type::HRESULT => (
                "HRESULT".to_string(),
                "::windows_core".to_string(),
                empty_str,
            ),
            Type::GUID => ("GUID".to_string(), "::windows_core".to_string(), empty_str),
            Type::TypeRef(x) => match x {
                windows_metadata::TypeDefOrRef::TypeDef(x) => {
                    let te = TypeEntry::from_typedef_reader(x, r);
                    (te.name, te.ns, te.prefix)
                }
                windows_metadata::TypeDefOrRef::TypeRef(x) => {
                    let te = TypeEntry::from_typeref(x, r);
                    (te.name, te.ns, te.prefix)
                }
                windows_metadata::TypeDefOrRef::TypeSpec(_) => {
                    panic!("TypeSpec not implemented")
                }
            },
            Type::Void => ("c_void".to_string(), "::core::ffi".to_string(), empty_str),
            _ => {
                panic!("{:?} not supported", t)
            }
        };
        TypeEntry {
            ns,
            name,
            prefix: qualifer,
        }
    }

    // custom convert for namespace
    fn normalize_namespace(ns: &str) -> String {
        // replace . with :: for rust
        let ns_str = ns.replace('.', "::");
        ns_str.replace("Windows", "windows") // crate name is lowercase
    }

    // fabric types
    fn from_typedef_reader(t: &TypeDef, r: &Reader) -> TypeEntry {
        let name = r.type_def_name(*t);
        let ns = r.type_def_namespace(*t);
        let ns_str = TypeEntry::normalize_namespace(ns);
        TypeEntry {
            ns: format!("::fabric_base::{}", ns_str),
            name: name.to_string(),
            prefix: "".to_string(),
        }
    }

    fn from_typeref(t: &TypeRef, r: &Reader) -> TypeEntry {
        let name = r.type_ref_name(*t);
        let ns = r.type_ref_namespace(*t);
        let ns_str = TypeEntry::normalize_namespace(ns);
        TypeEntry {
            ns: ns_str,
            name: name.to_string(),
            prefix: "".to_string(),
        }
    }
}

#[derive(Default, Clone)]
pub struct FuncPair {
    begin: String,
    begin_param: Vec<ParamEntry>, // name + type
    begin_ret: TypeEntry,         // type
    end: String,
    end_param: Vec<ParamEntry>,
    end_ret: TypeEntry, // type
}

impl InterfaceLayout {
    // removes redundant types for code gen
    pub fn to_async(&self) -> InterfaceLayout {
        // make a copy
        let mut itf: InterfaceLayout = self.clone();

        for pair in itf.func_pairs.values_mut() {
            // callback and ctx are removed
            pair.begin_param.retain(|x| {
                x.type_entry.name != "IFabricAsyncOperationCallback"
                    && x.type_entry.name != "IFabricAsyncOperationContext"
            });
            // ctx is removed
            pair.end_param
                .retain(|x| x.type_entry.name != "IFabricAsyncOperationContext");
        }
        itf
    }

    // merge functions from other into self
    // this is for merging parent functions
    pub fn merge(&mut self, other: &InterfaceLayout) {
        self.func_pairs.extend(other.func_pairs.clone());
    }

    // verify all func pair are ok
    // some interfaces have none matching begin and end
    fn check_integrity(&self) {
        for (f_name, pair) in &self.func_pairs {
            assert!(
                pair.begin.contains(f_name),
                "f_name {}, begin {}",
                f_name,
                pair.begin
            );
            assert!(
                pair.end.contains(f_name),
                "f_name {}, end {}",
                f_name,
                pair.end
            );
        }
    }

    // some sf api end has new return type but the begin is the same
    // so the func pair will have empty begin function
    fn patch_end_func_without_begin(&mut self) {
        // let mut missing_funcs = Vec::<String>::new();
        // for (f_name, pair) in &self.func_pairs{
        //     if !pair.begin.contains(f_name){
        //         // end api has new version
        //         missing_funcs.push(f_name.clone());
        //     }
        // }

        // for f in missing_funcs {
        //     let pair = self.func_pairs.get_mut(&f).unwrap();
        //     assert!(pair.end.contains(&f));

        //     // trim the (version) number from f
        //     // f.
        // }

        // for now we remove such api
        let mut missing_funcs = Vec::<String>::new();
        for (f_name, pair) in &self.func_pairs {
            if !pair.begin.contains(f_name) {
                // end api has new version
                missing_funcs.push(f_name.clone());
            }
        }
        for f in missing_funcs {
            let ok = self.func_pairs.remove(&f);
            assert!(ok.is_some());
        }
    }

    // remove functions
    pub fn exclude_funcs(&mut self, fs: &Vec<&str>) {
        for f in fs {
            let ok = self.func_pairs.remove(&f.to_string());
            assert!(ok.is_some());
        }
    }
}

pub struct Parser<'a> {
    r: &'a Reader<'a>,
}

// return is begin and the trimmed val.
fn trim_begin_end_function_name(name: &str) -> Option<(bool, String)> {
    let begin_removed = name.strip_prefix("Begin");
    let end_removed = name.strip_prefix("End");
    if let Some(x) = begin_removed {
        return Some((true, String::from(x)));
    }
    if let Some(x) = end_removed {
        return Some((false, String::from(x)));
    }
    None
}

impl Parser<'_> {
    pub fn new<'a>(r: &'a Reader<'a>) -> Parser<'a> {
        Parser { r }
    }

    pub fn get_type_def(&self, tn: &TypeName) -> TypeDef {
        let mut type_def = self.r.get_type_def(*tn).peekable();
        assert!(type_def.peek().is_some());

        // find the type def for this interface.
        let mut x_td: Option<TypeDef> = None;

        for td in type_def {
            let ttn = self.r.type_def_type_name(td);
            if ttn.name != tn.name {
                continue;
            }
            x_td = Some(td);
        }

        assert!(x_td.is_some(), "typedef not found");
        x_td.unwrap()
    }

    fn get_parent_type(&self, td: &TypeDef) -> Option<TypeDef> {
        let parents: Vec<InterfaceImpl> = self.r.type_def_interface_impls(*td).collect();
        if parents.is_empty() {
            return None;
        }
        // get the first one
        let p = parents[0];
        let t = self.r.interface_impl_type(p, &[]);
        match t {
            Type::TypeDef(x, _) => Some(x),
            _ => None,
        }
    }

    pub fn get_interface_layout(&self, td: TypeDef) -> InterfaceLayout {
        let r = self.r;

        let tk = r.type_def_kind(td);
        assert_eq!(tk, TypeKind::Interface);

        // find all methods
        let mut tmds = r.type_def_methods(td).peekable();
        assert!(tmds.peek().is_some());

        let mut func_pairs = HashMap::<String, FuncPair>::new();

        for tmd in tmds {
            let tmdn = r.method_def_name(tmd);
            let trimmed_name = trim_begin_end_function_name(tmdn);
            if trimmed_name.is_none() {
                continue;
            }

            // find or initialize the func pair entry
            let (is_begin, trimmed_name) = trimmed_name.unwrap();
            let fp_opt = func_pairs.get_mut(&trimmed_name);
            if fp_opt.is_none() {
                // insert the pair
                let new_fp = FuncPair::default();
                func_pairs.insert(trimmed_name.clone(), new_fp);
            }
            let fp_opt = func_pairs.get_mut(&trimmed_name);

            let fp = fp_opt.unwrap();
            // fill the entry
            (if is_begin { &mut fp.begin } else { &mut fp.end }).push_str(tmdn); // should be append empty str

            // fill method info
            let mut tmdps = r.method_def_params(tmd).peekable();
            assert!(tmdps.peek().is_some());
            for p in tmdps {
                let pn = r.param_name(p);
                let flag = r.param_flags(p);

                if pn.is_empty() {
                    // This might be self ptr
                    continue;
                }
                let pe = ParamEntry {
                    name: pn.to_string(),
                    type_entry: TypeEntry::default(),
                    attr: flag,
                };
                (if is_begin {
                    &mut fp.begin_param
                } else {
                    &mut fp.end_param
                })
                .push(pe);
            }

            let sig = r.method_def_signature(tmd, &[]);
            for (i, p) in sig.params.iter().enumerate() {
                let pev = if is_begin {
                    &mut fp.begin_param
                } else {
                    &mut fp.end_param
                };
                let pe = &mut pev[i];
                let p_type = TypeEntry::from_type_reader(p, r, pe.attr);
                pe.type_entry = p_type;
            }

            // ret type does not have attr
            let ret_type = TypeEntry::from_type_reader(&sig.return_type, r, ParamAttributes(0));

            if is_begin {
                fp.begin_ret = ret_type;
            } else {
                fp.end_ret = ret_type;
            }
        }
        InterfaceLayout {
            type_entry: TypeEntry::from_typedef_reader(&td, r),
            func_pairs,
        }
    }

    // get all functions including parents
    pub fn get_interface_layout_recursive(&self, tn: &TypeName) -> InterfaceLayout {
        let td = self.get_type_def(tn);
        let mut itf = self.get_interface_layout(td);

        let mut ptd_opt = self.get_parent_type(&td);
        while ptd_opt.is_some() {
            let ptd = ptd_opt.unwrap();
            let pitf = self.get_interface_layout(ptd);
            itf.merge(&pitf);
            // loop for the next parent
            ptd_opt = self.get_parent_type(&ptd);
        }
        itf.patch_end_func_without_begin();
        itf.check_integrity();
        itf
    }
}

#[cfg(test)]
mod tests {

    //use windows_metadata::{Type, TypeKind, TypeName};

    use windows_metadata::TypeName;

    use crate::{gen::TypeEntry, get_reader};

    use super::Parser;

    #[test]
    fn reading_test1() {
        let r = get_reader();
        let gen = Parser::new(r);
        let itf_layout = gen.get_interface_layout(gen.get_type_def(&TypeName {
            namespace: "Microsoft.ServiceFabric.FabricCommon",
            name: "IFabricAsyncOperationCallback",
        }));

        assert_eq!(itf_layout.type_entry.name, "IFabricAsyncOperationCallback");
        assert_eq!(
            itf_layout.type_entry.ns,
            "::fabric_base::Microsoft::ServiceFabric::FabricCommon"
        );

        let func_pairs = &itf_layout.func_pairs;
        assert_eq!(func_pairs.len(), 0);
    }

    #[test]
    fn reading_test2() {
        let r = get_reader();
        let gen = Parser::new(r);
        let itf_layout = gen.get_interface_layout(gen.get_type_def(&TypeName {
            namespace: "Microsoft.ServiceFabric.FabricCommon.FabricClient",
            name: "IFabricClusterManagementClient",
        }));

        assert_eq!(
            itf_layout.type_entry.ns,
            "::fabric_base::Microsoft::ServiceFabric::FabricCommon::FabricClient"
        );
        assert_eq!(itf_layout.type_entry.name, "IFabricClusterManagementClient");

        let func_pairs = &itf_layout.func_pairs;
        assert_eq!(func_pairs.len(), 2);
        let pair = func_pairs.get("NodeStateRemoved").unwrap();
        assert_eq!(pair.begin, "BeginNodeStateRemoved");

        let param = &pair.begin_param;
        assert_eq!(param.len(), 4);
        let param1 = &param[0];
        assert_eq!(param1.name, "nodeName");
        assert_eq!(param1.type_entry.name, "PCWSTR");
        assert_eq!(param1.type_entry.ns, "::windows_core");

        assert_eq!(pair.end_ret.name, "HRESULT");
        assert_eq!(pair.end_ret.ns, "::windows_core");

        {
            // async layout for generation
            let itf2 = itf_layout.to_async();
            let func_pairs = &itf2.func_pairs;
            let pair = func_pairs.get("NodeStateRemoved").unwrap();
            assert_eq!(pair.begin, "BeginNodeStateRemoved");
            let param = &pair.begin_param;
            // callback and ctx are removed.
            assert_eq!(param.len(), 4 - 2);
        }
    }

    #[test]
    fn reading_hierachy() {
        let r = get_reader();
        let gen = Parser::new(r);
        let td = gen.get_type_def(&TypeName {
            namespace: "Microsoft.ServiceFabric.FabricCommon.FabricClient",
            name: "IFabricClusterManagementClient2",
        });

        let ptd = gen.get_parent_type(&td).unwrap();
        let ptde = TypeEntry::from_typedef_reader(&ptd, r);
        assert_eq!(ptde.name, "IFabricClusterManagementClient");

        let pptd = gen.get_parent_type(&ptd);
        assert_eq!(pptd, None);
    }
}

pub mod code {

    use itertools::Itertools;
    use proc_macro2::TokenStream;
    use quote::quote;

    use super::{FuncPair, InterfaceLayout, ParamEntry};

    pub struct Gen {
        itf: InterfaceLayout,
    }

    impl Gen {
        pub fn new(itf: InterfaceLayout) -> Gen {
            Gen { itf }
        }

        pub fn gen(&self) -> TokenStream {
            self.gen_iterface()
        }

        fn gen_iterface(&self) -> TokenStream {
            let itf_type = &self.itf.type_entry.name;
            let itf_ns = &self.itf.type_entry.ns;
            let itf_full_type: syn::Expr =
                syn::parse_str(format!("{}::{}", itf_ns, itf_type).as_str()).unwrap();
            let wrap_ident = quote::format_ident!("{}Wrap", itf_type);

            // output method in alphabetic order
            let mut method_stream = TokenStream::new();
            for (name, pair) in self.itf.func_pairs.iter().sorted_by_key(|x| x.0) {
                method_stream.extend(self.gen_method(name, pair));
            }

            quote! {
                pub struct #wrap_ident
                {
                    com: #itf_full_type
                }

                impl Default for #wrap_ident {
                    fn default() -> Self {
                        Self::new()
                    }
                }

                impl #wrap_ident{
                    pub fn new() -> #wrap_ident {
                        #wrap_ident {
                            com: crate::sync::CreateLocalClient::<#itf_full_type>(),
                        }
                    }

                    #method_stream
                }
            }
        }

        fn gen_method(&self, name: &String, pair: &FuncPair) -> TokenStream {
            let f_name = quote::format_ident!("{}", name);
            let begin_params = self.gen_begin_params(&pair.begin_param);
            let begin_params_call_args = self.gen_begin_params_call_arg(&pair.begin_param);
            let ret_t = self.gen_return_type(&pair.end_param);

            let begin_f_ident = quote::format_ident!("{}", pair.begin);
            let end_f_ident = quote::format_ident!("{}", pair.end);

            quote! {
                pub fn #f_name(&self #begin_params) -> #ret_t{
                    let (tx, rx) = tokio::sync::oneshot::channel();

                    let callback = crate::sync::AwaitableCallback2::i_new(move |ctx| {
                        let res = unsafe { self.com.#end_f_ident(ctx) };
                        if tx.send(res).is_err()
                        {
                            // This can happen if user on the receiver end use cancel or select.
                            // Ideally user should always wait for result.
                            debug_assert!(false, "Receiver is dropped.");
                        }
                    });
                    let ctx = unsafe { self.com.#begin_f_ident(#begin_params_call_args &callback) };
                    if ctx.is_err() {
                        let (tx2, rx2) = tokio::sync::oneshot::channel();
                        tx2.send(Err(ctx.err().unwrap())).expect("fail to send tx2"); // This should never fail since rx2 is available
                        crate::sync::FabricReceiver::new(rx2)
                    } else {
                        crate::sync::FabricReceiver::new(rx)
                    }
                }
            }
        }

        fn gen_begin_params(&self, params: &Vec<ParamEntry>) -> TokenStream {
            let mut stream = TokenStream::new();

            for param in params {
                let te = &param.type_entry;
                let t_ns = if te.ns.is_empty() {
                    te.name.clone()
                } else {
                    format!("{}::{}", te.ns, te.name)
                };
                // param name
                let p_name = &param.name;
                let p_name_ident = quote::format_ident!("{}", p_name);

                let prefix = &te.prefix;

                // type with attribute
                let t_ns_prefix = format!("{} {}", prefix, t_ns);
                let type_exp: syn::Type = syn::parse_str(t_ns_prefix.as_str())
                    .unwrap_or_else(|_| panic!("{}", t_ns_prefix));

                let token = quote! {
                    ,#p_name_ident : #type_exp
                };

                stream.extend(token);
            }

            stream
        }

        // arguments to pass to begin call
        fn gen_begin_params_call_arg(&self, params: &Vec<ParamEntry>) -> TokenStream {
            let mut stream = TokenStream::new();

            for param in params {
                let p_name = &param.name;
                let p_name_ident = quote::format_ident!("{}", p_name);
                let token = quote! {
                    #p_name_ident,
                };
                stream.extend(token);
            }

            stream
        }

        fn gen_return_type(&self, params: &Vec<ParamEntry>) -> TokenStream {
            if params.is_empty() {
                quote! {
                    crate::sync::FabricReceiver<::windows_core::Result<()>>
                }
            } else {
                let pe = &params[0];
                let te = &pe.type_entry;
                let t: syn::Expr = if te.ns.is_empty() {
                    // this only works in limited cases
                    syn::parse_str(te.name.as_str()).unwrap()
                    // panic!("raw type return not supported ")
                } else {
                    syn::parse_str(format!("{}::{}", te.ns, te.name).as_str()).unwrap()
                };
                quote! {
                    crate::sync::FabricReceiver<::windows_core::Result<#t>>
                }
            }
        }
    }
}
