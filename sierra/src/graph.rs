use std::fmt;

// A full sierra program.
#[derive(Clone, Debug)]
pub struct Program {
    // All code blocks of the program.
    pub blocks: Vec<Block>,
    // Descriptions of the functions - signature and entry point.
    pub funcs: Vec<Function>,
}

// Descriptor of a function.
#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<TypedVar>,
    pub res_types: Vec<Type>,
    pub entry: BlockId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedVar {
    pub id: Identifier,
    pub ty: Type,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identifier(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Type {
    pub name: String,
    pub args: Vec<TemplateArg>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        let mut iter = self.args.iter();
        if let Some(ta) = iter.next() {
            write!(f, "<{}", ta)?;
            for ta in iter {
                write!(f, ", {}", ta)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlockId(pub usize);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TemplateArg {
    Type(Type),
    Value(i64),
}

impl fmt::Display for TemplateArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplateArg::Type(t) => write!(f, "{}", t),
            TemplateArg::Value(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub invocations: Vec<Invocation>,
    pub exit: BlockExit,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for invc in &self.invocations {
            writeln!(f, "{};", invc)?;
        }
        writeln!(f, "{};", self.exit)
    }
}

#[derive(Clone, Debug)]
pub struct Invocation {
    pub ext: Extension,
    pub args: Vec<Identifier>,
    pub results: Vec<Identifier>,
}

impl fmt::Display for Invocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self.ext)?;
        self.args
            .iter()
            .take(1)
            .try_for_each(|n| write!(f, "{}", n.0))?;
        self.args
            .iter()
            .skip(1)
            .try_for_each(|n| write!(f, ", {}", n.0))?;
        write!(f, ") -> (")?;
        self.results
            .iter()
            .take(1)
            .try_for_each(|n| write!(f, "{}", n.0))?;
        self.results
            .iter()
            .skip(1)
            .try_for_each(|n| write!(f, ", {}", n.0))?;
        write!(f, ")")
    }
}

#[derive(Clone, Debug)]
pub enum BlockExit {
    Return(Vec<Identifier>),
    Jump(JumpInfo),
}

impl fmt::Display for BlockExit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockExit::Return(ids) => {
                write!(f, "return (")?;
                ids.iter().take(1).try_for_each(|n| write!(f, "{}", n.0))?;
                ids.iter()
                    .skip(1)
                    .try_for_each(|n| write!(f, ", {}", n.0))?;
                write!(f, ")")
            }
            BlockExit::Jump(j) => {
                write!(f, "{}", j)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct JumpInfo {
    pub ext: Extension,
    pub args: Vec<Identifier>,
    pub branches: Vec<BranchInfo>,
}

impl fmt::Display for JumpInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self.ext)?;
        self.args
            .iter()
            .take(1)
            .try_for_each(|n| write!(f, "{}", n.0))?;
        self.args
            .iter()
            .skip(1)
            .try_for_each(|n| write!(f, ", {}", n.0))?;
        writeln!(f, ") {{")?;
        self.branches
            .iter()
            .try_for_each(|b| writeln!(f, "{}", b))?;
        write!(f, "}}")
    }
}

#[derive(Clone, Debug)]
pub struct Extension {
    pub name: String,
    pub tmpl_args: Vec<TemplateArg>,
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        let mut iter = self.tmpl_args.iter();
        if let Some(ta) = iter.next() {
            write!(f, "<{}", ta)?;
            for ta in iter {
                write!(f, ", {}", ta)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct BranchInfo {
    pub target: BranchTarget,
    pub exports: Vec<Identifier>,
}

impl fmt::Display for BranchInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self.target)?;
        self.exports
            .iter()
            .take(1)
            .try_for_each(|n| write!(f, "{}", n.0))?;
        self.exports
            .iter()
            .skip(1)
            .try_for_each(|n| write!(f, ", {}", n.0))?;
        write!(f, ")")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BranchTarget {
    Fallthrough,
    Block(BlockId),
}

impl fmt::Display for BranchTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BranchTarget::Fallthrough => write!(f, "fallthrough"),
            BranchTarget::Block(b) => write!(f, "{}", b.0),
        }
    }
}
