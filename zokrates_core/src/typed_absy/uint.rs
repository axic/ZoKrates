use typed_absy::types::{FunctionKey, UBitwidth};
use typed_absy::*;
use zokrates_field::Field;

type Bitwidth = usize;

impl<'ast, T: Field> UExpression<'ast, T> {
    pub fn add(self, other: Self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        assert_eq!(bitwidth, other.bitwidth);
        UExpressionInner::Add(box self, box other).annotate(bitwidth)
    }

    pub fn sub(self, other: Self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        assert_eq!(bitwidth, other.bitwidth);
        UExpressionInner::Sub(box self, box other).annotate(bitwidth)
    }

    pub fn mult(self, other: Self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        assert_eq!(bitwidth, other.bitwidth);
        UExpressionInner::Mult(box self, box other).annotate(bitwidth)
    }

    pub fn xor(self, other: Self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        assert_eq!(bitwidth, other.bitwidth);
        UExpressionInner::Xor(box self, box other).annotate(bitwidth)
    }

    pub fn or(self, other: Self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        assert_eq!(bitwidth, other.bitwidth);
        UExpressionInner::Or(box self, box other).annotate(bitwidth)
    }

    pub fn and(self, other: Self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        assert_eq!(bitwidth, other.bitwidth);
        UExpressionInner::And(box self, box other).annotate(bitwidth)
    }

    pub fn not(self) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        UExpressionInner::Not(box self).annotate(bitwidth)
    }

    pub fn left_shift(self, by: FieldElementExpression<'ast, T>) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        UExpressionInner::LeftShift(box self, box by).annotate(bitwidth)
    }

    pub fn right_shift(self, by: FieldElementExpression<'ast, T>) -> UExpression<'ast, T> {
        let bitwidth = self.bitwidth;
        UExpressionInner::RightShift(box self, box by).annotate(bitwidth)
    }

    pub fn try_from_typed(
        e: TypedExpression<'ast, T>,
        bitwidth: UBitwidth,
    ) -> Result<Self, TypedExpression<'ast, T>> {
        match e {
            TypedExpression::Uint(e) => match e.bitwidth == bitwidth {
                true => Ok(e),
                _ => Err(TypedExpression::Uint(e)),
            },
            TypedExpression::Int(e) => {
                Self::try_from_int(e.clone(), bitwidth).map_err(|_| TypedExpression::Int(e))
            }
            e => Err(e),
        }
    }

    pub fn try_from_int(
        i: IntExpression<'ast, T>,
        bitwidth: UBitwidth,
    ) -> Result<Self, IntExpression<'ast, T>> {
        use self::IntExpression::*;

        match i {
            Value(i) => {
                if i <= BigUint::from(2u128.pow(bitwidth.to_usize() as u32 - 1)) {
                    Ok(UExpressionInner::Value(
                        u128::from_str_radix(&i.to_str_radix(16), 16).unwrap(),
                    )
                    .annotate(bitwidth))
                } else {
                    Err(Value(i))
                }
            }
            Add(box e1, box e2) => Ok(UExpression::add(
                Self::try_from_int(e1, bitwidth)?,
                Self::try_from_int(e2, bitwidth)?,
            )),
            Sub(box e1, box e2) => Ok(UExpression::sub(
                Self::try_from_int(e1, bitwidth)?,
                Self::try_from_int(e2, bitwidth)?,
            )),
            Mult(box e1, box e2) => Ok(UExpression::mult(
                Self::try_from_int(e1, bitwidth)?,
                Self::try_from_int(e2, bitwidth)?,
            )),
            And(box e1, box e2) => Ok(UExpression::and(
                Self::try_from_int(e1, bitwidth)?,
                Self::try_from_int(e2, bitwidth)?,
            )),
            Or(box e1, box e2) => Ok(UExpression::or(
                Self::try_from_int(e1, bitwidth)?,
                Self::try_from_int(e2, bitwidth)?,
            )),
            Xor(box e1, box e2) => Ok(UExpression::xor(
                Self::try_from_int(e1, bitwidth)?,
                Self::try_from_int(e2, bitwidth)?,
            )),
            RightShift(box e1, box e2) => Ok(UExpression::right_shift(
                Self::try_from_int(e1, bitwidth)?,
                e2,
            )),
            LeftShift(box e1, box e2) => Ok(UExpression::left_shift(
                Self::try_from_int(e1, bitwidth)?,
                e2,
            )),
            IfElse(box condition, box consequence, box alternative) => Ok(UExpression::if_else(
                condition,
                Self::try_from_int(consequence, bitwidth)?,
                Self::try_from_int(alternative, bitwidth)?,
            )),
            Select(..) => unimplemented!(),
            i => Err(i),
        }
    }
}

