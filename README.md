# 🔮 imcc (Ultimate Multi-Language Compiler & Interpreter Router)

ระบบจัดการและรันภาษาโปรแกรมมิ่งแบบ All-in-One ประสิทธิภาพสูง พัฒนาด้วยภาษา Rust ถูกออกแบบมาเพื่อคัดแยกสายการทำงานอย่างเป็นระเบียบและแม่นยำสูงสุด ด้วยสถาปัตยกรรมระดับ Quantum ที่แยกฝั่ง "คอมไพล์รัน" และ "ปล่อยรันสด" ออกจากกันอย่างชัดเจน

---

## 🛠️ โครงสร้างการทำงานภายใน (Core Architecture)

ระบบทำการตรวจจับนามสกุลไฟล์โดยอัตโนมัติ และสลับโหมดการทำงานผ่านระบบ Pattern Matching ที่ปลอดภัยและไม่มีวันทำงานตีกัน:

### 📦 1. ฝั่งต้องคอมไพล์ (Compile to Binary/Bytecode)
- **C** (`.c`)
- **C++** (`.cpp`, `.cc`, `.cxx`)
- **Go** (`.go`)
- **Rust** (`.rs`)
- **Java** (`.java`)
- **C#** (`.cs`)
- **Swift** (`.swift`)
- **Kotlin** (`.kt`, `.kts`)

### ⚡ 2. ฝั่งปล่อยรันสด (Interpreter / Scripting)
- **Python** (`.py`, `.pyw`)
- **JavaScript** (`.js`, `.mjs`)
- **TypeScript** (`.ts`)
- **Lua** (`.lua`)
- **Ruby** (`.rb`)
- **PHP** (`.php`)

---

## 🚀 การติดตั้งและใช้งาน (Installation & Usage)

### การติดตั้งแบบรวดเร็วอัตโนมัติ
```bash
./install.sh
