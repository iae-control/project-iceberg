        // === AUTO-GENERATED Level 1 candidate operators ===
        // Total: 2011 operators
        (
            "Plus_Exp_Exp",
            Binary {
                f: |a, b| { Some((a).exp().add((b).exp())) },
                commutative: true,
            },
        ),
        (
            "Plus_Exp_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).exp().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Sin",
            Binary {
                f: |a, b| { Some((a).exp().add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).exp().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Sinh",
            Binary {
                f: |a, b| { Some((a).exp().add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).exp().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).exp().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Minus",
            Binary {
                f: |a, b| { Some((a).exp().add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Sqr",
            Binary {
                f: |a, b| { Some((a).exp().add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Sqrt",
            Binary {
                f: |a, b| { Some((a).exp().add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_Id",
            Binary {
                f: |a, b| { Some((a).exp().add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_Exp_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).exp().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Log",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_Log_Sin",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Tan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Sinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Tanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Minus",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Sqr",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_Sqrt",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Log_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sin_Sin",
            Binary {
                f: |a, b| { Some((a).sin().add((b).sin())) },
                commutative: true,
            },
        ),
        (
            "Plus_Sin_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sin().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sin_Sinh",
            Binary {
                f: |a, b| { Some((a).sin().add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Sin_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sin().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sin_Sqr",
            Binary {
                f: |a, b| { Some((a).sin().add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Sin_Sqrt",
            Binary {
                f: |a, b| { Some((a).sin().add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Exp",
            Binary {
                f: |a, b| { Some((a).cos().add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).cos().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Sin",
            Binary {
                f: |a, b| { Some((a).cos().add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Cos",
            Binary {
                f: |a, b| { Some((a).cos().add((b).cos())) },
                commutative: true,
            },
        ),
        (
            "Plus_Cos_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).cos().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Sinh",
            Binary {
                f: |a, b| { Some((a).cos().add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Cosh",
            Binary {
                f: |a, b| { Some((a).cos().add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).cos().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).cos().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Minus",
            Binary {
                f: |a, b| { Some((a).cos().add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Sqr",
            Binary {
                f: |a, b| { Some((a).cos().add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Sqrt",
            Binary {
                f: |a, b| { Some((a).cos().add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_Id",
            Binary {
                f: |a, b| { Some((a).cos().add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cos_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).cos().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Tan_Tan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_Tan_Tanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Exp",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Log",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Sin",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Cos",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Tan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asin()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcSin_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Sinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Cosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Tanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asinh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Inv",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Minus",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Sqr",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_Id",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSin_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Exp",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Log",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Sin",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Cos",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Tan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asin()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acos()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcCos_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Sinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Cosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Tanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asinh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acosh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Inv",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Minus",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Sqr",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_Id",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCos_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Exp",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Log",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Sin",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Cos",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Tan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atan()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcTan_Sinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Cosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Tanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Inv",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Minus",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Sqr",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_Id",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sinh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sinh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sinh_Sinh",
            Binary {
                f: |a, b| { Some((a).sinh().add((b).sinh())) },
                commutative: true,
            },
        ),
        (
            "Plus_Sinh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sinh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sinh_Sqr",
            Binary {
                f: |a, b| { Some((a).sinh().add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Sinh_Sqrt",
            Binary {
                f: |a, b| { Some((a).sinh().add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Exp",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).cosh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Sin",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).cosh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Sinh",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Cosh",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).cosh())) },
                commutative: true,
            },
        ),
        (
            "Plus_Cosh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).cosh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).cosh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Minus",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Sqr",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Sqrt",
            Binary {
                f: |a, b| { Some((a).cosh().add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_Id",
            Binary {
                f: |a, b| { Some((a).cosh().add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_Cosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).cosh().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Tanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcSinh_Exp",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Log",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Sin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Cos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Tan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Sinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Cosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Tanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asinh()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcSinh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Inv",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Minus",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Sqr",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_Id",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcSinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Exp",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Log",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Sin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Cos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Tan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asin()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Sinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Cosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Tanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asinh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acosh()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcCosh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Inv",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Minus",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Sqr",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_Id",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcCosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Exp",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Log",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Sin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Cos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Tan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atanh()?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_ArcTanh_Inv",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Minus",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_Id",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.add(b)) },
                commutative: false,
            },
        ),
        (
            "Plus_ArcTanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Log",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).ln()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Sin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Tan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Sinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Tanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Inv",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = C::real(1.0).div(b)?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Plus_Inv_Minus",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Sqr",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_Sqrt",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Inv_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Minus_Sin",
            Binary {
                f: |a, b| { Some((a).neg().add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Minus_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).neg().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Minus_Sinh",
            Binary {
                f: |a, b| { Some((a).neg().add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Minus_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).neg().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Minus_Minus",
            Binary {
                f: |a, b| { Some((a).neg().add((b).neg())) },
                commutative: true,
            },
        ),
        (
            "Plus_Minus_Sqr",
            Binary {
                f: |a, b| { Some((a).neg().add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Minus_Sqrt",
            Binary {
                f: |a, b| { Some((a).neg().add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Sqr_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).mul(a).add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sqr_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).mul(a).add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sqr_Sqr",
            Binary {
                f: |a, b| { Some((a).mul(a).add((b).mul(b))) },
                commutative: true,
            },
        ),
        (
            "Plus_Sqr_Sqrt",
            Binary {
                f: |a, b| { Some((a).mul(a).add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Sqrt_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sqrt().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sqrt_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sqrt().add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Sqrt_Sqrt",
            Binary {
                f: |a, b| { Some((a).sqrt().add((b).sqrt())) },
                commutative: true,
            },
        ),
        (
            "Plus_Id_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some(a.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Sin",
            Binary {
                f: |a, b| { Some(a.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some(a.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Sinh",
            Binary {
                f: |a, b| { Some(a.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some(a.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some(a.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Minus",
            Binary {
                f: |a, b| { Some(a.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Sqr",
            Binary {
                f: |a, b| { Some(a.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_Sqrt",
            Binary {
                f: |a, b| { Some(a.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_Id_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some(a.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Sin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.add((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Tan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tan()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Sinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.add((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Tanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tanh()?; Some(left.add(right)) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Minus",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.add((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Sqr",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.add((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_Sqrt",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.add((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Plus_LogisticSigmoid_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = logistic_sigmoid(b)?; Some(left.add(right)) },
                commutative: true,
            },
        ),
        (
            "Subtract_Exp_Exp",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Sin",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Cos",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Sinh",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Cosh",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Minus",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Sqr",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Sqrt",
            Binary {
                f: |a, b| { Some((a).exp().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_Id",
            Binary {
                f: |a, b| { Some((a).exp().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Exp_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).exp().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Log",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Sin",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Cos",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Tan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_ArcSin",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_ArcCos",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_ArcTan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Sinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Cosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Tanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Inv",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Minus",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Sqr",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Sqrt",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_Id",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Log_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Exp",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Sin",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Cos",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Sinh",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Cosh",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Minus",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Sqr",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Sqrt",
            Binary {
                f: |a, b| { Some((a).sin().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_Id",
            Binary {
                f: |a, b| { Some((a).sin().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sin_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).sin().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Exp",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Sin",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Cos",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Sinh",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Cosh",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Minus",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Sqr",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Sqrt",
            Binary {
                f: |a, b| { Some((a).cos().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_Id",
            Binary {
                f: |a, b| { Some((a).cos().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cos_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).cos().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Exp",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Log",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Sin",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Cos",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Tan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Sinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Cosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Tanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Inv",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Minus",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Sqr",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_Id",
            Binary {
                f: |a, b| { let left = (a).tan()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Exp",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Log",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Sin",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Cos",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Tan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Sinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Cosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Tanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Inv",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Minus",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Sqr",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_Id",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSin_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Exp",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Log",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Sin",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Cos",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Tan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Sinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Cosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Tanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Inv",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Minus",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Sqr",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_Id",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCos_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Exp",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Log",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Sin",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Cos",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Tan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Sinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Cosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Tanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Inv",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Minus",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Sqr",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_Id",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Exp",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Sin",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Cos",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Sinh",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Cosh",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Minus",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Sqr",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Sqrt",
            Binary {
                f: |a, b| { Some((a).sinh().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_Id",
            Binary {
                f: |a, b| { Some((a).sinh().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).sinh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Exp",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Sin",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Cos",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Sinh",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Cosh",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Minus",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Sqr",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Sqrt",
            Binary {
                f: |a, b| { Some((a).cosh().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_Id",
            Binary {
                f: |a, b| { Some((a).cosh().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Cosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).cosh().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Exp",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Log",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Sin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Cos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Tan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Inv",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Minus",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_Id",
            Binary {
                f: |a, b| { let left = (a).tanh()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Tanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Exp",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Log",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Sin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Cos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Tan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Sinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Cosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Tanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Inv",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Minus",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Sqr",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_Id",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcSinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Exp",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Log",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Sin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Cos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Tan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Sinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Cosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Tanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Inv",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Minus",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Sqr",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_Id",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcCosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Exp",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Log",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Sin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Cos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Tan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Inv",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Minus",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_Id",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_ArcTanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Exp",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Log",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Sin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Cos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Tan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_ArcSin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_ArcCos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_ArcTan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Sinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Cosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Tanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_ArcSinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_ArcCosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_ArcTanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Inv",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Minus",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Sqr",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Sqrt",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_Id",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Inv_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Exp",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Sin",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Cos",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Sinh",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Cosh",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Minus",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Sqr",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Sqrt",
            Binary {
                f: |a, b| { Some((a).neg().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_Id",
            Binary {
                f: |a, b| { Some((a).neg().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Minus_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).neg().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Exp",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Sin",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Cos",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Sinh",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Cosh",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Minus",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Sqr",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Sqrt",
            Binary {
                f: |a, b| { Some((a).mul(a).sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_Id",
            Binary {
                f: |a, b| { Some((a).mul(a).sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqr_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).mul(a).sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Exp",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Sin",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Cos",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Sinh",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Cosh",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Minus",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Sqr",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Sqrt",
            Binary {
                f: |a, b| { Some((a).sqrt().sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_Id",
            Binary {
                f: |a, b| { Some((a).sqrt().sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Sqrt_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).sqrt().sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Exp",
            Binary {
                f: |a, b| { Some(a.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Sin",
            Binary {
                f: |a, b| { Some(a.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Cos",
            Binary {
                f: |a, b| { Some(a.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Sinh",
            Binary {
                f: |a, b| { Some(a.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Cosh",
            Binary {
                f: |a, b| { Some(a.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Minus",
            Binary {
                f: |a, b| { Some(a.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Sqr",
            Binary {
                f: |a, b| { Some(a.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_Sqrt",
            Binary {
                f: |a, b| { Some(a.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_Id_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some(a.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Exp",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Log",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).ln()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Sin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Cos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Tan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_ArcSin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asin()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_ArcCos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acos()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_ArcTan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atan()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Sinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Cosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Tanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_ArcSinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asinh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_ArcCosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acosh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_ArcTanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atanh()?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Inv",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = C::real(1.0).div(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Minus",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Sqr",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Sqrt",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_Id",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.sub(b)) },
                commutative: false,
            },
        ),
        (
            "Subtract_LogisticSigmoid_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = logistic_sigmoid(b)?; Some(left.sub(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Exp",
            Binary {
                f: |a, b| { Some((a).exp().mul((b).exp())) },
                commutative: true,
            },
        ),
        (
            "Times_Exp_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).exp().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Sin",
            Binary {
                f: |a, b| { Some((a).exp().mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).exp().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Sinh",
            Binary {
                f: |a, b| { Some((a).exp().mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).exp().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).exp().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Minus",
            Binary {
                f: |a, b| { Some((a).exp().mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Sqr",
            Binary {
                f: |a, b| { Some((a).exp().mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Sqrt",
            Binary {
                f: |a, b| { Some((a).exp().mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_Id",
            Binary {
                f: |a, b| { Some((a).exp().mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_Exp_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).exp().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Log",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_Log_Sin",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Tan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Sinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Tanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Minus",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Sqr",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Log_Sqrt",
            Binary {
                f: |a, b| { let left = (a).ln()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Log_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sin_Sin",
            Binary {
                f: |a, b| { Some((a).sin().mul((b).sin())) },
                commutative: true,
            },
        ),
        (
            "Times_Sin_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sin().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sin_Sinh",
            Binary {
                f: |a, b| { Some((a).sin().mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Sin_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sin().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sin_Sqr",
            Binary {
                f: |a, b| { Some((a).sin().mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Sin_Sqrt",
            Binary {
                f: |a, b| { Some((a).sin().mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Exp",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).cos().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Sin",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Cos",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).cos())) },
                commutative: true,
            },
        ),
        (
            "Times_Cos_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).cos().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Sinh",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Cosh",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).cos().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).cos().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Minus",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Sqr",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Sqrt",
            Binary {
                f: |a, b| { Some((a).cos().mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_Id",
            Binary {
                f: |a, b| { Some((a).cos().mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_Cos_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).cos().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Tan_Tan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_Tan_Tanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Exp",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Log",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Sin",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Cos",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Tan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asin()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcSin_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Sinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Cosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Tanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asinh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Inv",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Minus",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Sqr",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_Id",
            Binary {
                f: |a, b| { let left = (a).asin()?; Some(left.mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSin_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Exp",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Log",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Sin",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Cos",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Tan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asin()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acos()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcCos_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Sinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Cosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Tanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asinh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acosh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Inv",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Minus",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Sqr",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_Id",
            Binary {
                f: |a, b| { let left = (a).acos()?; Some(left.mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCos_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Exp",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Log",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Sin",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Cos",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Tan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atan()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcTan_Sinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Cosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Tanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Inv",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Minus",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Sqr",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_Id",
            Binary {
                f: |a, b| { let left = (a).atan()?; Some(left.mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sinh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sinh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sinh_Sinh",
            Binary {
                f: |a, b| { Some((a).sinh().mul((b).sinh())) },
                commutative: true,
            },
        ),
        (
            "Times_Sinh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sinh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sinh_Sqr",
            Binary {
                f: |a, b| { Some((a).sinh().mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Sinh_Sqrt",
            Binary {
                f: |a, b| { Some((a).sinh().mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Exp",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some((a).cosh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Sin",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).cosh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Sinh",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Cosh",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).cosh())) },
                commutative: true,
            },
        ),
        (
            "Times_Cosh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).cosh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some((a).cosh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Minus",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Sqr",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Sqrt",
            Binary {
                f: |a, b| { Some((a).cosh().mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_Id",
            Binary {
                f: |a, b| { Some((a).cosh().mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_Cosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some((a).cosh().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Tanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcSinh_Exp",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Log",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Sin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Cos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Tan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Sinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Cosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Tanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asinh()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcSinh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Inv",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Minus",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Sqr",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_Id",
            Binary {
                f: |a, b| { let left = (a).asinh()?; Some(left.mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcSinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Exp",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Log",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Sin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Cos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Tan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asin()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Sinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Cosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Tanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asinh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acosh()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcCosh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Inv",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Minus",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Sqr",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_Id",
            Binary {
                f: |a, b| { let left = (a).acosh()?; Some(left.mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcCosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Exp",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).exp())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Log",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Sin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Cos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).cos())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Tan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).cosh())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atanh()?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_ArcTanh_Inv",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Minus",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_Id",
            Binary {
                f: |a, b| { let left = (a).atanh()?; Some(left.mul(b)) },
                commutative: false,
            },
        ),
        (
            "Times_ArcTanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Log",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).ln()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Sin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Tan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Sinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Tanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Inv",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = C::real(1.0).div(b)?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Times_Inv_Minus",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Sqr",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_Sqrt",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Inv_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Minus_Sin",
            Binary {
                f: |a, b| { Some((a).neg().mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Minus_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).neg().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Minus_Sinh",
            Binary {
                f: |a, b| { Some((a).neg().mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Minus_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).neg().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Minus_Minus",
            Binary {
                f: |a, b| { Some((a).neg().mul((b).neg())) },
                commutative: true,
            },
        ),
        (
            "Times_Minus_Sqr",
            Binary {
                f: |a, b| { Some((a).neg().mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Minus_Sqrt",
            Binary {
                f: |a, b| { Some((a).neg().mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Sqr_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).mul(a).mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sqr_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).mul(a).mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sqr_Sqr",
            Binary {
                f: |a, b| { Some((a).mul(a).mul((b).mul(b))) },
                commutative: true,
            },
        ),
        (
            "Times_Sqr_Sqrt",
            Binary {
                f: |a, b| { Some((a).mul(a).mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Sqrt_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some((a).sqrt().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sqrt_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some((a).sqrt().mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Sqrt_Sqrt",
            Binary {
                f: |a, b| { Some((a).sqrt().mul((b).sqrt())) },
                commutative: true,
            },
        ),
        (
            "Times_Id_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; Some(a.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Sin",
            Binary {
                f: |a, b| { Some(a.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; Some(a.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Sinh",
            Binary {
                f: |a, b| { Some(a.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; Some(a.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; Some(a.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Minus",
            Binary {
                f: |a, b| { Some(a.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Sqr",
            Binary {
                f: |a, b| { Some(a.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_Id_Sqrt",
            Binary {
                f: |a, b| { Some(a.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_Id_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; Some(a.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Sin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.mul((b).sin())) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Tan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tan()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Sinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.mul((b).sinh())) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Tanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tanh()?; Some(left.mul(right)) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Minus",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.mul((b).neg())) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Sqr",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.mul((b).mul(b))) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_Sqrt",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; Some(left.mul((b).sqrt())) },
                commutative: false,
            },
        ),
        (
            "Times_LogisticSigmoid_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = logistic_sigmoid(b)?; Some(left.mul(right)) },
                commutative: true,
            },
        ),
        (
            "Divide_Exp_Exp",
            Binary {
                f: |a, b| { (a).exp().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Sin",
            Binary {
                f: |a, b| { (a).exp().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Cos",
            Binary {
                f: |a, b| { (a).exp().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Sinh",
            Binary {
                f: |a, b| { (a).exp().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Cosh",
            Binary {
                f: |a, b| { (a).exp().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Minus",
            Binary {
                f: |a, b| { (a).exp().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Sqr",
            Binary {
                f: |a, b| { (a).exp().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Sqrt",
            Binary {
                f: |a, b| { (a).exp().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_Id",
            Binary {
                f: |a, b| { (a).exp().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Exp_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).exp().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Exp",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Log",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Sin",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Cos",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Tan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_ArcSin",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_ArcCos",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_ArcTan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Sinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Cosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Tanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Inv",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Minus",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Sqr",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Sqrt",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_Id",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Log_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Exp",
            Binary {
                f: |a, b| { (a).sin().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Sin",
            Binary {
                f: |a, b| { (a).sin().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Cos",
            Binary {
                f: |a, b| { (a).sin().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Sinh",
            Binary {
                f: |a, b| { (a).sin().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Cosh",
            Binary {
                f: |a, b| { (a).sin().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Minus",
            Binary {
                f: |a, b| { (a).sin().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Sqr",
            Binary {
                f: |a, b| { (a).sin().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Sqrt",
            Binary {
                f: |a, b| { (a).sin().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_Id",
            Binary {
                f: |a, b| { (a).sin().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Sin_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).sin().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Exp",
            Binary {
                f: |a, b| { (a).cos().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Sin",
            Binary {
                f: |a, b| { (a).cos().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Cos",
            Binary {
                f: |a, b| { (a).cos().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Sinh",
            Binary {
                f: |a, b| { (a).cos().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Cosh",
            Binary {
                f: |a, b| { (a).cos().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Minus",
            Binary {
                f: |a, b| { (a).cos().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Sqr",
            Binary {
                f: |a, b| { (a).cos().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Sqrt",
            Binary {
                f: |a, b| { (a).cos().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_Id",
            Binary {
                f: |a, b| { (a).cos().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Cos_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).cos().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Exp",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Log",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Sin",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Cos",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Tan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Sinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Cosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Tanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Inv",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Minus",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Sqr",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_Id",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Tan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Exp",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Log",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Sin",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Cos",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Tan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Sinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Cosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Tanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Inv",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Minus",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Sqr",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_Id",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSin_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Exp",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Log",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Sin",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Cos",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Tan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Sinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Cosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Tanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Inv",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Minus",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Sqr",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_Id",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCos_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Exp",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Log",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Sin",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Cos",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Tan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Sinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Cosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Tanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Inv",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Minus",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Sqr",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_Id",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Exp",
            Binary {
                f: |a, b| { (a).sinh().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Sin",
            Binary {
                f: |a, b| { (a).sinh().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Cos",
            Binary {
                f: |a, b| { (a).sinh().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Sinh",
            Binary {
                f: |a, b| { (a).sinh().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Cosh",
            Binary {
                f: |a, b| { (a).sinh().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Minus",
            Binary {
                f: |a, b| { (a).sinh().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Sqr",
            Binary {
                f: |a, b| { (a).sinh().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Sqrt",
            Binary {
                f: |a, b| { (a).sinh().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_Id",
            Binary {
                f: |a, b| { (a).sinh().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Sinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).sinh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Exp",
            Binary {
                f: |a, b| { (a).cosh().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Sin",
            Binary {
                f: |a, b| { (a).cosh().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Cos",
            Binary {
                f: |a, b| { (a).cosh().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Sinh",
            Binary {
                f: |a, b| { (a).cosh().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Cosh",
            Binary {
                f: |a, b| { (a).cosh().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Minus",
            Binary {
                f: |a, b| { (a).cosh().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Sqr",
            Binary {
                f: |a, b| { (a).cosh().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Sqrt",
            Binary {
                f: |a, b| { (a).cosh().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_Id",
            Binary {
                f: |a, b| { (a).cosh().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Cosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).cosh().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Exp",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Log",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Sin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Cos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Tan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Inv",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Minus",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_Id",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Tanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Exp",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Log",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Sin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Cos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Tan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Sinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Cosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Tanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Inv",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Minus",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Sqr",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_Id",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcSinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Exp",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Log",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Sin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Cos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Tan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Sinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Cosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Tanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Inv",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Minus",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Sqr",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_Id",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcCosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Exp",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Log",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Sin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Cos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Tan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Inv",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Minus",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_Id",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_ArcTanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Exp",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Log",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Sin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Cos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Tan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_ArcSin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_ArcCos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_ArcTan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Sinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Cosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Tanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_ArcSinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_ArcCosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_ArcTanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Inv",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Minus",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Sqr",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Sqrt",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_Id",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Inv_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Exp",
            Binary {
                f: |a, b| { (a).neg().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Sin",
            Binary {
                f: |a, b| { (a).neg().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Cos",
            Binary {
                f: |a, b| { (a).neg().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Sinh",
            Binary {
                f: |a, b| { (a).neg().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Cosh",
            Binary {
                f: |a, b| { (a).neg().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Minus",
            Binary {
                f: |a, b| { (a).neg().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Sqr",
            Binary {
                f: |a, b| { (a).neg().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Sqrt",
            Binary {
                f: |a, b| { (a).neg().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_Id",
            Binary {
                f: |a, b| { (a).neg().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Minus_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).neg().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Exp",
            Binary {
                f: |a, b| { (a).mul(a).div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Sin",
            Binary {
                f: |a, b| { (a).mul(a).div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Cos",
            Binary {
                f: |a, b| { (a).mul(a).div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Sinh",
            Binary {
                f: |a, b| { (a).mul(a).div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Cosh",
            Binary {
                f: |a, b| { (a).mul(a).div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Minus",
            Binary {
                f: |a, b| { (a).mul(a).div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Sqr",
            Binary {
                f: |a, b| { (a).mul(a).div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Sqrt",
            Binary {
                f: |a, b| { (a).mul(a).div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_Id",
            Binary {
                f: |a, b| { (a).mul(a).div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqr_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).mul(a).div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Exp",
            Binary {
                f: |a, b| { (a).sqrt().div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Sin",
            Binary {
                f: |a, b| { (a).sqrt().div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Cos",
            Binary {
                f: |a, b| { (a).sqrt().div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Sinh",
            Binary {
                f: |a, b| { (a).sqrt().div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Cosh",
            Binary {
                f: |a, b| { (a).sqrt().div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Minus",
            Binary {
                f: |a, b| { (a).sqrt().div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Sqr",
            Binary {
                f: |a, b| { (a).sqrt().div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Sqrt",
            Binary {
                f: |a, b| { (a).sqrt().div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_Id",
            Binary {
                f: |a, b| { (a).sqrt().div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_Sqrt_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).sqrt().div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Exp",
            Binary {
                f: |a, b| { a.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Sin",
            Binary {
                f: |a, b| { a.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Cos",
            Binary {
                f: |a, b| { a.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Sinh",
            Binary {
                f: |a, b| { a.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Cosh",
            Binary {
                f: |a, b| { a.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Minus",
            Binary {
                f: |a, b| { a.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Sqr",
            Binary {
                f: |a, b| { a.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_Sqrt",
            Binary {
                f: |a, b| { a.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_Id_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; a.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Exp",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Log",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).ln()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Sin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Cos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Tan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_ArcSin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asin()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_ArcCos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acos()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_ArcTan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atan()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Sinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Cosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Tanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_ArcSinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asinh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_ArcCosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acosh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_ArcTanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atanh()?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Inv",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = C::real(1.0).div(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Minus",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Sqr",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Sqrt",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_Id",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.div(b) },
                commutative: false,
            },
        ),
        (
            "Divide_LogisticSigmoid_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = logistic_sigmoid(b)?; left.div(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Exp",
            Binary {
                f: |a, b| { (a).exp().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Sin",
            Binary {
                f: |a, b| { (a).exp().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Cos",
            Binary {
                f: |a, b| { (a).exp().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Sinh",
            Binary {
                f: |a, b| { (a).exp().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Cosh",
            Binary {
                f: |a, b| { (a).exp().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Minus",
            Binary {
                f: |a, b| { (a).exp().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Sqr",
            Binary {
                f: |a, b| { (a).exp().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Sqrt",
            Binary {
                f: |a, b| { (a).exp().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_Id",
            Binary {
                f: |a, b| { (a).exp().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Exp_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).exp().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Exp",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Log",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Sin",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Cos",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Tan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_ArcSin",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_ArcCos",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_ArcTan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Sinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Cosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Tanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Inv",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Minus",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Sqr",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Sqrt",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Log_Id",
            Binary {
                f: |a, b| { let left = (a).ln()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Log_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Exp",
            Binary {
                f: |a, b| { (a).sin().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Sin",
            Binary {
                f: |a, b| { (a).sin().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Cos",
            Binary {
                f: |a, b| { (a).sin().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Sinh",
            Binary {
                f: |a, b| { (a).sin().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Cosh",
            Binary {
                f: |a, b| { (a).sin().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Minus",
            Binary {
                f: |a, b| { (a).sin().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Sqr",
            Binary {
                f: |a, b| { (a).sin().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Sqrt",
            Binary {
                f: |a, b| { (a).sin().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_Id",
            Binary {
                f: |a, b| { (a).sin().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Sin_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).sin().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Exp",
            Binary {
                f: |a, b| { (a).cos().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Sin",
            Binary {
                f: |a, b| { (a).cos().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Cos",
            Binary {
                f: |a, b| { (a).cos().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Sinh",
            Binary {
                f: |a, b| { (a).cos().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Cosh",
            Binary {
                f: |a, b| { (a).cos().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Minus",
            Binary {
                f: |a, b| { (a).cos().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Sqr",
            Binary {
                f: |a, b| { (a).cos().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Sqrt",
            Binary {
                f: |a, b| { (a).cos().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_Id",
            Binary {
                f: |a, b| { (a).cos().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Cos_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).cos().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Exp",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Log",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Sin",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Cos",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Tan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Sinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Cosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Tanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Inv",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Minus",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Sqr",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_Id",
            Binary {
                f: |a, b| { let left = (a).tan()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Tan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Exp",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Log",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Sin",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Cos",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Tan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Sinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Cosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Tanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Inv",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Minus",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Sqr",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_Id",
            Binary {
                f: |a, b| { let left = (a).asin()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSin_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Exp",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Log",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Sin",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Cos",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Tan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Sinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Cosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Tanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Inv",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Minus",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Sqr",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_Id",
            Binary {
                f: |a, b| { let left = (a).acos()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCos_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Exp",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Log",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Sin",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Cos",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Tan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Sinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Cosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Tanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Inv",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Minus",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Sqr",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_Id",
            Binary {
                f: |a, b| { let left = (a).atan()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Exp",
            Binary {
                f: |a, b| { (a).sinh().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Sin",
            Binary {
                f: |a, b| { (a).sinh().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Cos",
            Binary {
                f: |a, b| { (a).sinh().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Sinh",
            Binary {
                f: |a, b| { (a).sinh().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Cosh",
            Binary {
                f: |a, b| { (a).sinh().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Minus",
            Binary {
                f: |a, b| { (a).sinh().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Sqr",
            Binary {
                f: |a, b| { (a).sinh().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Sqrt",
            Binary {
                f: |a, b| { (a).sinh().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_Id",
            Binary {
                f: |a, b| { (a).sinh().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Sinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).sinh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Exp",
            Binary {
                f: |a, b| { (a).cosh().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Sin",
            Binary {
                f: |a, b| { (a).cosh().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Cos",
            Binary {
                f: |a, b| { (a).cosh().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Sinh",
            Binary {
                f: |a, b| { (a).cosh().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Cosh",
            Binary {
                f: |a, b| { (a).cosh().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Minus",
            Binary {
                f: |a, b| { (a).cosh().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Sqr",
            Binary {
                f: |a, b| { (a).cosh().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Sqrt",
            Binary {
                f: |a, b| { (a).cosh().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_Id",
            Binary {
                f: |a, b| { (a).cosh().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Cosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).cosh().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Exp",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Log",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Sin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Cos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Tan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Inv",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Minus",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_Id",
            Binary {
                f: |a, b| { let left = (a).tanh()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Tanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Exp",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Log",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Sin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Cos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Tan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Sinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Cosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Tanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Inv",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Minus",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Sqr",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_Id",
            Binary {
                f: |a, b| { let left = (a).asinh()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_ArcSinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Exp",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Log",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Sin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Cos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Tan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Sinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Cosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Tanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Inv",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Minus",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Sqr",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_Id",
            Binary {
                f: |a, b| { let left = (a).acosh()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_ArcCosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Exp",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Log",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Sin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Cos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Tan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Inv",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Minus",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_Id",
            Binary {
                f: |a, b| { let left = (a).atanh()?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_ArcTanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Exp",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Log",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Sin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Cos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Tan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_ArcSin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_ArcCos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_ArcTan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Sinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Cosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Tanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_ArcSinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_ArcCosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_ArcTanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Inv",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Minus",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Sqr",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Sqrt",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_Id",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Inv_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Exp",
            Binary {
                f: |a, b| { (a).neg().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Sin",
            Binary {
                f: |a, b| { (a).neg().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Cos",
            Binary {
                f: |a, b| { (a).neg().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Sinh",
            Binary {
                f: |a, b| { (a).neg().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Cosh",
            Binary {
                f: |a, b| { (a).neg().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Minus",
            Binary {
                f: |a, b| { (a).neg().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Sqr",
            Binary {
                f: |a, b| { (a).neg().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Sqrt",
            Binary {
                f: |a, b| { (a).neg().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_Id",
            Binary {
                f: |a, b| { (a).neg().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Minus_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).neg().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Exp",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Sin",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Cos",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Sinh",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Cosh",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Minus",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Sqr",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Sqrt",
            Binary {
                f: |a, b| { (a).mul(a).pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_Id",
            Binary {
                f: |a, b| { (a).mul(a).pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Sqr_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).mul(a).pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Exp",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Sin",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Cos",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Sinh",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Cosh",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Minus",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Sqr",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Sqrt",
            Binary {
                f: |a, b| { (a).sqrt().pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_Id",
            Binary {
                f: |a, b| { (a).sqrt().pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_Sqrt_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; (a).sqrt().pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Exp",
            Binary {
                f: |a, b| { a.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Sin",
            Binary {
                f: |a, b| { a.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Cos",
            Binary {
                f: |a, b| { a.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Sinh",
            Binary {
                f: |a, b| { a.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Cosh",
            Binary {
                f: |a, b| { a.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Minus",
            Binary {
                f: |a, b| { a.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Sqr",
            Binary {
                f: |a, b| { a.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_Id_Sqrt",
            Binary {
                f: |a, b| { a.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_Id_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; a.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Exp",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).exp()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Log",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).ln()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Sin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).sin()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Cos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).cos()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Tan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_ArcSin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asin()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_ArcCos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acos()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_ArcTan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atan()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Sinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).sinh()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Cosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).cosh()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Tanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_ArcSinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asinh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_ArcCosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acosh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_ArcTanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atanh()?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Inv",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = C::real(1.0).div(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Minus",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).neg()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Sqr",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).mul(b)) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Sqrt",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow((b).sqrt()) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_Id",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; left.pow(b) },
                commutative: false,
            },
        ),
        (
            "Power_LogisticSigmoid_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = logistic_sigmoid(b)?; left.pow(right) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Exp_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).exp().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Exp",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Log",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Sin",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Cos",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Tan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_ArcSin",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_ArcCos",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_ArcTan",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Sinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Cosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Tanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Inv",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Minus",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Sqr",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Sqrt",
            Binary {
                f: |a, b| { let left = (a).ln()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_Id",
            Binary {
                f: |a, b| { let left = (a).ln()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Log_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).ln()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sin_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).sin().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cos_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).cos().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Exp",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Log",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Sin",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Cos",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Tan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Sinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Cosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Tanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Inv",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Minus",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Sqr",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tan()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_Id",
            Binary {
                f: |a, b| { let left = (a).tan()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tan()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Exp",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Log",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Sin",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Cos",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Tan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Sinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Cosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Tanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Inv",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Minus",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Sqr",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asin()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_Id",
            Binary {
                f: |a, b| { let left = (a).asin()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSin_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asin()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Exp",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Log",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Sin",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Cos",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Tan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Sinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Cosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Tanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Inv",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Minus",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Sqr",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acos()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_Id",
            Binary {
                f: |a, b| { let left = (a).acos()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCos_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acos()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Exp",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Log",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Sin",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Cos",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Tan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Sinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Cosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Tanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Inv",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Minus",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Sqr",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atan()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_Id",
            Binary {
                f: |a, b| { let left = (a).atan()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTan_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atan()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).sinh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Cosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).cosh().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Exp",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Log",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Sin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Cos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Tan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Inv",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Minus",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).tanh()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_Id",
            Binary {
                f: |a, b| { let left = (a).tanh()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Tanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).tanh()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Exp",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Log",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Sin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Cos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Tan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Sinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Cosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Tanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Inv",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Minus",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Sqr",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).asinh()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_Id",
            Binary {
                f: |a, b| { let left = (a).asinh()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcSinh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).asinh()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Exp",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Log",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Sin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Cos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Tan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Sinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Cosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Tanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Inv",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Minus",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Sqr",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).acosh()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_Id",
            Binary {
                f: |a, b| { let left = (a).acosh()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcCosh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).acosh()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Exp",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Log",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Sin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Cos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Tan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_ArcSin",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_ArcCos",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_ArcTan",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Sinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Cosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Tanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_ArcSinh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_ArcCosh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_ArcTanh",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Inv",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Minus",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Sqr",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Sqrt",
            Binary {
                f: |a, b| { let left = (a).atanh()?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_Id",
            Binary {
                f: |a, b| { let left = (a).atanh()?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_ArcTanh_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = (a).atanh()?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Exp",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Log",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Sin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Cos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Tan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_ArcSin",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_ArcCos",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_ArcTan",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Sinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Cosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Tanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_ArcSinh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_ArcCosh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_ArcTanh",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Inv",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Minus",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Sqr",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Sqrt",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_Id",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Inv_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = C::real(1.0).div(a)?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Minus_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).neg().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqr_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).mul(a).ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_Id",
            Binary {
                f: |a, b| { b.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Sqrt_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div((a).sqrt().ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Exp",
            Binary {
                f: |a, b| { (b).exp().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Log",
            Binary {
                f: |a, b| { let right = (b).ln()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Sin",
            Binary {
                f: |a, b| { (b).sin().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Cos",
            Binary {
                f: |a, b| { (b).cos().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Tan",
            Binary {
                f: |a, b| { let right = (b).tan()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_ArcSin",
            Binary {
                f: |a, b| { let right = (b).asin()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_ArcCos",
            Binary {
                f: |a, b| { let right = (b).acos()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_ArcTan",
            Binary {
                f: |a, b| { let right = (b).atan()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Sinh",
            Binary {
                f: |a, b| { (b).sinh().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Cosh",
            Binary {
                f: |a, b| { (b).cosh().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Tanh",
            Binary {
                f: |a, b| { let right = (b).tanh()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_ArcSinh",
            Binary {
                f: |a, b| { let right = (b).asinh()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_ArcCosh",
            Binary {
                f: |a, b| { let right = (b).acosh()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_ArcTanh",
            Binary {
                f: |a, b| { let right = (b).atanh()?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Inv",
            Binary {
                f: |a, b| { let right = C::real(1.0).div(b)?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Minus",
            Binary {
                f: |a, b| { (b).neg().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Sqr",
            Binary {
                f: |a, b| { (b).mul(b).ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_Sqrt",
            Binary {
                f: |a, b| { (b).sqrt().ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_Id_LogisticSigmoid",
            Binary {
                f: |a, b| { let right = logistic_sigmoid(b)?; right.ln()?.div(a.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Exp",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).exp().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Log",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).ln()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Sin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).sin().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Cos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).cos().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Tan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_ArcSin",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asin()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_ArcCos",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acos()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_ArcTan",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atan()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Sinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).sinh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Cosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).cosh().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Tanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).tanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_ArcSinh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).asinh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_ArcCosh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).acosh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_ArcTanh",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = (b).atanh()?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Inv",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = C::real(1.0).div(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Minus",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).neg().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Sqr",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).mul(b).ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Sqrt",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; (b).sqrt().ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_Id",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; b.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),
        (
            "LogBase_LogisticSigmoid_LogisticSigmoid",
            Binary {
                f: |a, b| { let left = logistic_sigmoid(a)?; let right = logistic_sigmoid(b)?; right.ln()?.div(left.ln()?) },
                commutative: false,
            },
        ),