impl<'ast, T: Field> From<u128> for UExpressionInner<'ast, T> {
    fn from(e: u128) -> Self {
        UExpressionInner::Value(e)
    }
}

impl<'ast, T: Field> From<&'ast str> for UExpressionInner<'ast, T> {
    fn from(e: &'ast str) -> Self {
        UExpressionInner::Identifier(e.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UMetadata {
    pub bitwidth: Option<Bitwidth>,
    pub should_reduce: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UExpression<'ast, T> {
    pub bitwidth: UBitwidth,
    pub metadata: Option<UMetadata>,
    pub inner: UExpressionInner<'ast, T>,
}

impl<'ast, T> From<u32> for UExpression<'ast, T> {
    fn from(u: u32) -> Self {
        UExpressionInner::Value(u as u128).annotate(UBitwidth::B32)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum UExpressionInner<'ast, T> {
    Identifier(Identifier<'ast>),
    Value(u128),
    Add(Box<UExpression<'ast, T>>, Box<UExpression<'ast, T>>),
    Sub(Box<UExpression<'ast, T>>, Box<UExpression<'ast, T>>),
    Mult(Box<UExpression<'ast, T>>, Box<UExpression<'ast, T>>),
    Xor(Box<UExpression<'ast, T>>, Box<UExpression<'ast, T>>),
    And(Box<UExpression<'ast, T>>, Box<UExpression<'ast, T>>),
    Or(Box<UExpression<'ast, T>>, Box<UExpression<'ast, T>>),
    Not(Box<UExpression<'ast, T>>),
    LeftShift(
        Box<UExpression<'ast, T>>,
        Box<FieldElementExpression<'ast, T>>,
    ),
    RightShift(
        Box<UExpression<'ast, T>>,
        Box<FieldElementExpression<'ast, T>>,
    ),
    FunctionCall(FunctionKey<'ast>, Vec<TypedExpression<'ast, T>>),
    IfElse(
        Box<BooleanExpression<'ast, T>>,
        Box<UExpression<'ast, T>>,
        Box<UExpression<'ast, T>>,
    ),
    Member(Box<StructExpression<'ast, T>>, MemberId),
    Select(
        Box<ArrayExpression<'ast, T>>,
        Box<FieldElementExpression<'ast, T>>,
    ),
}

impl<'ast, T: fmt::Display> fmt::Display for UExpression<'ast, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            UExpressionInner::Value(ref v) => write!(f, "{}u{}", v, self.bitwidth),
            UExpressionInner::Identifier(ref var) => write!(f, "{}", var),
            UExpressionInner::Add(ref lhs, ref rhs) => write!(f, "({} + {})", lhs, rhs),
            UExpressionInner::And(ref lhs, ref rhs) => write!(f, "({} & {})", lhs, rhs),
            UExpressionInner::Or(ref lhs, ref rhs) => write!(f, "({} | {})", lhs, rhs),
            UExpressionInner::Xor(ref lhs, ref rhs) => write!(f, "({} ^ {})", lhs, rhs),
            UExpressionInner::Sub(ref lhs, ref rhs) => write!(f, "({} - {})", lhs, rhs),
            UExpressionInner::Mult(ref lhs, ref rhs) => write!(f, "({} * {})", lhs, rhs),
            UExpressionInner::RightShift(ref e, ref by) => write!(f, "({} >> {})", e, by),
            UExpressionInner::LeftShift(ref e, ref by) => write!(f, "({} << {})", e, by),
            UExpressionInner::Not(ref e) => write!(f, "!{}", e),
            UExpressionInner::Select(ref id, ref index) => write!(f, "{}[{}]", id, index),
            UExpressionInner::FunctionCall(ref k, ref p) => {
                write!(f, "{}(", k.id,)?;
                for (i, param) in p.iter().enumerate() {
                    write!(f, "{}", param)?;
                    if i < p.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            UExpressionInner::IfElse(ref condition, ref consequent, ref alternative) => write!(
                f,
                "if {} then {} else {} fi",
                condition, consequent, alternative
            ),
            UExpressionInner::Member(ref struc, ref id) => write!(f, "{}.{}", struc, id),
        }
    }
}

impl<'ast, T: fmt::Debug> fmt::Debug for UExpressionInner<'ast, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UExpressionInner::Identifier(ref var) => write!(f, "Ide({})", var),
            UExpressionInner::Value(ref i) => write!(f, "Num({:?})", i),
            UExpressionInner::Add(ref lhs, ref rhs) => write!(f, "Add({:?}, {:?})", lhs, rhs),
            UExpressionInner::Sub(ref lhs, ref rhs) => write!(f, "Sub({:?}, {:?})", lhs, rhs),
            UExpressionInner::Mult(ref lhs, ref rhs) => write!(f, "Mult({:?}, {:?})", lhs, rhs),
            UExpressionInner::IfElse(ref condition, ref consequent, ref alternative) => write!(
                f,
                "IfElse({:?}, {:?}, {:?})",
                condition, consequent, alternative
            ),
            UExpressionInner::Select(ref id, ref index) => {
                write!(f, "Select({:?}, {:?})", id, index)
            }
            UExpressionInner::And(ref lhs, ref rhs) => write!(f, "And({:?}, {:?})", lhs, rhs),
            UExpressionInner::Or(ref lhs, ref rhs) => write!(f, "Or({:?}, {:?})", lhs, rhs),
            UExpressionInner::Xor(ref lhs, ref rhs) => write!(f, "Xor({:?}, {:?})", lhs, rhs),
            UExpressionInner::RightShift(ref e, ref by) => {
                write!(f, "RightShift({:?}, {:?})", e, by)
            }
            UExpressionInner::LeftShift(ref e, ref by) => write!(f, "LeftShift({:?}, {:?})", e, by),
            UExpressionInner::Not(ref e) => write!(f, "Not({:?})", e),
            UExpressionInner::FunctionCall(ref i, ref p) => {
                write!(f, "FunctionCall({:?}, (", i)?;
                f.debug_list().entries(p.iter()).finish()?;
                write!(f, ")")
            }
            UExpressionInner::Member(ref struc, ref id) => {
                write!(f, "Access({:?}, {:?})", struc, id)
            }
        }
    }
}

impl<'ast, T> UExpressionInner<'ast, T> {
    pub fn annotate<W: Into<UBitwidth>>(self, bitwidth: W) -> UExpression<'ast, T> {
        UExpression {
            metadata: None,
            bitwidth: bitwidth.into(),
            inner: self,
        }
    }
}

impl<'ast, T> UExpression<'ast, T> {
    pub fn metadata(self, metadata: UMetadata) -> UExpression<'ast, T> {
        UExpression {
            metadata: Some(metadata),
            ..self
        }
    }
}

pub fn bitwidth(a: u128) -> Bitwidth {
    (128 - a.leading_zeros()) as Bitwidth
}

impl<'ast, T> UExpression<'ast, T> {
    pub fn bitwidth(&self) -> UBitwidth {
        self.bitwidth
    }

    pub fn as_inner(&self) -> &UExpressionInner<'ast, T> {
        &self.inner
    }

    pub fn into_inner(self) -> UExpressionInner<'ast, T> {
        self.inner
    }
}
