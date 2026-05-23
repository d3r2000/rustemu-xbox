/// Wine-style Syscall Frame Discipline for R14 (ESP) Protection
///
/// Each guest CALL pushes a frame recording the current R14 (ESP).
/// Each guest RET pops the frame and verifies R14 matches.
/// If R14 has drifted, the frame restores it — self-healing.
///
/// Design references:
///   Wine: dlls/ntdll/unix/signal_i386.c — syscall_frame + prev_frame chain
///   Frida: guminterceptor.c — GumInvocationStackEntry + caller_ret_addr
///
/// Fixed-size array (no heap allocation in hot path).

/// Maximum call nesting depth. 256 handles deep game init chains.
/// If exceeded, frames stop being pushed (no crash, just no protection).
const MAX_FRAMES: usize = 256;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FrameType {
    IndirectCall = 0,  // VEH indirect CALL dispatch
    KernelCall = 1,    // Kernel thunk CALL
    KernelJmp = 2,     // Kernel thunk JMP
    EmitCallRel = 3,   // Direct CALL (emit_call_rel in compiled code)
}

#[derive(Clone, Copy)]
pub struct SyscallFrame {
    /// Guest ESP (R14) at the moment the CALL fired, BEFORE the push.
    /// On RET, ESP should return to this value (for cdecl) or this + ret_cleanup (for stdcall).
    pub saved_r14: u32,
    /// Guest EBP at frame entry (for diagnostics).
    pub saved_ebp: u32,
    /// Expected guest return address (pushed by the CALL).
    pub guest_ret_addr: u32,
    /// What created this frame.
    pub frame_type: FrameType,
}

/// Fixed-size frame stack. No heap allocation in hot path.
pub struct FrameStack {
    frames: [SyscallFrame; MAX_FRAMES],
    top: usize, // index of next free slot (0 = empty)
}

impl FrameStack {
    pub fn new() -> Self {
        Self {
            frames: [SyscallFrame {
                saved_r14: 0,
                saved_ebp: 0,
                guest_ret_addr: 0,
                frame_type: FrameType::IndirectCall,
            }; MAX_FRAMES],
            top: 0,
        }
    }

    /// Push a new frame. Returns false if stack is full (no crash).
    pub fn push(&mut self, frame: SyscallFrame) -> bool {
        if self.top >= MAX_FRAMES {
            return false;
        }
        self.frames[self.top] = frame;
        self.top += 1;
        true
    }

    /// Pop the top frame. Returns None if empty.
    pub fn pop(&mut self) -> Option<SyscallFrame> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        Some(self.frames[self.top])
    }

    /// Peek at the top frame without popping.
    pub fn peek(&self) -> Option<&SyscallFrame> {
        if self.top == 0 {
            None
        } else {
            Some(&self.frames[self.top - 1])
        }
    }

    /// Current nesting depth.
    pub fn depth(&self) -> usize {
        self.top
    }

    /// Clear all frames (used on shadow stack reset / phase change).
    pub fn clear(&mut self) {
        self.top = 0;
    }

    /// SEH unwinding sync: pop all frames where saved_r14 < current_esp.
    /// When an exception unwinds the stack, frames from unwound functions
    /// become "zombies." This removes them so the next RET pops the right frame.
    pub fn sync_to_esp(&mut self, current_esp: u32) {
        while self.top > 0 {
            let frame = &self.frames[self.top - 1];
            // If the saved ESP is BELOW current ESP, this frame was from a
            // function that has been unwound (stack grew down, so lower = deeper).
            if frame.saved_r14 < current_esp {
                self.top -= 1;
            } else {
                break;
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn push_pop_balanced() {
        let mut fs = FrameStack::new();
        assert_eq!(fs.depth(), 0);

        fs.push(SyscallFrame {
            saved_r14: 0x00C1FF00,
            saved_ebp: 0x00C1FF10,
            guest_ret_addr: 0x0029C090,
            frame_type: FrameType::IndirectCall,
        });
        fs.push(SyscallFrame {
            saved_r14: 0x00C1FE00,
            saved_ebp: 0x00C1FE10,
            guest_ret_addr: 0x002B52AD,
            frame_type: FrameType::EmitCallRel,
        });
        assert_eq!(fs.depth(), 2);

        let f2 = fs.pop().unwrap();
        assert_eq!(f2.saved_r14, 0x00C1FE00);
        assert_eq!(f2.guest_ret_addr, 0x002B52AD);

        let f1 = fs.pop().unwrap();
        assert_eq!(f1.saved_r14, 0x00C1FF00);
        assert_eq!(f1.guest_ret_addr, 0x0029C090);

        assert!(fs.pop().is_none());
        assert_eq!(fs.depth(), 0);
    }

    #[test]
    fn depth_cap_no_panic() {
        let mut fs = FrameStack::new();
        for i in 0..300 {
            let ok = fs.push(SyscallFrame {
                saved_r14: i,
                saved_ebp: 0,
                guest_ret_addr: 0,
                frame_type: FrameType::IndirectCall,
            });
            if i < MAX_FRAMES as u32 {
                assert!(ok, "push should succeed for i={}", i);
            } else {
                assert!(!ok, "push should fail for i={}", i);
            }
        }
        assert_eq!(fs.depth(), MAX_FRAMES);
    }

    #[test]
    fn seh_sync_removes_zombie_frames() {
        let mut fs = FrameStack::new();
        // Outer function at ESP=0x00C1FF00
        fs.push(SyscallFrame {
            saved_r14: 0x00C1FF00,
            saved_ebp: 0,
            guest_ret_addr: 0x1000,
            frame_type: FrameType::IndirectCall,
        });
        // Inner function at ESP=0x00C1FE00 (deeper = lower)
        fs.push(SyscallFrame {
            saved_r14: 0x00C1FE00,
            saved_ebp: 0,
            guest_ret_addr: 0x2000,
            frame_type: FrameType::EmitCallRel,
        });
        // Deepest function at ESP=0x00C1FD00
        fs.push(SyscallFrame {
            saved_r14: 0x00C1FD00,
            saved_ebp: 0,
            guest_ret_addr: 0x3000,
            frame_type: FrameType::KernelCall,
        });
        assert_eq!(fs.depth(), 3);

        // Exception unwinds to ESP=0x00C1FE80 (between inner and outer)
        // Frames with saved_r14 < 0x00C1FE80 are zombies
        fs.sync_to_esp(0x00C1FE80);

        // Only the outer frame (0x00C1FF00 >= 0x00C1FE80) survives
        assert_eq!(fs.depth(), 1);
        let f = fs.peek().unwrap();
        assert_eq!(f.saved_r14, 0x00C1FF00);
    }

    #[test]
    fn drift_detection_scenario() {
        // Simulate: callback pushes frame at ESP=0x00C1FF00
        // sprintf runs, returns, but ESP is now 0x00C1FEBC (drifted by 0x44)
        let mut fs = FrameStack::new();
        fs.push(SyscallFrame {
            saved_r14: 0x00C1FF00,
            saved_ebp: 0x00C1FF10,
            guest_ret_addr: 0x0029C090,
            frame_type: FrameType::IndirectCall,
        });

        let frame = fs.pop().unwrap();
        let actual_r14: u32 = 0x00C1FEBC; // drifted!
        let expected = frame.saved_r14;    // 0x00C1FF00
        let drift = (actual_r14 as i64 - expected as i64).unsigned_abs();

        assert_eq!(drift, 0x44, "drift should be exactly 0x44 (the missing add esp cleanup)");
        // In production: restore R14 to `expected` to self-heal
    }
}
