use std::collections::LinkedList;

fn main() {
    println!("Hello, world!");
}

pub struct JsRuntime {
    // atom related
    pub atom_hash_size: u32,
    pub atom_count: u32,
    pub atom_size: u32,
    pub atom_count_resize: u32,
    pub atom_hash: (),
    pub atom_array: (),

    pub context_list: LinkedList<()>,

    pub gc_obj_list: LinkedList<()>,
    pub gc_zero_ref_count_list: LinkedList<()>,
    pub tmp_obj_list: LinkedList<()>,

    pub shape_hash_bits: u32,
    pub shape_hash_size: u32,
    pub shape_hash_count: u32,
    pub shape_hash: (),

    pub job_list: LinkedList<()>,
}

pub struct JsContext {
    // TODO: how do we natively represent gc header in rust?
    // Gc<T> does provide a pointer interface, but how to we trace/record
    // that in the runtime heap?
    //
    // Instead we want something like `Local<T>`, `Global<T>`, `TracedReference<T>`,
    // `Weak<T>` in rusty_v8.
    //
    // The lifetime Local<'s, T> is a good design to limit the value inside a HandleScope.
    // But how do we integrate the scope mechanism into quickjs like? quickjs seem not to
    // have any persistent root like design. Need to investigate the difference of the two
    // and determine whether we need to switch to tracing gc.
    //
    // Also, tracing gc Local<T>, Global<T> seem more native to rust ideas.
    pub header: (),
}

impl JsContext {
    pub fn create_function(&mut self, _fd: ()) -> () {
        todo!()
    }
}

pub struct JsFunctionBytecode {
    pub header: (),
    pub is_strict_mode: bool,
    pub has_prototype: bool,
    // TODO
}

pub struct JsClassId(pub u32);
pub struct JsAtom(pub u32);

pub enum JsTag {
    BigInt = -9,
    Symbol = -8,
    String = -7,
    StringRope = -6,
    Module = -3,
    FunctionBytecode = -2,
    Object = -1,
    Int = 0,
    Bool = 1,
    Null = 2,
    Undefined = 3,
    Uninitialized = 4,
    CatchOffset = 5,
    Exception = 6,
    ShortBigInt = 7,
    Float64 = 8,
}

impl JsTag {
    pub const FIRST: Self = Self::BigInt;
}

pub enum JsAtomType {
    String = 1,
    GlobalSymbol,
    Symbol,
    Private,
}

pub enum JsStringKind {
    Normal,
    Slice,
    Indirect,
}

pub enum JsClosureType {
    Local,
    Arg,
    Ref,
    GlobalRef,
    GlobalDecl,
    Global,
    ModuleDecl,
    ModuleImport,
}
