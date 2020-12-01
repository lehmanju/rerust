//pub mod combinators;
//pub mod fold;
//pub mod signal;
//pub mod var;

mod test {

    // fn fold1
    // fn fold2
    // fn fold3
    // fn map

    // (var1 -> fold1), (var1 -> fold2), (fold1 -> map), (fold2 -> map), (map -> fold3)

    #[derive(Clone)]
    pub struct ReactiveData {
        fold1: u32,
        fold2: String,
        fold3: String,
        var1: String,
    }

    pub struct ReactiveChange {
        fold1: bool,
        fold2: bool,
        fold3: bool,
        var1: bool,
        map: bool,
    }

    pub struct Evaluation {
        change: ReactiveChange,
        data: ReactiveData,
    }

    pub struct Observers {
        fold1: Vec<Box<dyn Fn(u32)>>,
        fold2: Vec<Box<dyn Fn(String)>>,
        fold3: Vec<Box<dyn Fn(String)>>,
        var1: Vec<Box<dyn Fn(String)>>,
    }

    fn reduce(data: ReactiveData) -> Evaluation {
        //let fold1_new = fold1(data.var1)
        // cmp fold1_new fold1_old
        //let fold2_new = fold2(data.var1)
        // cmp
        //let map_new = map(fold1_new, fold2_new)
        // cmp
        //let fold3_new = fold3(map_new)
        //
        //return evaluation
        todo!()
    }

    fn call_observers(eval: Evaluation, observers: &Observers) {}

    // loop: reduce, call_observers
}

mod compiler {
    use petgraph::{Directed, Graph};
    use std::{any::TypeId, rc::Rc};

    trait Reactive {
        type Item;
        fn function(&self) -> &syn::Expr;
    }

    trait Source {
        type Item;
        fn reactives_mut(&mut self) -> &mut Vec<Rc<dyn Reactive<Item = Self::Item>>>;
    }

    trait Stateful {
        type Item;
        fn initial(&self) -> &syn::Expr;
    }

    struct Var<T>
    where
        T: Clone + PartialEq,
    {
        initial: (T, syn::Expr),
        reactives: Vec<Rc<dyn Reactive<Item = T>>>,
    }

    impl<T> Source for Var<T> where T: Clone + PartialEq {
        type Item = T;
        fn reactives_mut(&mut self) -> &mut Vec<Rc<dyn Reactive<Item = Self::Item>>> {
        &mut self.reactives
        }

    }

    impl<T> Stateful for Var<T> where T: Clone + PartialEq {
        type Item = T;
        fn initial(&self) -> &syn::Expr {
            &self.initial.1
        }
    }

    struct Fold<I, T>
    where
        I: Clone + PartialEq,
        T: Clone + PartialEq,
    {
        initial: (T, syn::Expr),
        function: (Box<dyn Fn(T, I) -> T>, syn::Expr),
        reactives: Vec<Rc<dyn Reactive<Item = T>>>,
    }

    impl<I, T> Reactive for Fold<I, T>
    where
        T: Clone + PartialEq,
        I: Clone + PartialEq,
    {
        type Item = I;
        fn function(&self) -> &syn::Expr {
            &self.function.1
        }
    }

    impl<I, T> Source for Fold<I, T>
    where
        T: Clone + PartialEq,
        I: Clone + PartialEq,
    {
        type Item = T;
        fn reactives_mut(&mut self) -> &mut Vec<Rc<dyn Reactive<Item = Self::Item>>> {
            &mut self.reactives
        }
    }

    impl<I, T> Stateful for Fold<I, T>
    where
        T: Clone + PartialEq,
        I: Clone + PartialEq,
    {
        type Item = T;
        fn initial(&self) -> &syn::Expr {
            &self.initial.1
        }
    }

    trait SourceExt {
        type Item: Clone + PartialEq;
        fn fold<A, F>(
            &mut self,
            initial: (A, syn::Expr),
            function: (F, syn::Expr),
        ) -> &mut Fold<Self::Item, A>
        where
            A: Clone + PartialEq,
            F: Fn(A, Self::Item) -> A;
    }

    impl<T> SourceExt for T
    where
        T: Source,
        T::Item: Clone + PartialEq,
    {
        type Item = T::Item;
        fn fold<A, F>(
            &mut self,
            initial: (A, syn::Expr),
            function: (F, syn::Expr),
        ) -> &mut Fold<Self::Item, A>
        where
            A: Clone + PartialEq,
            F: Fn(A, Self::Item) -> A,
        {
            todo!()
        }
    }

    macro_rules! var {
        ($initial:expr, $t:ty) => {
            struct Var<$t> {
                initial: $t
            }
        };
    }

    fn test() {
        let var = Var {
            initial: (
                0u32,
                syn::Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(syn::LitInt::new("0u32", proc_macro2::Span::call_site())),
                }),
            ),
            reactives: vec![],
        };
        /*var.fold((0u32, syn::Expr::Lit(syn::ExprLit {
            attrs: vec![],
            lit: syn::Lit::Int(syn::LitInt::new("0u32", proc_macro2::Span::call_site())),
        }), (|))
        let var = var!(initial); 
        let fold = fold!(var, initial, fn); // generate struct and impls for fold,  add fold to mutable reference var, return mutable reference fold
        let map = map!(var, fold, var2, fn); // generate struct and impl for map, consume all mutable references, return mutable reference to map

        
        */
    }
}
