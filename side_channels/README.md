# leak check experiment

Simulate "secret data"

```
Secret128bit { secret: [u8; 16] }
```

This would leak timing as it decides code execution flow based on secret data in for loop:

```
check_eq_vartime(&[u8; 16])
```

Would MIR let allow generalised timing leak check ?

$ cargo rustc -- -Z unpretty=mir

```
Secret128bit::secret::{constant#0}: usize = {
    let mut _0: usize;

    bb0: {
        _0 = const 16_usize;
        return;
    }
}

fn <impl at src/lib.rs:17:1: 17:18>::check_eq_vartime(_1: &Secret128bit, _2: &[u8; 16]) -> bool {
    debug self => _1;
    debug input => _2;
    let mut _0: bool;
    let mut _3: std::ops::RangeInclusive<usize>;
    let mut _4: std::ops::RangeInclusive<usize>;
    let mut _5: std::ops::RangeInclusive<usize>;
    let mut _6: std::option::Option<usize>;
    let mut _7: &mut std::ops::RangeInclusive<usize>;
    let mut _8: isize;
    let mut _10: bool;
    let mut _11: u8;
    let mut _12: usize;
    let mut _13: bool;
    let mut _14: u8;
    let mut _15: usize;
    let mut _16: bool;
    scope 1 {
        debug iter => _5;
        let _9: usize;
        scope 2 {
            debug i => _9;
        }
    }

    bb0: {
        _4 = RangeInclusive::<usize>::new(const 0_usize, const 15_usize) -> [return: bb1, unwind continue];
    }

    bb1: {
        _3 = <RangeInclusive<usize> as IntoIterator>::into_iter(move _4) -> [return: bb2, unwind continue];
    }

    bb2: {
        _5 = move _3;
        goto -> bb3;
    }

    bb3: {
        _7 = &mut _5;
        _6 = <RangeInclusive<usize> as Iterator>::next(_7) -> [return: bb4, unwind continue];
    }

    bb4: {
        _8 = discriminant(_6);
        switchInt(move _8) -> [0: bb7, 1: bb6, otherwise: bb5];
    }

    bb5: {
        unreachable;
    }

    bb6: {
        _9 = ((_6 as Some).0: usize);
        _12 = const 16_usize;
        _13 = Lt(_9, _12);
        assert(move _13, "index out of bounds: the length is {} but the index is {}", move _12, _9) -> [success: bb8, unwind continue];
    }

    bb7: {
        _0 = const true;
        goto -> bb11;
    }

    bb8: {
        _11 = (*_2)[_9];
        _15 = const 16_usize;
        _16 = Lt(_9, _15);
        assert(move _16, "index out of bounds: the length is {} but the index is {}", move _15, _9) -> [success: bb9, unwind continue];
    }

    bb9: {
        _14 = ((*_1).0: [u8; 16])[_9];
        _10 = Ne(move _11, move _14);
        switchInt(move _10) -> [0: bb3, otherwise: bb10];
    }

    bb10: {
        _0 = const false;
        goto -> bb11;
    }

    bb11: {
        return;
    }
}

<impl at src/lib.rs:17:1: 17:18>::check_eq_vartime::{constant#0}: usize = {
    let mut _0: usize;

    bb0: {
        _0 = const 16_usize;
        return;
    }
}
```


$ cargo asm
```
.section .text.side_channels::Secret128bit::check_eq_vartime,"ax",@progbits
	.globl	side_channels::Secret128bit::check_eq_vartime
	.p2align	4, 0x90
	.type	side_channels::Secret128bit::check_eq_vartime,@function
side_channels::Secret128bit::check_eq_vartime:
	.cfi_startproc
	movzx eax, byte ptr [rsi]
	cmp al, byte ptr [rdi]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 1]
	cmp al, byte ptr [rdi + 1]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 2]
	cmp al, byte ptr [rdi + 2]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 3]
	cmp al, byte ptr [rdi + 3]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 4]
	cmp al, byte ptr [rdi + 4]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 5]
	cmp al, byte ptr [rdi + 5]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 6]
	cmp al, byte ptr [rdi + 6]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 7]
	cmp al, byte ptr [rdi + 7]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 8]
	cmp al, byte ptr [rdi + 8]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 9]
	cmp al, byte ptr [rdi + 9]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 10]
	cmp al, byte ptr [rdi + 10]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 11]
	cmp al, byte ptr [rdi + 11]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 12]
	cmp al, byte ptr [rdi + 12]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 13]
	cmp al, byte ptr [rdi + 13]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 14]
	cmp al, byte ptr [rdi + 14]
	jne .LBB0_16
	movzx eax, byte ptr [rsi + 15]
	cmp al, byte ptr [rdi + 15]
	sete al
	neg al
	and al, 1
	ret
.LBB0_16:
	xor eax, eax
	and al, 1
	ret
```
