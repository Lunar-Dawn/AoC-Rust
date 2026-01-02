#[macro_export]
macro_rules! impl_operators {
    ($op:tt $type1:ty; ($self:ident, $other:ident) { $($implementation:tt)* }) => {
        impl_operators!($op $type1, $type1; ($self, $other) { $($implementation)* });
    };

    (+ $type_left:ty, $type_right:ty; ($self:ident, $other:ident) { $($implementation:tt)* }) => {
        impl_operators!(+ += Add AddAssign add add_assign $type_left, $type_right; ($self, $other) { $($implementation)* });
    };
    (- $type_left:ty, $type_right:ty; ($self:ident, $other:ident) { $($implementation:tt)* }) => {
        impl_operators!(- -= Sub SubAssign sub sub_assign $type_left, $type_right; ($self, $other) { $($implementation)* });
    };
    (* $type_left:ty, $type_right:ty; ($self:ident, $other:ident) { $($implementation:tt)* }) => {
        impl_operators!(* *= Mul MulAssign mul mul_assign $type_left, $type_right; ($self, $other) { $($implementation)* });
    };
    (/ $type_left:ty, $type_right:ty; ($self:ident, $other:ident) { $($implementation:tt)* }) => {
        impl_operators!(/ /= Div DivAssign div div_assign $type_left, $type_right; ($self, $other) { $($implementation)* });
    };


    ($op:tt $opa:tt $OpTrait:ident $OpAssTrait:ident $OpFn:ident $OpAssFn:ident $lhs:ty, $rhs:ty; ($self:ident, $other:ident) { $($implementation:tt)* }) => {
        impl $OpAssTrait<$rhs> for $lhs {
            fn $OpAssFn(&mut $self, $other: $rhs) {
                $( $implementation )*
            }
        }
        impl $OpAssTrait<&$rhs> for $lhs {
            fn $OpAssFn(&mut $self, $other: &$rhs) {
                $( $implementation )*
            }
        }

        impl $OpTrait<$rhs> for $lhs {
            type Output = $lhs;
            fn $OpFn(mut $self, $other: $rhs) -> $lhs {
                $self $opa $other;
                $self
            }
        }
        impl $OpTrait<$rhs> for &$lhs {
            type Output = $lhs;
            fn $OpFn($self, $other: $rhs) -> $lhs {
                *$self $op $other
            }
        }
        impl $OpTrait<&$rhs> for $lhs {
            type Output = $lhs;
            fn $OpFn($self, $other: &$rhs) -> $lhs {
                $self $op *$other
            }
        }
        impl $OpTrait<&$rhs> for &$lhs {
            type Output = $lhs;
            fn $OpFn($self, $other: &$rhs) -> $lhs {
                *$self $op *$other
            }
        }
    };
}

#[macro_export]
macro_rules! impl_neg {
    ($ty:ty; ($self:ident) { $( $implementation:tt )* }) => {
        impl Neg for $ty {
            type Output = $ty;
            fn neg($self) -> $ty {
                $( $implementation )*
            }
        }
        impl Neg for &$ty {
            type Output = $ty;
            fn neg($self) -> $ty {
                -(*$self)
            }
        }
    }
}
