use std::borrow::Cow;

pub enum Resolution<Local, Item> {
    Bound(Local),
    Item(Item),
}

pub enum Namespace {
    Type,
    Value,
}

pub trait Resolver {
    type BindingId;
    type ItemId;

    type Name;
    type Path;

    fn lookup_name(&self,
                   name: Self::Name,
                   ns: Namespace)
                   -> Option<Resolution<Self::BindingId, Self::ItemId>>;

    fn lookup_path(&self,
                   path: Self::Path,
                   ns: Namespace)
                   -> Option<Resolution<Self::BindingId, Self::ItemId>>;
}

pub trait Definition {
    type Name;
    fn introduces(&self) -> Iterator<Item=&Self::Name>;
}

// FIXME Some notes as I go...

//   * a Name can be a special case of a Path
//
//   * a Path can be regarded as being made up of a sequence of Names,
//   or at least Symbols/Identifiers (where a singleton is the special
//   case)
//
//   * a Path needs resolution too: namely, its root component may be
//   relative to some local import
//
//   * is it worth considering explicitly differentiating between Name
//   and Path, since a Name can refer to a formal parameter, while any
//   non-singleton path *always* resolves to an item from some module?
//
// If you want to parameterize over the Name/Path representation,
// strive to do it in a way that supports these relationships.

// Make sure to handle cases like:
//
// * fn foo<A: Trait1<B>, B: Trait2<A>>(x: (A, B)) where (A, B): Trait3 { }

pub trait SyntaxNode: Clone {
    type Name: Clone;
    type Path: Clone;
    type Scope: Clone;

    fn children(&self) -> Iterator<Item=Cow<Self>>;

    fn uses_names(&self) -> Iterator<Item=Cow<Self::Name>>;

    fn uses_paths(&self) -> Iterator<Item=Cow<Self::Path>>;

    fn in_scope(&self, scope: &Self::Scope) -> bool; // is this actually useful?

    fn provides_names(&self) -> Iterator<Item=(Cow<Self::Name>, Cow<Self::Scope>)>;
}

pub trait SyntaxForest: Clone {
    type Def: Definition<Name=Self::Name>;
    type Name: Clone;
    type Node: Clone;
    type Path: Clone;
    type Scope: Clone;

    fn roots(&self) -> Iterator<Item=Cow<Self::Node>>;
    fn children(&self, node: &Self::Node) -> Iterator<Item=Cow<Self::Node>>;
    fn uses_names(&self, node: &Self::Node) -> Iterator<Item=Cow<Self::Name>>;
    fn uses_paths(&self, node: &Self::Node) -> Iterator<Item=Cow<Self::Path>>;
    fn in_scope(&self, node: &Self::Node, scope: &Self::Scope) -> bool;
    fn provides_names(&self, node: &Self::Node) -> Iterator<Item=(Cow<Self::Name>, Cow<Self::Scope>)>;
    fn defines_items(&self, node: &Self::Node) -> Iterator<Item=&Self::Def>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
